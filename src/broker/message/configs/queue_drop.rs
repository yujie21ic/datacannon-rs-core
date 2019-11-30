//! Config for dropping a queue
//!
//! ---
//! author: Andrew Evans
//! ---


/// Config for dropping a queue
pub struct DropQueue{
    pub queue: String,
    pub nowait: bool,
}
