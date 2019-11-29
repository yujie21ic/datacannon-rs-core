//! Enumeration for storing a broker type
//!
//! ---
//! author: Andrew Evans
//! ---

use tokio::runtime::Runtime;

use crate::app::send_rpc::SendArgs;
use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::broker::kafka::kafka::KafkaBroker;
use crate::statistics::message::Statistics;
use crate::task::config::TaskConfig;
use crate::message_protocol::message_body::MessageBody;
use crate::config::config::CannonConfig;


/// An enumeration of available brokers
///
/// # Arugments
/// * `RabbitMQ` - RabbitMQ broker
/// * `Kafka` - Kafka broker
pub enum AvailableBroker{
    RabbitMQ(RabbitMQBroker),
    Kafka(KafkaBroker),
}


/// For receiving a communication event
///
/// # Arguments
/// * `PING` - For ascertaining health
pub enum CommunicationEvent{
    STATISTICS(Statistics),
    COMPLETE,
    PING
}


/// Enum for individual broker events
///
/// # Arguments
/// * `SEND` - For sending messages to the broker
/// * `POISON_PILL` - Kill the future
#[derive(Clone, Debug)]
pub enum BrokerEvent{
    SEND(SendArgs),
    GETSTATS,
    POISONPILL
}


/// The broker
pub trait Broker{

    /// Restart a future in the broker
    fn create_fut(&mut self, runtime: &Runtime);

    /// start the broker
    fn setup(&mut self, runtime: &Runtime);

    /// tear down the broker
    fn teardown(&mut self);

    /// close the broker which should call teardown
    fn close(&mut self);

    /// send a task
    fn send_task(&mut self, runtime: &Runtime, task: TaskConfig, message_body: Option<MessageBody>);

    /// Allows workers to subscribe to the broker
    fn subscribe_to_queues(&mut self, runtime: &Runtime, config: &CannonConfig);

    /// Drop a specific future on failure
    fn drop_future(&mut self, idx: usize);
}
