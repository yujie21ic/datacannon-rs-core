//! AMQP Trait for the queue
//!
//! ---
//! author: Andrew Evans
//! ---

use lapin::Channel;
use crate::message_protocol::message::Message;
use crate::error::queue_error::QueueError;
use crate::config::config::CannonConfig;


/// Trait implementing required queue functions.
pub trait AMQPQueueHandler{

    /// Performs setup operations and calls create.Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations on
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn setup(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>;

    /// Performs teardown operations and calls drop. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn teardown(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>;

    /// Sends messages to the queue. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    /// * `message` - The `crate::message_protocol::message::Message`
    fn send(&self, channel: &Channel, config: &CannonConfig, message: Message) -> Result<bool, QueueError>;

    /// Create the queue. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for performing operations
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn create(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>;

    /// Drop the Queue. Returns a `std::Result<std::bool, crate::error::queue_error::QueueError>`.
    /// The return value includes an operation status or an error status.
    ///
    /// # Arguments
    /// * `channel` - The `amiquip::Channel` for dropping messages
    /// * `config` - The `crate::config::config::CannonConfig` for the app
    fn drop(&self, channel: &Channel, config: &CannonConfig) -> Result<bool, QueueError>;
}