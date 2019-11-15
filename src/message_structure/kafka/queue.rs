//! Structure and functions storing data for when kafka acts as a queue
//!
//! ---
//! author: Andrew Evans
//! ---


/// Creates a kafka structure
///
/// # Arguments
/// * `queue` - Name of the queue
/// * `default_exchange` - Exchange for te
pub struct KafkaQueue{
    queue: String,
    default_exchange: String,
    ha_policy: Option<i8>,

}
