//! Channel attainment failure
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;


/// Thrown when the connection pool is empty
pub struct ChannelAttainmentError;

/// Display implementation for when the pool is empty
impl fmt::Display for ChannelAttainmentError{

    /// Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to Obtain Channel")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for ChannelAttainmentError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
