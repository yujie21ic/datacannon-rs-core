//! Enumeration for storing a broker type
//!
//! ---
//! author: Andrew Evans
//! ---

use tokio::sync::mpsc::Sender;
use tokio::runtime::Runtime;

use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::argparse::argtype::ArgType;
use crate::connection::pool::Pool;
use crate::message_protocol::message::Message;
use crate::broker::kafka::kafka::KafkaBroker;
use crate::statistics::message::Statistics;
use crate::app::send_rpc::SendArgs;
use crate::task::config::TaskConfig;


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
    GET_STATS,
    POISON_PILL
}


/// The broker
pub trait Broker{

    /// Restart a future in the broker
    fn create_fut(&mut self, runtime: &Runtime);

    /// start the broker
    async fn setup(&mut self);

    /// tear down the broker
    async fn teardown(&mut self);

    /// close the broker which should call teardown
    async fn close(&mut self);

    /// send a task
    fn send_task(&mut self, runtime: &Runtime, task: TaskConfig);

    /// Drop a specific future on failure
    async fn drop_future(&mut self, idx: usize);
}
