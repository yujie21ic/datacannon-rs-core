//! used when a future failed to create
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;


/// Thrown when the connection pool is empty
pub struct FutureCreationError;

/// Display implementation for when the pool is empty
impl fmt::Display for FutureCreationError{

    /// Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to Create Future")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for FutureCreationError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
