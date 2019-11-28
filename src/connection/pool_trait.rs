//! A general pool trait
//!
//! ---
//! author: Andrew Evans
//! ---


use crate::error::pool_creation_error::PoolCreationError;

/// Contains the pool functions
pub trait Pool{

    /// Setup the Pool
    fn setup(&mut self) -> Result<(), PoolCreationError>;

    /// Drop all connections
    fn drop_connections(&mut self);

    /// Closes the pool
    fn close(&mut self);
}