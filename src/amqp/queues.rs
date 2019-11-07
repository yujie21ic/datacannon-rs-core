/*
Store application information to setup existing queues

Author Andrew Evans
*/


#[derive(Clone, Debug)]
pub struct AMQPQueue {
    name: String,
    exchange: Option<String>,
    routing_key: Option<String>,
    max_priority: i8,
    ha_policy: String,
}


impl AMQPQueue {

}

