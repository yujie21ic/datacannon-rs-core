//! For dropping an exchange
//!
//! ---
//! author: Andrew Evans
//! ---


/// Drop Exchange config
pub struct DropExchange{
    pub exchange: String,
    pub nowait: bool,
}
