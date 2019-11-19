//! RabbitMQ HA Policy
//!
//! ---
//! author: Andrew Evans
//! ---


/// RabbitMQ HA Policies
///
/// # Arguments
/// * `ALL` - Queue is mirrored on all nodes
/// * `EXACTLY` - Queue is mirrored on n nodes
/// * `NODES` - Queue si mirrored on the named nodes
#[derive(Clone, Debug)]
pub enum RabbitMQHAPolicies{
    ALL,
    EXACTLY(i8),
    NODES(Vec<String>),
}


/// RabbitMQ HA Policy
///
/// # Arguments
/// * `ha_policy` - Availability policy
/// * `replication_factor` - Replication factor for the queue
#[derive(Clone, Debug)]
pub struct RabbitMQHAPolicy {
    pub ha_policy: RabbitMQHAPolicies,
    pub replication_factor: i8,
}

/// Implementation of the HA Policy
impl RabbitMQHAPolicy {

    /// Creates a new HA Policy
    ///
    /// # Arguments
    /// * `ha_policy` - HA Policy
    /// * `replication_factor` - Number of times to replicate
    pub fn new(ha_policy: RabbitMQHAPolicies, replication_factor: i8) -> RabbitMQHAPolicy {
        RabbitMQHAPolicy{
            ha_policy: ha_policy,
            replication_factor: replication_factor,
        }
    }
}
