//! Replication policy structure to allow for use of different brokers
//!
//! ---
//! author: Andrew Evans
//! ---


use crate::replication::kafka::KafkaHAPolicy;
use crate::replication::rabbitmq::RabbitMQHAPolicy;

/// HA Policy enum storing relevant policy
#[derive(Clone, Debug)]
pub enum HAPolicy{
    Kafka(KafkaHAPolicy),
    RabbitMQ(RabbitMQHAPolicy),
}
