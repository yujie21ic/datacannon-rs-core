//! Generic storage for connection pools
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::connection::amqp::rabbitmq_connection_pool::ThreadableRabbitMQConnectionPool;

/// Enum storage for relevant pool types
pub enum Pool{
    RabbitMQ(ThreadableRabbitMQConnectionPool),
}
