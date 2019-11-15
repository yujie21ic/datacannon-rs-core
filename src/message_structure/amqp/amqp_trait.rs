//! AMQP Trait for the queue
//!
//! ---
//! author: Andrew Evans
//! ---

use amiquip::Channel;
use crate::message_protocol::message::Message;
use crate::error::queue_error::QueueError;


/// Trait implementing required queue functions.
pub trait AMQPQueueHandler{

    /// Performs setup operations and calls create.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations on
    fn setup(&self, channel: &Channel) -> Result<bool, QueueError>;

    /// Performs teardown operations and calls drop
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    fn teardown(&self, channel: &Channel) -> Result<bool, QueueError>;

    /// Sends messages to the queue
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    fn send(&self, channel: &Channel, message: Message) -> Result<bool, QueueError>;

    /// Create the queue
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    fn create(&self, channel: &channel) -> Result<bool, QueueError>;

    /// Drop the Queue
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for dropping messages
    fn drop(&self, channel: &channel) -> Result<bool, QueueError>;
}