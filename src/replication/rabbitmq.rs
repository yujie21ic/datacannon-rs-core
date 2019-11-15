//! RabbitMQ HA Policy
//!
//! ---
//! author: Andrew Evans
//! ---


#[derive(Clone, Debug)]
pub struct RabbitMQHAPolicy{
    ha_policy: String,
    replication_factor: i8,
}