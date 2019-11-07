/*
An enum to store connection pools

Author Andrew Evans
*/

use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;


pub enum Pool{
    RabbitMQ(ThreadableRabbitMQConnectionPool),
}
