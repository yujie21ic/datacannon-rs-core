/*
Enumeration for storing a broker type

Author Andrew Evans
*/

use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::argparse::argtype::ArgType;
use crate::connection::pool::Pool;


pub enum AvailableBroker{
    RabbitMQ(RabbitMQBroker),
}


pub trait Broker{
    fn send_task(&self, pool: &mut Pool, task: String, args: Vec<ArgType>);
}
