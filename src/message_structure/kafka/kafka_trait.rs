//! Queue trait
//!
//! ---
//! author: Andrew Evans
//!

use crate::message_protocol::message::Message;


/// Trait implementing required queue functions.
pub trait KafkaQueueHandler{

    /// Performs setup operations and calls create.
    fn setup(&self);

    /// Performs teardown operations and calls drop
    fn teardown(&self);

    /// Sends messages to the queue
    fn send(&self, message: Message);

    /// Create the queue
    fn create(&self);

    /// Drop the Queue
    fn drop(&self);
}
