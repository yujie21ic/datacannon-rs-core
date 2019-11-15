//! Kafka HA Policy
//!
//! ---
//! author: Andrew Evans
//! ---


#[derive(Clone, Debug)]
pub struct KafkaHAPolicy{
    replication_factor: i8,
}
