//! General Trait for the broker
//!
//! ---
//! author: Andrew Evans
//! ---

use amiquip::{Channel, ExchangeType};
use crate::message_protocol::properties::Properties;
use crate::error::exchange_error::ExchangeError;
use crate::message_protocol::message_body::MessageBody;
use crate::message_protocol::headers::Headers;
use crate::error::publish_error::PublishError;
use crate::error::queue_error::QueueError;
use crate::config::config::CannonConfig;


/// AMQP Broker
pub trait AMQPBroker{

    /// Bind queue to the exchange
    fn bind_to_exchange(config: &CannonConfig, channel: &Channel, exchange: String, queue: String, routing_key: String) -> Result<bool, ExchangeError>;

    /// Create a queue
    fn create_queue(config: &CannonConfig, channel: &Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, QueueError>;

    /// Create an exchange
    fn create_exchange(config: &CannonConfig, channel: &Channel, durable: bool, exchange: String, exchange_type: ExchangeType) -> Result<bool, ExchangeError>;

    /// Send task to the broker
    fn do_send(config: &CannonConfig, channel: &Channel, props: Properties, headers: Headers, body: MessageBody, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, PublishError>;

    /// Drop the queue
    fn do_drop_queue(config: &CannonConfig, channel: &Channel, queue: String) -> Result<bool, QueueError>;
}
