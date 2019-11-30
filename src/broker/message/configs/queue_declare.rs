//! Sent when declaring a queue
//!
//! ---
//! author: Andrew Evans
//! ---


/// Queue Creation config
pub struct CreateQueue{
    pub durable: bool,
    pub queue: String,
    pub nowait: bool,
}
