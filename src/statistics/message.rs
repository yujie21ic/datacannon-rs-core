//! The statistics message
//!
//! ---
//! author: Andrew Evans
//! ---


/// Statistics related message
///
/// # Argument
/// * `messages_sent` - Number of messages sent
/// * `messages_received` - Number of messages received
pub struct Statistics{
    pub messages_sent: usize,
    pub messages_received: usize,
}