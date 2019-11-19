//! Implementation of available brokers in a non-asynchronous manner.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap};
use std::env::Args;
use std::error::Error;

use amiquip::{AmqpProperties, AmqpValue, Channel, Exchange, ExchangeDeclareOptions, ExchangeType, FieldTable, Publish, Queue, QueueDeclareOptions, QueueDeleteOptions};
use serde_json::{to_string, Value};
use serde_json::map::Values;
use tokio;

use crate::argparse::argtype::ArgType;
use crate::argparse::kwargs::KwArgs;
use crate::broker::amqp::broker_trait::AMQPBroker;
use crate::broker::broker::Broker;
use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::connection::amqp::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::connection::pool::Pool;
use crate::error::{exchange_error::ExchangeError, publish_error::PublishError, queue_error::QueueError};
use crate::message_protocol::{headers::Headers, message::Message, message_body::MessageBody, properties::Properties};
use crate::router::router::Router;
use crate::task::config::TaskConfig;
use std::future::Future;
use tokio::sync::mpsc::{Sender, Receiver};
use crate::config::config::CannonConfig;
use crate::connection::connection::ConnectionConfig;
use crate::error::broker_type_error::BrokerTypeError;
use uuid::Uuid;


/// RabbitMQ Broker
pub struct RabbitMQBroker{
    config: CannonConfig,
    num_futures: usize,
    pool: ThreadableRabbitMQConnectionPool,
    app_receiver: Receiver<Message>,
    backend_sender: Option<Sender<Message>>,
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


/// Broker implementation
impl Broker for RabbitMQBroker{

    /// Start the broker futures
    fn setup(&mut self, rt: tokio::runtime::Runtime){
        for i in 0..self.num_futures{
            
        }
    }

    /// teardown the broker
    fn teardown(&mut self){

    }

    /// close all connections and thus the broker. shut down futures first
    fn close(&mut self){
        self.pool.close_pool();
    }

    /// send a task
    fn send_task(&mut self, task: String, args: Vec<ArgType>,  app_sender: Sender<Message>) {

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
    pub fn new(config: &mut CannonConfig, routers: Option<HashMap<String, Router>>, min_connections: Option<usize>, sender: Option<Sender<Message>>, receiver: Receiver<Message>, num_futures: usize) -> Result<RabbitMQBroker, BrokerTypeError>{
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
                num_futures: num_futures,
                app_receiver: receiver,
                backend_sender: sender,
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
    use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
    use crate::security::ssl::SSLConfig;
    use crate::security::uaa::UAAConfig;

    use super::*;
    use crate::connection::amqp::connection_inf::AMQPConnectionInf;
    use crate::connection::kafka::connection_inf::KafkaConnectionInf;
    use crate::connection::connection::ConnectionConfig;
    use crate::router::router::Routers;


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
        let (s,r) = tokio::sync::mpsc::channel(1024);
        let rmq = RabbitMQBroker::new(&mut conf, None,  Some(1), Some(s), r, 1);
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
        let (s,r) = tokio::sync::mpsc::channel(1024);
        let rmq = RabbitMQBroker::new(&mut conf.clone(), None, Some(1), Some(s), r, 1);
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
        let (s,r) = tokio::sync::mpsc::channel(1024);
        let rmq = RabbitMQBroker::new(&mut conf.clone(), None,  Some(1), Some(s), r,1);
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
        let (s,r) = tokio::sync::mpsc::channel(1024);
        let rmq = RabbitMQBroker::new(&mut cnf.clone(), None, Some(1), Some(s), r,1);
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
