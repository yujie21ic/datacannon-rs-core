//! Implementation of available brokers in a non-asynchronous manner.
//!
//! ---
//! author: Andrew Evans
//! ---


use std::collections::HashMap;

use amiquip::{Channel, ExchangeDeclareOptions, ExchangeType, FieldTable, Publish, QueueDeclareOptions, QueueDeleteOptions};
use serde_json::{to_string, Value};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc;

use crate::broker::amqp::broker_trait::AMQPBroker;
use crate::broker::broker::{Broker, BrokerEvent, CommunicationEvent};
use crate::config::config::CannonConfig;
use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::connection::connection::ConnectionConfig;
use crate::error::broker_type_error::BrokerTypeError;
use crate::error::exchange_error::ExchangeError;
use crate::error::publish_error::PublishError;
use crate::error::queue_error::QueueError;
use crate::message_protocol::headers::Headers;
use crate::message_protocol::message::Message;
use crate::message_protocol::message_body::MessageBody;
use crate::message_protocol::properties::Properties;
use crate::router::router::Router;
use crate::statistics::message::Statistics;
use crate::task::config::TaskConfig;


/// RabbitMQ Broker
///
/// # Arguments
/// * `config` - Canon configuration
/// * `pool` - RabbitMQ connectionpool
/// * `broker_futures` - Active futures for monitoring
/// * `event_senders` - For sending events to futures
/// * `comm_receivers` - Receivers for communications from futures
/// * `num_failures` - Number of failures
pub struct RabbitMQBroker{
    config: CannonConfig,
    pool: ThreadableRabbitMQConnectionPool,
    event_senders: Vec<UnboundedSender<BrokerEvent>>,
    comm_receivers: Vec<UnboundedReceiver<CommunicationEvent>>,
    num_failure: i32,
    current_future: i8,
}


/// AMQP Broker
impl AMQPBroker for RabbitMQBroker{

    /// Create the exchange
    ///
    /// # Arguments
    /// * `config` - The reference to the application `crate::config::config::CannonConfig`
    /// * `channel` - Reference to a `amiquip::Channel`
    /// * `durabe` - Whether the exchange persists
    /// * `exchange` - Name of the exchange
    /// * `exchange_type` - The exchange type
    fn create_exchange(config: &CannonConfig, channel: &Channel, durable: bool, exchange: String, exchange_type: ExchangeType) -> Result<bool, ExchangeError> {
        let mut opts = ExchangeDeclareOptions::default();
        opts.durable = durable;
        let r = channel.exchange_declare(exchange_type, exchange, opts);
        if r.is_ok(){
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// Create a queue
    ///
    /// # Arguments
    /// * `config` - Reference to the application `crate::config::config::CannonConfig`
    /// * `channel` - A reference to a `amiquip::Channel`
    /// * `durable` - Whether the channel should persist
    /// * `queue` - The name of the queue
    /// * `declare_exchange` - Whether to declare the exchange
    /// * `uuid` - Unique id for the message e
    /// * `exchange` - Name of the exchange
    /// * `routing_key` - Routing key for the exchange
    fn create_queue(config: &CannonConfig, channel: &Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, QueueError>{
        let mut qopts = QueueDeclareOptions::default();
        if declare_exchange{
            let mut etype = ExchangeType::Direct;
            let mut eopts= ExchangeDeclareOptions::default();
            eopts.durable = durable;
            channel.exchange_declare(etype, exchange.clone().unwrap(), eopts);
        }
        if durable {
            qopts.durable = durable;
        }
        let r = channel.queue_declare(queue.clone(), qopts);
        if r.is_ok(){
            //bind queue to exchange
            if exchange.is_some(){
                let exchange_name = exchange.unwrap();
                let args = FieldTable::new();
                let mut m_routing_key = config.default_routing_key.clone();
                if routing_key.is_some(){
                    m_routing_key = routing_key.unwrap();
                }
                let er = channel.queue_bind(queue, exchange_name, m_routing_key, args);
                if er.is_ok(){
                    Ok(true)
                }else{
                    Err(QueueError)
                }
            }else {
                Ok(true)
            }
        }else{
            Err(QueueError)
        }
    }

    /// Bind a queue to an exchange
    ///
    /// # Arguments
    /// * `config` - The application `crate::config::config::CannonConfig`
    /// * `channel` - A `amiquip::Channel`
    /// * `exchange` - Name of the exchange
    /// * `queue` - Name of the queue to bind
    /// * `routing_key` - Name of the routing key
    fn bind_to_exchange(config: &CannonConfig, channel: &Channel, exchange: String, queue: String, routing_key: String) -> Result<bool, ExchangeError> {
        let args = FieldTable::new();
        let r = channel.queue_bind(queue, exchange, routing_key, args);
        if r.is_ok(){
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// Send a task to the broker
    ///
    /// # Arguments
    /// * `config` - The application `crate::config::config::CannonConfig`
    /// * `channel` - A `amiquip::Channel`
    /// * `props` - The `amiquip::Properties` for the message
    /// * `headers` - Relevant message `crate::message_protocol::headers::Headers`
    /// * `body` - Relevant message `crate::message_protocol::body::MessageBody`\
    /// * `exchange` - Name of the exchange to use
    /// * `routing_key` - Name of the routing key
    fn do_send(config: &CannonConfig, channel: &Channel, props: Properties, headers: Headers, body: MessageBody, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, PublishError> {
        let cfg = config.clone();
        let mut amq_properties = props.convert_to_amqp_properties();
        let amq_headers = headers.convert_to_btree_map();
        let json_val = Value::from(body.convert_to_json_map());
        let mut json_message = to_string(&json_val);
        if json_message.is_ok() {
            let mut m_routing_key = cfg.default_routing_key.clone();
            let mut m_exchange = cfg.default_routing_key;
            if exchange.is_some(){
                m_exchange = exchange.unwrap();
            }
            if routing_key.is_some(){
                m_routing_key = routing_key.unwrap();
            }
            amq_properties = amq_properties.with_headers(amq_headers);
            let jmessage = json_message.unwrap();
            let jbytes = jmessage.as_bytes();
            let mut publish = Publish::with_properties(jbytes, m_routing_key, amq_properties);
            channel.basic_publish(m_exchange, publish);
            Ok(true)
        }else{
            let e = PublishError;
            Err(e)
        }
    }

    /// Drop the queue
    ///
    /// # Arguments
    /// * `config` - Reference to the application `crate::config::config::CannonConfig`
    /// * `channel` - Reference to an `amiquip::Channel`
    /// * `name` - Name of the string
    fn do_drop_queue(config: &CannonConfig, channel: &Channel, queue: String) -> Result<bool, QueueError> {
        let qopts = QueueDeleteOptions::default();
        let r = channel.queue_delete(queue, qopts);
        if r.is_ok() {
            Ok(true)
        }else{
            Err(QueueError)
        }
    }
}


/// create a rabbitmq broker future
async fn send_task_future(channel: Channel, cannon_config: CannonConfig, mut sender: UnboundedSender<CommunicationEvent>, receiver: &mut UnboundedReceiver<BrokerEvent>) -> Result<(), &'static str> {
    let mut msg_received = 0;
    let mut msg_sent = 0;
    loop{
        let m = receiver.recv().await.unwrap();
        msg_received += 1;
        if let BrokerEvent::SEND(m) = m {
            let props = m.message.properties.clone();
            let headers = m.message.headers.clone();
            let body = m.message.body.clone();
            let kwargs = m.message.kwargs.clone();
            let exchange = m.exchange;
            let routing_key = m.routing_key;
            RabbitMQBroker::do_send(&cannon_config, &channel, props, headers, body, exchange, routing_key);
            msg_sent += 1;
        }else if let BrokerEvent::GET_STATS = m{
            let stats_message = Statistics{
                messages_received: msg_received,
                messages_sent: msg_sent,
            };
            sender.try_send(CommunicationEvent::STATISTICS(stats_message));
        }else if let BrokerEvent::POISON_PILL = m{
            receiver.close();
            break;
        }
    }
    Ok(())
}

/// Broker implementation
impl Broker for RabbitMQBroker{

    /// Restart a future when another future completes (called when another terminates)
    ///
    /// # Arguments
    /// * `runtime` - Tokio runtime to create the future on
    fn create_fut(&mut self, runtime: &Runtime) {
        let ch = self.pool.get_connection().unwrap().connection.open_channel(None).unwrap();
        let (mut send, mut rcv): (UnboundedSender<BrokerEvent>, UnboundedReceiver<BrokerEvent>) = mpsc::unbounded_channel();
        let (mut chsend, mut chrcv): (UnboundedSender<CommunicationEvent>, UnboundedReceiver<CommunicationEvent>) = mpsc::unbounded_channel();
        let future_config = self.config.clone();
        let future_sender = chsend.clone();
        runtime.spawn(async move{
           send_task_future(ch, future_config, future_sender, &mut rcv).await;
        });
        self.comm_receivers.push(chrcv);
        self.event_senders.push(send);
    }

    /// Start the broker futures
    fn setup(&mut self, runtime: &Runtime){
        for i in 0..self.config.num_broker_connections{
            self.create_fut(runtime);
        }
    }

    /// Drop the future
    ///
    /// # Arguments
    /// * `idx` - Index of the future to drop
    fn drop_future(&mut self, idx: usize) {
        // drop receiver and sender
        let mut  r = self.comm_receivers.get(i).unwrap();
        r.close();
        self.comm_receivers.remove(idx);
        self.event_senders.remove(idx);

        // replace future and add new sender and receiver
        self.create_fut(runtime)
    }

    /// Teardown the broker
    fn teardown(&mut self){
        //teardown the futures
        for i in 0..self.event_senders.len(){
            let s = self.event_senders.get(i).unwrap();
            let m = BrokerEvent::POISON_PILL;
            s.clone().try_send(m);
        }
        // close the threadpool
        self.close();
    }

    /// Close all connections and thus the broker. shut down futures first
    fn close(&mut self){
        self.pool.close_pool();
    }

    /// Send a task
    ///
    /// # Arguments
    /// * `runtime` - Tokio runtime to send task on
    /// * `task` - The task config
    fn send_task(&mut self, runtime: &Runtime, task: TaskConfig, message_body: Option<MessageBody>) {

    }

    /// Allows workers to subscribe to the broker which may also invoek send_task for chains and chords
    ///
    /// # Arguments
    /// * `runtime` - Runtime for the application
    /// * `config` - The Cannon Configuration for the application
    fn subscribe_to_queues(&mut self, runtime: &Runtime, config: &CanonConfig){

    }
}


/// Rabbit MQ broker
impl RabbitMQBroker{

    /// get a channel from the pool
    pub fn get_channel(&mut self) -> Channel{
        let conn_result = self.pool.get_connection();
        let mut conn = conn_result.unwrap();
        let ch = conn.connection.open_channel(None);
        self.pool.add_connection();
        ch.unwrap()
    }

    /// Create a new broker
    pub fn new(config: &mut CannonConfig, routers: Option<HashMap<String, Router>>, min_connections: Option<usize>, num_futures: usize) -> Result<RabbitMQBroker, BrokerTypeError>{
        let mut min_conn = num_cpus::get() - 1;
        if min_connections.is_some(){
            min_conn = min_connections.unwrap();
        }
        let cfg = config.clone().connection_inf;
        if let ConnectionConfig::RabbitMQ(cfg) = cfg {
            let pool = ThreadableRabbitMQConnectionPool::new(&mut cfg.clone(), min_conn);
            let rmb = RabbitMQBroker {
                config: config.clone(),
                pool: pool,
                event_senders: vec![],
                comm_receivers: vec![],
                num_failure: 0,
                current_future: 0,
            };
            Ok(rmb)
        }else{
            Err(BrokerTypeError)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::thread;
    use std::thread::JoinHandle;

    use amq_protocol::frame::AMQPFrameType::Header;
    use tokio::prelude::*;
    use tokio::runtime::Runtime;
    use uuid::Uuid;

    use crate::backend::config::BackendConfig;
    use crate::broker::amqp::broker_trait::AMQPBroker;
    use crate::broker::amqp::rabbitmq::RabbitMQBroker;
    use crate::config::config::CannonConfig;
    use crate::connection::amqp::connection_inf::AMQPConnectionInf;
    use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
    use crate::connection::connection::ConnectionConfig;
    use crate::connection::kafka::connection_inf::KafkaConnectionInf;
    use crate::router::router::Routers;
    use crate::security::ssl::SSLConfig;
    use crate::security::uaa::UAAConfig;

    use super::*;

    fn get_config(ssl_config: Option<SSLConfig>, uaa_config: Option<UAAConfig>) -> CannonConfig {
        let protocol = "amqp".to_string();
        let host = "127.0.0.1".to_string();
        let port = 5672;
        let vhost = Some("test".to_string());
        let username = Some("dev".to_string());
        let password = Some("rtp*4500".to_string());
        let broker_conn = AMQPConnectionInf::new(protocol, host, port, vhost, username, password, false, ssl_config, uaa_config);
        let backend = BackendConfig{
            url: "rpc://".to_string(),
            username: None,
            password: None,
            transport_options: None,
        };
        let kinf = KafkaConnectionInf{
            ack_timeout: 0,
            num_acks: 0,
            host: "".to_string(),
            port: "".to_string(),
        };
        let rs = Routers::new();
        let conf = CannonConfig::new(ConnectionConfig::RabbitMQ(broker_conn), backend, rs);
        conf
    }

    #[test]
    fn should_create_queue(){
        let mut conf = get_config(None, None);
        let rmq = RabbitMQBroker::new(&mut conf, None,  Some(1),  1);
        let mut conn_inf = conf.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 2);
            pool.start();
            let rconn = pool.get_connection();
            if rconn.is_ok() {
                let mut c = rconn.unwrap();
                let channel = c.connection.open_channel(None).unwrap();
                let uuid = format!("{}", Uuid::new_v4());
                let rq = RabbitMQBroker::create_queue(&conf, &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
                c.connection.close();
                assert!(rq.is_ok());
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn should_create_and_bind_queue_to_exchange(){
        let conf = get_config(None, None);
        let rmq = RabbitMQBroker::new(&mut conf.clone(), None, Some(1),  1);
        let conn_inf = conf.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 2);
            pool.start();
            let rconn = pool.get_connection();
            if rconn.is_ok() {
                let mut c = rconn.unwrap();
                let channel = c.connection.open_channel(None).unwrap();
                let uuid = format!("{}", Uuid::new_v4());
                let rq = RabbitMQBroker::create_queue(&conf, &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
                RabbitMQBroker::bind_to_exchange(&conf, &channel, "test_exchange".to_string(), "test_queue".to_string(), "test_routing_key".to_string());
                c.connection.close();
                assert!(rq.is_ok());
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn should_send_task_to_queue(){
        let conf = get_config(None, None);
        let rmq = RabbitMQBroker::new(&mut conf.clone(), None,  Some(1), 1);
        let conn_inf = conf.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 2);
            pool.start();
            let rconn = pool.get_connection();
            if rconn.is_ok() {
                let mut c = rconn.unwrap();
                let channel = c.connection.open_channel(None).unwrap();
                let uuid = format!("{}", Uuid::new_v4());

                // create queue if necessary
                let rq = RabbitMQBroker::create_queue(&conf, &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));

                // create and send task
                let body = MessageBody::new(None, None, None, None);
                let uuid = Uuid::new_v4();
                let ustr = format!("{}", uuid);
                let headers = Headers::new("rs".to_string(), "test_task".to_string(), ustr.clone(), ustr.clone());
                let reply_queue = Uuid::new_v4();
                let props = Properties::new(ustr.clone(), "application/json".to_string(), "utf-8".to_string(), None);
                let br = RabbitMQBroker::do_send(&conf, &channel, props, headers, body, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
                c.connection.close();
                assert!(br.is_ok());
                assert!(rq.is_ok());
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn should_work_with_threads(){
        let cnf = get_config(None, None);
        let rmq = RabbitMQBroker::new(&mut cnf.clone(), None, Some(1),1);
        let conn_inf = cnf.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 2);
            pool.start();
            let rconn = pool.get_connection();
            if rconn.is_ok() {
                let mut c = rconn.unwrap();
                let channel = c.connection.open_channel(None).unwrap();
                let mut conf = cnf.clone();
                let ja = thread::spawn(move || {
                    for i in 0..2666 {
                        let uuid = format!("{}", Uuid::new_v4());
                        // create queue if necessary

                        let rq = RabbitMQBroker::create_queue(&conf, &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));

                        // create and send task
                        let body = MessageBody::new(None, None, None, None);
                        let uuid = Uuid::new_v4();
                        let ustr = format!("{}", uuid);
                        let headers = Headers::new("rs".to_string(), "test_task".to_string(), ustr.clone(), ustr.clone());
                        let reply_queue = Uuid::new_v4();
                        let props = Properties::new(ustr.clone(), "application/json".to_string(), "utf-8".to_string(), None);
                        let br = RabbitMQBroker::do_send(&conf, &channel, props, headers, body, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
                    }
                });

                conf = cnf.clone();
                let channelb = c.connection.open_channel(None).unwrap();
                let jb = thread::spawn(move || {
                    for i in 0..2666 {
                        let uuid = format!("{}", Uuid::new_v4());
                        // create queue if necessary

                        let rq = RabbitMQBroker::create_queue(&conf, &channelb, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));

                        // create and send task
                        let body = MessageBody::new(None, None, None, None);
                        let uuid = Uuid::new_v4();
                        let ustr = format!("{}", uuid);
                        let headers = Headers::new("rs".to_string(), "test_task".to_string(), ustr.clone(), ustr.clone());
                        let reply_queue = Uuid::new_v4();
                        let props = Properties::new(ustr.clone(), "application/json".to_string(), "utf-8".to_string(), None);
                        let br = RabbitMQBroker::do_send(&conf, &channelb, props, headers, body, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
                    }
                });

                conf = cnf.clone();
                let channelc = c.connection.open_channel(None).unwrap();
                let jc = thread::spawn(move || {
                    for i in 0..2666 {
                        let uuid = format!("{}", Uuid::new_v4());
                        // create queue if necessary

                        let rq = RabbitMQBroker::create_queue(&conf, &channelc, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));

                        // create and send task
                        let body = MessageBody::new(None, None, None, None);
                        let uuid = Uuid::new_v4();
                        let ustr = format!("{}", uuid);
                        let headers = Headers::new("rs".to_string(), "test_task".to_string(), ustr.clone(), ustr.clone());
                        let reply_queue = Uuid::new_v4();
                        let props = Properties::new(ustr.clone(), "application/json".to_string(), "utf-8".to_string(), None);
                        let br = RabbitMQBroker::do_send(&conf, &channelc, props, headers, body, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
                    }
                });

                ja.join();
                jb.join();
                jc.join();
                c.connection.server_properties();
            }
        }
    }
}
