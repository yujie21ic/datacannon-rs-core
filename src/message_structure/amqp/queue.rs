/*
Store application information to setup existing queues

Author Andrew Evans
*/


use amiquip::{AmqpProperties, Queue, Channel, QueueDeclareOptions};
use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::message_structure::amqp::amqp_trait::AMQPQueueHandler;
use crate::message_protocol::message::Message;
use crate::error::queue_error::QueueError;
use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::config::config::CannonConfig;
use uuid::Uuid;
use crate::broker::amqp::broker_trait::AMQPBroker;
use crate::connection::connection::ConnectionConfig;
use crate::replication::replication::HAPolicy;
use crate::message_structure::queue_trait::QueueHandler;


/// AMQP Queue
#[derive(Clone, Debug)]
pub struct AMQPQueue {
    name: String,
    exchange: Option<String>,
    routing_key: Option<String>,
    max_priority: i8,
    ha_policy: HAPolicy,
    conn_inf: AMQPConnectionInf,
    is_durable: bool,
}


/// Implements the base queue functions
impl QueueHandler for AMQPQueue {
    /// Get the queue name `std::string::String`
    fn get_name(&self) -> String {
        self.name.clone()
    }
}


/// Implements the protocol specific functions
impl AMQPQueueHandler for AMQPQueue {


    /// Performs setup operations and calls create.Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations on
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn setup(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>{
        let r = self.create(channel, config);
        if r.is_ok() {
            Ok(true)
        }else{
            Err(QueueError)
        }
    }

    /// Performs teardown operations and calls drop. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn teardown(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>{
        let r = self.drop(channel, config);
        if r.is_ok() {
            Ok(true)
        }else{
            Err(QueueError)
        }
    }

    /// Sends messages to the queue. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    /// * `message` - The `crate::message_protocol::message::Message`
    fn send(&self, channel: &Channel, config: &CannonConfig, message: Message) -> Result<bool, QueueError>{
        let props = message.properties;
        let headers = message.headers;
        let body = message.body;
        let mut exchange = config.default_exchange.clone();
        if self.exchange.is_some(){
            exchange = self.exchange.clone().unwrap();
        }
        let mut routing_key = config.default_routing_key.clone();
        if self.routing_key.is_some(){
            routing_key = self.routing_key.clone().unwrap();
        }
        let r = RabbitMQBroker::do_send(config, channel, props, headers, body, Some(exchange), Some(routing_key));
        if r.is_ok() {
            Ok(true)
        }else{
            Err(QueueError)
        }
    }


    /// Create the queue. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn create(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>{
        let uuid = Uuid::new_v4();
        let unique_id = format!("{}", uuid);
        let mut exchange = config.default_exchange.clone();
        if self.exchange.is_some(){
            exchange = self.exchange.clone().unwrap();
        }
        let mut routing_key = config.default_routing_key.clone();
        if self.routing_key.is_some(){
            routing_key = self.routing_key.clone().unwrap();
        }
        RabbitMQBroker::create_queue(config, channel, self.is_durable, self.name.clone(), true, unique_id, Some(exchange), Some(routing_key))
    }

    /// Drop the Queue. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for dropping messages
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn drop(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>{
        let r = RabbitMQBroker::do_drop_queue(config, channel, self.name.clone());
        if r.is_ok(){
            Ok(true)
        }else {
            Err(QueueError)
        }
    }
}


/// unique implementation
impl AMQPQueue {


    /// Handle the queue creation
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    pub fn do_create(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>{
        self.create(channel, config)
    }

    /// Handle sending a message to the queue
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    /// * `message` - The `crate::message_protocol::message::Message` to send
    pub fn do_send(&self, channel: &Channel, config: &CannonConfig, message: Message) -> Result<bool, QueueError>{
        self.send(channel, config, message)
    }

    /// Create a new AMQP queue
    ///
    /// # Arguments
    /// * `name` - Name of the queue
    /// * `exchange` - The exchange name
    /// * `routing_key` - Routing key for queue
    /// * `max_priority` - The maximum priority
    /// * `ha_policy` - Availability policy
    /// * `is_durable` - Whether the queue is durable
    /// * `conn_inf` - The connection information
    pub fn new(name: String,
           exchange: Option<String>,
           routing_key: Option<String>,
           max_priority: i8,
           ha_policy: HAPolicy,
           is_durable: bool,
           conn_inf: AMQPConnectionInf) -> AMQPQueue{
        AMQPQueue{
            name: name,
            exchange: exchange,
            routing_key: routing_key,
            max_priority: max_priority,
            ha_policy: ha_policy,
            conn_inf: conn_inf,
            is_durable: is_durable,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::{panic, thread};
    use amiquip::{Channel, Result};
    use crate::broker::amqp::rabbitmq::RabbitMQBroker;
    use crate::connection::connection::ConnectionConfig;
    use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
    use uuid::Uuid;
    use crate::broker::amqp::broker_trait::AMQPBroker;
    use crate::replication::rabbitmq::{RabbitMQHAPolicy, RabbitMQHAPolicies};
    use crate::connection::kafka::connection_inf::KafkaConnectionInf;
    use crate::security::ssl::SSLConfig;
    use crate::security::uaa::UAAConfig;
    use crate::backend::config::BackendConfig;
    use crate::message_protocol::properties::Properties;
    use crate::message_protocol::message_body::MessageBody;
    use crate::message_protocol::headers::Headers;
    use crate::router::router::Routers;

    fn get_message() -> Message{
        let body = MessageBody::new(None, None, None, None);
        let uuid = Uuid::new_v4();
        let ustr = format!("{}", uuid);
        let headers = Headers::new("rs".to_string(), "test_task".to_string(), ustr.clone(), ustr.clone());
        let reply_queue = Uuid::new_v4();
        let props = Properties::new(ustr.clone(), "application/json".to_string(), "utf-8".to_string(), None);
        let message = Message::new(props, headers, body, None, None);
        message
    }

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
        let routers = Routers::new();
        let conf = CannonConfig::new(ConnectionConfig::RabbitMQ(broker_conn), backend, routers);
        conf
    }

    fn get_test_queue() -> AMQPQueue{
        let protocol = "amqp".to_string();
        let host = "127.0.0.1".to_string();
        let port = 5672;
        let vhost = Some("test".to_string());
        let username = Some("dev".to_string());
        let password = Some("rtp*4500".to_string());
        let broker_conn = AMQPConnectionInf::new(protocol, host, port, vhost, username, password, false, None, None);
        let policy = RabbitMQHAPolicy{
            ha_policy: RabbitMQHAPolicies::ALL,
            replication_factor: 1,
        };
        AMQPQueue::new(
            "test".to_string(),
            Some("test_exchange".to_string()),
        Some("test_routing_key".to_string()),
        1,
        HAPolicy::RabbitMQ(policy),
            true,
            broker_conn)
    }

    fn drop_test_queue(channel: &Channel){
        let q = get_test_queue();
        let config = get_config(None, None);
        let r= q.drop(channel, &config);
        assert!(r.is_ok());
    }

    #[test]
    fn should_create_the_queue(){
        let mut conf = get_config(None, None);
        let rmq = RabbitMQBroker::new(&mut conf, None,  Some(1), 1);
        let mut conn_inf = conf.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 2);
            pool.start();
            let rconn = pool.get_connection();
            if rconn.is_ok() {
                let mut c = rconn.unwrap();
                let channel = c.connection.open_channel(None).unwrap();
                let j = thread::spawn(move|| -> Result<bool, QueueError> {
                    let q = get_test_queue();
                    let cid = channel.channel_id();
                    let r = q.do_create(&channel, &conf.clone());
                    if r.is_ok(){
                        Ok(r.ok().unwrap())
                    }else{
                        Err(r.err().unwrap())
                    }
                });
                let result = j.join();
                let close_channel = c.connection.open_channel(None).unwrap();
                let j2 = thread::spawn(move || ->  Result<()>{
                    drop_test_queue(&close_channel);
                    Ok(())
                });
                let close_result = j2.join();
                c.connection.close();
                assert!(close_result.is_ok());
                assert!(result.is_ok());
                assert!(result.ok().unwrap().is_ok());
            }
        }
    }

    #[test]
    fn should_drop_the_queue(){
        let mut conf = get_config(None, None);
        let rmq = RabbitMQBroker::new(&mut conf, None, Some(1),  1);
        let mut conn_inf = conf.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 2);
            pool.start();
            let rconn = pool.get_connection();
            if rconn.is_ok() {
                let mut c = rconn.unwrap();
                let channel = c.connection.open_channel(None).unwrap();
                let j = thread::spawn(move|| -> Result<()>{
                    let q = get_test_queue();
                    let mut r = q.do_create(&channel, &conf);
                    assert!(r.is_ok());
                    Ok(())
                });
                let result = j.join();
                let dchannel = c.connection.open_channel(None).unwrap();
                let j2 = thread::spawn(move|| -> Result<()>{
                    drop_test_queue(&dchannel);
                    Ok(())
                });
                let drop_result = j2.join();
                c.connection.close();
                assert!(result.is_ok());
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn should_send_message_to_queue(){
        let mut conf = get_config(None, None);
        let rmq = RabbitMQBroker::new(&mut conf, None, Some(1),  1);
        let mut conn_inf = conf.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 2);
            pool.start();
            let rconn = pool.get_connection();
            if rconn.is_ok() {
                let mut c = rconn.unwrap();
                let channel = c.connection.open_channel(None).unwrap();
                let j = thread::spawn(move|| -> Result<()>{
                    let q = get_test_queue();
                    let mut r = q.do_create(&channel, &conf);
                    assert!(r.is_ok());
                    let message = get_message();
                    r = q.send(&channel, &conf, message);
                    assert!(r.is_ok());
                    Ok(())
                });
                let result = j.join();
                let dchannel = c.connection.open_channel(None).unwrap();
                let j2 = thread::spawn(move|| -> Result<()>{
                    drop_test_queue(&dchannel);
                    Ok(())
                });
                let drop_result = j2.join();
                c.connection.close();
                assert!(result.is_ok());
            } else {
                assert!(false);
            }
        }
    }
}