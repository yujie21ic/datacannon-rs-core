//! General Queues implementation.
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::message_structure::amqp::queue::AMQPQueue;
use crate::message_structure::kafka::queue::KafkaQueue;


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
