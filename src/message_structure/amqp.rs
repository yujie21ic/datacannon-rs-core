/*
Store application information to setup existing queues

Author Andrew Evans
*/


use amiquip::Queue;
use crate::message_structure::queue_trait::QueueHandler;
use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;
use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::message_structure::queues::QueueOptions;


/// AMQP Queue
#[derive(Clone, Debug)]
pub struct AMQPQueue {
    name: String,
    exchange: Option<String>,
    routing_key: Option<String>,
    max_priority: i8,
    ha_policy: String,
    conn_inf: AMQPConnectionInf,
}


/// implements the queue handler
impl QueueHandler for AMQPQueue {

    /// create the queue
    fn create(&self, name: String, qopts: QueueOptions) {
        unimplemented!()
    }

    /// send to the queue
    fn send(&self) {
        unimplemented!()
    }
}


/// unique implementation
impl AMQPQueue {

    fn do_create(&self){

    }

    fn do_send(&self){

    }

    /// create a new AMQP queue
    fn new(name: String,
           exchange: Option<String>,
           routing_key: Option<String>,
           max_priority: i8,
           ha_policy: String,
           conn_inf: AMQPConnectionInf) -> AMQPQueue{
        AMQPQueue{
            name: name,
            exchange: exchange,
            routing_key: routing_key,
            max_priority: max_priority,
            ha_policy: ha_policy,
            conn_inf: conn_inf,
        }
    }
}
