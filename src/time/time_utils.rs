//! Time utilities
//!
//! ---
//! author: Andrew Evans
//! ---

use chrono::Utc;


/// Get a future timestamp
///
/// # Arguments
/// * `offset` - Additional offset seconds
pub fn get_future_date(offset: i64) -> i64{
    let dt = Utc::now().timestamp();
    dt + offset
}
