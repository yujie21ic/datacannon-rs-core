//! QOS ERROR
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;


/// Thrown when the connection pool is empty
pub struct QOSError;

/// Display implementation for when the pool is empty
impl fmt::Display for QOSError{

    /// Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QOS Error")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for QOSError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
