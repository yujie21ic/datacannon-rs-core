//! Basic configuration for the tasks
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::argparse::args::Args;
use crate::argparse::kwargs::KwArgs;
use crate::config::config::CannonConfig;
use uuid::Uuid;
use chrono::Utc;


/// Task Configuration
///
/// # Arguments
/// * `task_name` - The task name string
/// * `reply_to` - Queue to reply to
/// * `correlation_id` - The unique correlation id
/// * `expires` - Expiration time for the task
/// * `priority` - Priority for the task
/// * `time_limit` - Time limit for the task
/// * `soft_time_limit` - Soft time limit for the Task
/// * `eta` - Estimated time of arrival
/// * `retries` - Number of retries
pub struct TaskConfig{
    task_name: String,
    args: Args,
    kwargs: KwArgs,
    reply_to: String,
    correlation_id: String,
    expires: i64,
    priority: i8,
    time_limit: i64,
    soft_time_limit: i64,
    eta: String,
    retries: i8,
}


/// Implementation for the task config
impl TaskConfig{

    /// Get the number of retries `std::i8`
    pub fn get_retries(&self) -> i8{
        self.retries.clone()
    }

    /// Get the eta date `std::String`
    pub fn get_eta(&self) -> String{
        self.eta.clone()
    }

    /// Get the soft time limit `std::i64`
    pub fn get_soft_time_limit(&self) -> i64{
        self.soft_time_limit.clone()
    }

    /// Get the time limit `std::i64`
    pub fn get_time_limit(&self) -> i64{
        self.time_limit.clone()
    }

    /// Get the priority `std::i8`
    pub fn get_priority(&self) -> i8{
        self.priority.clone()
    }

    /// Get the time to live
    pub fn get_expires(&self) -> i64{
        self.expires.clone()
    }

    /// Get the correlation id `std::String`
    pub fn get_correlation_id(&self) -> String{
        self.correlation_id.clone()
    }

    /// Get the reply to queue `std::String`
    pub fn get_reply_to(&self) -> String{
        self.reply_to.clone()
    }

    /// Get the `crate::argparse::kwargs::KwArgs`
    pub fn get_kwargs(&self) -> KwArgs{
        self.kwargs.clone()
    }

    /// Get the `crate::argparse::args::Args`
    pub fn get_args(&self) -> Args{
        self.args.clone()
    }

    /// Get the task name `std::String`
    pub fn get_task_name(&self) -> String{
        self.task_name.clone()
    }

    /// Create a new configuration
    pub fn new(
        config: &CannonConfig,
        task_name: String,
        args: Option<Args>,
        kwargs: Option<KwArgs>,
        reply_to: Option<String>,
        correlation_id: Option<String>,
        expires: Option<String>,
        priority: Option<i8>,
        time_limit: Option<i64>,
        soft_time_limit: Option<i64>,
        eta: Option<String>,
        retries: Option<i8>) -> Taskconfig{

        let mut targs = Args::new();
        if args.is_some(){
            targs = args.unwrap();
        }

        let mut tkwargs = KwArgs::new();
        if kwargs.is_some(){
            tkwargs = kwargs.unwrap();
        }

        let mut corrid = format!("{}", Uuid::new_v4());
        if correlation_id.is_some(){
            corrid = correlation_id.unwrap();
        }

        let mut reply = format!("{}", Uuid::new_v4());
        if reply_to.is_none() && correlation_id.is_some(){
            reply = corrid;
        }else if reply_to.is_some(){
            reply = reply_to.unwrap();
        }

        let mut expires =
        TaskConfig{
            task_name: task_name,

        }
    }
}
