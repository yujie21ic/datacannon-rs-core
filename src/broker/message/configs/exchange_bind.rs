//! Config to bind to an exchange
//!
//! ---
//! author: Andrew Evans
//! ---


/// Config for binding to an exchange
pub struct ExchangeBind{
    pub exchange: String,
    pub queue: String,
    pub routing_key: String,
    pub nowait: bool,
}
