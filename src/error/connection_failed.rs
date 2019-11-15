//! Error for a Failed connection
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;


/// Thrown when the connection pool is empty
pub struct ConnectionFailed;

/// Display implementation for when the pool is empty
impl fmt::Display for ConnectionFailed{

    /// Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to Establish a Connection to the Broker")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for ConnectionFailed{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
