//! RabbitMQ HA Policy
//!
//! ---
//! author: Andrew Evans
//! ---


#[derive(Clone, Debug)]
pub struct RabbitMQHAPolicy{
    pub ha_policy: String,
    pub replication_factor: i8,
}