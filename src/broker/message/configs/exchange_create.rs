//! Messages for creating the exchange
//!
//! ---
//! author: Andrew Evans
//! ---

use lapin::ExchangeKind;


/// Exchange Creation Config
pub struct ExchangeCreate{
    pub durable: bool,
    pub exchange: String,
    pub exchange_type: ExchangeKind,
    pub nowait: bool,
}