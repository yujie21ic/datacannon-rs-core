//! General Queues implementation.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::HashSet;

use amiquip::ExchangeType;

use crate::config::config::CannonConfig;
use crate::message_structure::amqp::AMQPQueue;


/// Stores queues for the broker
#[derive(Clone, Debug)]
pub enum GenericQueue {
    AMQPQueue(AMQPQueue),
}


/// Stores a queue with associated information
///
///
///  # Arguments
/// * `queue` - The actual `GenericQueue`
/// * `default_exchange` - Exchange for the queues
/// * `default_routing_key` - Routing key for within the exchange if available
/// * `create_missing` - create missing queues
/// * `ha_policy` - Policy for availability if available
/// * `max_priority` - Priority if available
/// * `consume_from` - Potential list of queues to consume from
pub struct Queue {
    pub queue: GenericQueue,
    pub default_exchange: String,
    pub default_routing_key: String,
    pub ha_policy: String,
    pub max_priority: i8,
    pub consume_from: Vec<GenericQueue>,
}


/// Queues structure storing multipe queues for creation
///
/// # Arguments
/// * `queues` - Stores a `Vec<GenericQueue>`
/// * `default_exchange` - Exchange for the queues
/// * `default_routing_key` - Routing key for within the exchange if available
/// * `create_missing` - create missing queues
/// * `ha_policy` - Policy for availability if available
/// * `max_priority` - Priority if available
/// * `consume_from` - Potential list of queues to consume from
#[derive(Clone, Debug)]
pub struct Queues {
    queues: Vec<GenericQueue>,
    default_exchange: String,
    default_routing_key: Option<String>,
    create_missing: Option<bool>,
    ha_policy: Option<String>,
    max_priority: Option<i8>,
    pub consume_from: Option<Vec<GenericQueue>>,
}


/// Queue options for creating concise functions
///
/// # Arguments
/// * `exchange` - Exchange name if available
/// * `routing_key` - Routing key if available
/// * `exchange_type` - Default exchange type
/// * `max_priority` - Maxpriority if available
pub struct QueueOptions{
    pub exchange: Option<String>,
    pub routing_key: Option<String>,
    pub exchange_type: Option<ExchangeType>,
    pub max_priority: Option<i8>,
}


/// implementation of queues
impl Queues{

    /// add a queue
    pub fn add(&self){

    }

    /// add a compatible queue
    pub fn add_compat(&self, name: String, qopts: Option<QueueOptions>){

    }

    /// deselect a queue for consumption
    pub fn deselect(&self, exclude: Option<HashSet<String>>){

    }

    pub fn new(queues: Vec<GenericQueue>,
        default_exchange: String,
        default_routing_key: Option<String>,
        create_missing: Option<bool>,
        ha_policy: Option<String>,
        max_priority: Option<i8>,
        consume_from: Option<Vec<GenericQueue>>) -> Queues{
        Queues{
            queues: queues,
            default_exchange: default_exchange,
            default_routing_key: default_routing_key,
            create_missing: create_missing,
            ha_policy: ha_policy,
            max_priority: max_priority,
            consume_from: consume_from,
        }
    }
}
