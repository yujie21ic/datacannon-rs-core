/*
Enumeration for storing a broker type

Author Andrew Evans
*/

use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::argparse::argtype::ArgType;


pub enum AvailableBroker{
    RabbitMQ(RabbitMQBroker),
}


pub trait Broker{
    fn send_task(&self, task: String, args: Vec<ArgType>);
}
