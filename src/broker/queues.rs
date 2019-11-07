/*
General Queues implementation

Author Andrew Evans
*/


use crate::config::config::CeleryConfig;
use amiquip::ExchangeType;
use std::collections::HashSet;
use crate::queues::amqp::AMQPQueue;


/// stores queues for the broker
#[derive(Clone, Debug)]
pub enum GenericQueue {
    AMQPQueue(AMQPQueue),
}


/// Queues structure
#[derive(Clone, Debug)]
pub struct Queues {
    queues: Vec<GenericQueue>,
    default_exchange: String,
    default_routing_key: String,
    create_missing: bool,
    ha_policy: String,
    max_priority: i8,
    pub consume_from: Vec<GenericQueue>,
}


/// Queue options for functions
pub struct QueueOptions{
    pub exchange: Option<String>,
    pub routing_key: Option<String>,
    pub exchange_type: Option<ExchangeType>,
    pub max_priority: Option<i8>,
}


/// implementation of queues
impl Queues{

    /// add a queue
    fn add(&self){

    }

    /// add a compatible queue
    fn add_compat(&self, name: String, qopts: Option<QueueOptions>){

    }

    /// deselect a queue for consumption
    fn deselect(&self, exclude: Option<HashSet<String>>){

    }

    fn new(queues: Vec<GenericQueue>,
        default_exchange: String,
        default_routing_key: String,
        create_missing: bool,
        ha_policy: String,
        max_priority: i8,
        consume_from: Vec<GenericQueue>) -> Queues{
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
