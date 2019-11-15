//! Connection information
//!
//! ---
//! author: Andrew Evans
//!---


/// Connection information for Kafka
///
/// # Arguments
/// * `ack_timeout` - Acknowledgement timeout
/// * `num_acks` - Number of acknolwedgements to accept
/// * `host` - Host
/// * `port` - Port
#[derive(Clone, Debug)]
pub struct KafkaConnectionInf {
    pub ack_timeout: i32,
    pub num_acks: i8,
    pub host: String,
    pub port: String,
}