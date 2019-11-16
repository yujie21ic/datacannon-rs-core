//! Error for a Failed connection
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;


/// Thrown when the connection pool is empty
pub struct QueueTypeError;

/// Display implementation for when the pool is empty
impl fmt::Display for QueueTypeError{

    /// Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue Type Error")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for QueueTypeError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
