//! A general pool trait
//!
//! ---
//! author: Andrew Evans
//! ---


use crate::error::pool_creation_error::PoolCreationError;
use crate::app::context::Context;

/// Contains the pool functions
pub trait Pool{
    /// Drop all connections
    fn drop_connections(&mut self, context: &mut Context);

    /// Closes the pool
    fn close(&mut self, context: &mut Context);
}