/*
Implementation of available brokers in a non-asynchronous manner.

Author Andrew Evans
*/

use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap};
use std::env::Args;
use std::error::Error;

use amiquip::{AmqpProperties, AmqpValue, Channel, Exchange, ExchangeDeclareOptions, ExchangeType, FieldTable, Publish, Queue, QueueDeclareOptions};
use serde_json::{to_string, Value};
use serde_json::map::Values;
use tokio;

use crate::argparse::argtype::ArgType;
use crate::argparse::kwargs::KwArgs;
use crate::broker::amqp::broker_trait::AMQPBroker;
use crate::broker::broker::Broker;
use crate::broker::queues::Queues;
use crate::config::config::CeleryConfig;
use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::connection::amqp::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::connection::pool::Pool;
use crate::error::{exchange_error::ExchangeError, publish_error::PublishError, queue_error::QueueError};
use crate::message_protocol::{headers::Headers, message::Message, message_body::MessageBody, properties::Properties};
use crate::protocol_configs::amqp::AMQPConnectionInf;
use crate::router::router::Router;
use crate::task::config::TaskConfig;
use std::future::Future;
use tokio::sync::mpsc::{Sender, Receiver};


/// RabbitMQ Broker
pub struct RabbitMQBroker{
    config: CeleryConfig,
    routers: Option<HashMap<String, Router>>,
    queues: Option<Queues>,
    num_futures: usize,
    pool: ThreadableRabbitMQConnectionPool,
    app_receiver: Receiver<Message>,
    backend_sender: Option<Sender<Message>>,
}


/// AMQP Broker
impl AMQPBroker for RabbitMQBroker{

    /// create the exchange
    fn create_exchange(config: &CeleryConfig, channel: &Channel, durable: bool, exchange: String, exchange_type: ExchangeType) -> Result<bool, ExchangeError> {
        let mut opts = ExchangeDeclareOptions::default();
        opts.durable = durable;
        let r = channel.exchange_declare(exchange_type, exchange, opts);
        if r.is_ok(){
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// create a queue
    fn create_queue(config: &CeleryConfig, channel: &Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, QueueError>{
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

    /// bind a queue to an exchange
    fn bind_to_exchange(config: &CeleryConfig, channel: &Channel, exchange: String, queue: String, routing_key: String) -> Result<bool, ExchangeError> {
        let args = FieldTable::new();
        let r = channel.queue_bind(queue, exchange, routing_key, args);
        if r.is_ok(){
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// send a task to the broker
    fn do_send(config: &CeleryConfig, channel: &Channel, props: Properties, headers: Headers, body: MessageBody, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, PublishError> {
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
}


/// Broker implementation
impl Broker for RabbitMQBroker{

    /// start the broker futures
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
    pub fn new(config: CeleryConfig, queues: Option<Queues>, routers: Option<HashMap<String, Router>>, min_connections: Option<usize>, sender: Option<Sender<Message>>, receiver: Receiver<Message>, num_futures: usize) -> RabbitMQBroker{
        let mut min_conn = num_cpus::get() - 1;
        if min_connections.is_some(){
            min_conn = min_connections.unwrap();
        }
        let pool = ThreadableRabbitMQConnectionPool::new(config.connection_inf.clone(), min_conn);
        RabbitMQBroker{
            config: config.clone(),
            queues: queues,
            routers: routers,
            pool: pool,
            num_futures: num_futures,
            app_receiver: receiver,
            backend_sender: sender,
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
    use crate::config::config::CeleryConfig;
    use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
    use crate::protocol_configs::amqp::AMQPConnectionInf;
    use crate::security::ssl::SSLConfig;
    use crate::security::uaa::UAAConfig;

    use super::*;


    fn get_config(ssl_config: Option<SSLConfig>, uaa_config: Option<UAAConfig>) -> CeleryConfig {
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
        let conf = CeleryConfig::new(broker_conn, backend);
        conf
    }

    #[test]
    fn should_work_with_an_application(){

    }

    #[test]
    fn should_create_queue(){
        let conf = get_config(None, None);
        let (s,r) = tokio::sync::mpsc::channel(1024);
        let rmq = RabbitMQBroker::new(conf.clone(), None, None, Some(1), Some(s), r, 1);
        let conn_inf = conf.connection_inf.clone();
        let mut pool = ThreadableRabbitMQConnectionPool::new(conn_inf, 2);
        pool.start();
        let rconn = pool.get_connection();
        if rconn.is_ok(){
            let mut c = rconn.unwrap();
            let channel = c.connection.open_channel(None).unwrap();
            let uuid = format!("{}", Uuid::new_v4());
            let rq = RabbitMQBroker::create_queue(&conf, &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
            c.connection.close();
            assert!(rq.is_ok());
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_create_and_bind_queue_to_exchange(){
        let conf = get_config(None, None);
        let (s,r) = tokio::sync::mpsc::channel(1024);
        let rmq = RabbitMQBroker::new(conf.clone(), None, None, Some(1), Some(s), r, 1);
        let conn_inf = conf.connection_inf.clone();
        let mut pool = ThreadableRabbitMQConnectionPool::new(conn_inf, 2);
        pool.start();
        let rconn = pool.get_connection();
        if rconn.is_ok(){
            let mut c = rconn.unwrap();
            let channel = c.connection.open_channel(None).unwrap();
            let uuid = format!("{}", Uuid::new_v4());
            let rq = RabbitMQBroker::create_queue(&conf, &channel, true, String::from("test_queue"), true, uuid, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
            RabbitMQBroker::bind_to_exchange(&conf, &channel,  "test_exchange".to_string(), "test_queue".to_string(), "test_routing_key".to_string());
            c.connection.close();
            assert!(rq.is_ok());
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_send_task_to_queue(){
        let conf = get_config(None, None);
        let (s,r) = tokio::sync::mpsc::channel(1024);
        let rmq = RabbitMQBroker::new(conf.clone(), None, None, Some(1), Some(s), r,1);
        let conn_inf = conf.connection_inf.clone();
        let mut pool = ThreadableRabbitMQConnectionPool::new(conn_inf, 2);
        pool.start();
        let rconn = pool.get_connection();
        if rconn.is_ok(){
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
            let br = RabbitMQBroker::do_send(&conf, &channel,props, headers, body, Some("test_exchange".to_string()), Some("test_routing_key".to_string()));
            c.connection.close();
            assert!(br.is_ok());
            assert!(rq.is_ok());
        }else{
            assert!(false);
        }
    }

    #[test]
    fn should_work_with_threads(){
        let cnf = get_config(None, None);
        let (s,r) = tokio::sync::mpsc::channel(1024);
        let rmq = RabbitMQBroker::new(cnf.clone(), None, None, Some(1), Some(s), r,1);
        let conn_inf = cnf.connection_inf.clone();
        let mut pool = ThreadableRabbitMQConnectionPool::new(conn_inf, 2);
        pool.start();
        let rconn = pool.get_connection();
        if rconn.is_ok() {
            let mut c = rconn.unwrap();
            let channel = c.connection.open_channel(None).unwrap();
            let mut conf = cnf.clone();
            let ja = thread::spawn( move ||{
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
            let jb = thread::spawn( move ||{
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
            let jc = thread::spawn( move ||{
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
