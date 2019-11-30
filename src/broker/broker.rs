//! Enumeration for storing a broker type
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::app::send_rpc::SendArgs;
use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::broker::kafka::kafka::KafkaBroker;
use crate::task::config::TaskConfig;
use crate::message_protocol::message_body::MessageBody;
use crate::config::config::CannonConfig;
use crate::error::future_creation_error::FutureCreationError;
use crate::app::context::Context;


/// An enumeration of available brokers
///
/// # Arugments
/// * `RabbitMQ` - RabbitMQ broker
/// * `Kafka` - Kafka broker
pub enum AvailableBroker{
    RabbitMQ(RabbitMQBroker),
    Kafka(KafkaBroker),
}


/// The broker
pub trait Broker{

    /// Restart a future in the broker
    fn create_fut(&mut self, context: &mut Context) -> Result<bool, FutureCreationError>;

    /// tear down the broker
    fn teardown(&mut self);

    /// close the broker which should call teardown
    fn close(&mut self);

    /// send a task
    fn send_task(&mut self, context: &mut Context, task: TaskConfig, message_body: Option<MessageBody>);

    /// Allows workers to subscribe to the broker
    fn subscribe_to_queues(&mut self, context: &mut Context, config: &CannonConfig);

    /// Drop a specific future on failure
    fn drop_future(&mut self, idx: usize);
}
