//! Queue send errors
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;


/// Thrown when the connection pool is empty
pub struct QueueReceiveError;


/// Display implementation for when the pool is empty
impl fmt::Display for QueueReceiveError{

    ///Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue Send Failed.")
    }
}


/// Debut for QueueSendError
impl fmt::Debug for QueueReceiveError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
