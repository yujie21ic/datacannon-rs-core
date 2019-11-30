/*
Store application information to setup existing queues

Author Andrew Evans
*/


use lapin::Channel;
use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::message_structure::amqp::amqp_trait::AMQPQueueHandler;
use crate::message_protocol::message::Message;
use crate::error::queue_error::QueueError;
use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::config::config::CannonConfig;
use uuid::Uuid;
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
        Ok(true)
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
        Ok(true)
    }

    /// Drop the Queue. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for dropping messages
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn drop(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>{
        unimplemented!()
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
