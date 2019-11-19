//! General Queues implementation.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::HashSet;

use amiquip::ExchangeType;

use crate::config::config::CannonConfig;
use crate::message_structure::amqp::queue::AMQPQueue;
use crate::message_structure::kafka::queue::KafkaQueue;
use crate::replication::replication::HAPolicy;
use crate::error::queue_type_error::QueueTypeError;


/// Stores queues for the broker
///
/// # Arguments
/// * `AMQPQueue` - Wrapper for `crate::message_structure::amqp::queue::AMQPQueue`
/// * `KafkaQueue` - Wrapper for `crate::message_structure::kafka::queue::KafkaQueue`
#[derive(Clone, Debug)]
pub enum GenericQueue {
    AMQPQueue(AMQPQueue),
    KafkaQueue(KafkaQueue),
}
