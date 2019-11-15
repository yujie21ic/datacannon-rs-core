//! Basic configuration for the tasks
//!
//! ---
//! author: Andrew Evans
//! ---


/// Task Configuration
///
/// # Arguments
/// * `task_name` - The task name string
/// * `reply_to` - Queue to reply to
/// * `correlation_id` - The unique correlation id
/// * `expires`
pub struct TaskConfig{
    task_name: String,
    reply_to: String,
    correlation_id: String,
    expires: String,
    priority: i8,
    time_limit: i64,
    soft_time_limit: i64,
    eta: String,
    retries: i8,
}
