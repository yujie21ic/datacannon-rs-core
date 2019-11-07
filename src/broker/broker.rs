/*
Enumeration for storing a broker type

Author Andrew Evans
*/

use crate::broker::amqp::RabbitMQBroker;


pub enum AvailableBroker{
    RabbitMQ(RabbitMQBroker),
}

pub trait Broker{
    fn send_task();
}
