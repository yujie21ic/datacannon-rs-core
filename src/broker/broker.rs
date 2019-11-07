/*
Enumeration for storing a broker type

Author Andrew Evans
*/

use crate::broker::amqp::RabbitMQBroker;


pub enum Broker{
    RabbitMQ(RabbitMQBroker),
}
