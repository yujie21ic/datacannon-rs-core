//! General queue trait
//!
//! ---
//! author: Andrew Evans
//! ---


/// Queue Handler for all queues
pub trait QueueHandler{
    /// Get the queue name `std::string::String`
    fn get_name(&self) -> String;
}