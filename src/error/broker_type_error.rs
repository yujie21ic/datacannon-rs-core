//!Exchange related error
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;


/// Thrown when the connection pool is empty
pub struct BrokerTypeError;

/// Display implementation for when the pool is empty
impl fmt::Display for BrokerTypeError{

    /// Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong Broker Type")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for BrokerTypeError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
