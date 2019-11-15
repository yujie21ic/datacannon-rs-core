/*
Store application information to setup existing queues

Author Andrew Evans
*/


use amiquip::{Queue, Channel};
use crate::message_structure::kafka_trait::QueueHandler;
use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::message_structure::queues::QueueOptions;
use crate::message_structure::amqp::amqp_trait::AMQPQueueHandler;
use crate::message_protocol::message::Message;
use crate::error::queue_error::QueueError;
use crate::broker::amqp::rabbitmq::RabbitMQBroker;


/// AMQP Queue
#[derive(Clone, Debug)]
pub struct AMQPQueue {
    name: String,
    exchange: Option<String>,
    routing_key: Option<String>,
    max_priority: i8,
    ha_policy: String,
    conn_inf: AMQPConnectionInf,
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

    /// Performs setup operations and calls create.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations on
    fn setup(&self, channel: &Channel) -> Result<bool, QueueError>{
        Ok(false)
    }

    /// Performs teardown operations and calls drop
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    fn teardown(&self, channel: &Channel) -> Result<bool, QueueError>{
        Ok(false)
    }

    /// Sends messages to the queue
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    fn send(&self, channel: &Channel, message: Message) -> Result<bool, QueueError>{
        Ok(false)
    }

    /// Create the queue
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    fn create(&self, channel: &channel) -> Result<bool, QueueError>{
        RabbitMQBroker::
        Ok(false)
    }

    /// Drop the Queue
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for dropping messages
    fn drop(&self, channel: &channel) -> Result<bool, QueueError>{
        Ok(false)
    }
}


/// unique implementation
impl AMQPQueue {


    /// Handle the queue creation
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    fn do_create(&self, channel: &Channel) -> Result<bool, QueueError>{
        self.create(channel)
    }

    /// Handle sending a message to the queue
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `message` - The `crate::message_protocol::message::Message` to send
    fn do_send(&self, channel: &Channel, message: Message) -> Result<bool, QueueError>{
        self.send(channel, message)
    }

    /// create a new AMQP queue
    fn new(name: String,
           exchange: Option<String>,
           routing_key: Option<String>,
           max_priority: i8,
           ha_policy: String,
           conn_inf: AMQPConnectionInf) -> AMQPQueue{
        AMQPQueue{
            name: name,
            exchange: exchange,
            routing_key: routing_key,
            max_priority: max_priority,
            ha_policy: ha_policy,
            conn_inf: conn_inf,
        }
    }
}
