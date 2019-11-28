//! Channel creation error
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;


/// Thrown when the connection pool is empty
pub struct ChannelCreationError;

/// Display implementation for when the pool is empty
impl fmt::Display for ChannelCreationError{

    /// Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong Broker Type")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for ChannelCreationError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
