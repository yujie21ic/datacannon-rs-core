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
    default_exchange: Option<String>,
    default_routing_key: Option<String>,
    create_missing: Option<bool>,
    ha_policy: Option<HAPolicy>,
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

    /// Add a queue to the list of queues
    ///
    /// # Arguments
    /// * `queue` - Add a `crate:message_struct::queues::GenericQueue`
    pub fn add(&mut self, queue: GenericQueue){
        self.queues.push(queue);
    }

    /// Set the default exchange
    ///
    /// # Arguments
    /// * `exchange_name` - The exchange name to use
    pub fn set_default_exchange(&mut self, exchange_name: String){
        self.default_exchange = Some(exchange_name);
    }

    /// Set default routing key
    ///
    /// # Arguments
    /// * `routing_key` - The routing key to use
    pub fn set_default_routing_key(&mut self, routing_key: String){
        self.default_routing_key = Some(routing_key);
    }

    /// Whether to create missing queues
    ///
    /// # Arguments
    /// * `create_missing` - Whether to create missing queues
    pub fn do_create_missing(&mut self, create_missing: bool){
        self.create_missing = Some(create_missing);
    }

    /// Set the HA Policy
    ///
    /// # Arguments
    /// * `ha_policy` - The `crate::replication::replication::HAPolicy`
    pub fn set_ha_policy(&mut self, ha_policy: Option<HAPolicy>){
        self.ha_policy = ha_policy;
    }

    /// Se the maximum priority
    ///
    /// # Arguments
    /// * `max_priority` - The maximum priority for the queue
    pub fn set_max_priority(&mut self, max_priority: i8){
        self.max_priority = Some(max_priority);
    }

    /// Deselect the specified queue
    ///
    /// # Arguments
    /// * `qname` - Name of the queue
    pub fn deselect_queue(&mut self, qname: String) -> Result<bool, QueueTypeError>{
        let new_qs = Vec::<GenericQueue>::new();
        let mut found_item = false;
        for i in 0..self.queues.len(){
            let q = self.queues.get(i).unwrap();
            if let GenericQueue::KafkaQueue(q) = q {

            }else if let GenericQueue::AMQPQueue(q) = q{

            }else{
                return Err(QueueTypeError)
            }
        }
        Ok(found_item)
    }

    /// Deselect a queue for consumption
    ///
    /// # Arguments
    /// * ``
    pub fn deselect(&mut self, exclude: Option<HashSet<String>>){
        unimplemented!()
    }

    /// Get a queue
    pub fn get_queue(&mut self, queue_name: String) -> Queues{
        unimplemented!()
    }

    /// Create a new queue set
    pub fn new(queues: Vec<GenericQueue>,
        default_exchange: Option<String>,
        default_routing_key: Option<String>,
        create_missing: Option<bool>,
        ha_policy: Option<HAPolicy>,
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
