//! Basic configuration for the tasks
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::argparse::args::Args;
use crate::argparse::kwargs::KwArgs;
use crate::config::config::CannonConfig;
use uuid::Uuid;
use crate::message_protocol::message_body::MessageBody;
use crate::message_protocol::message::Message;
use crate::message_protocol::properties::Properties;
use crate::message_protocol::headers::Headers;


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
    args: Option<Args>,
    kwargs: Option<KwArgs>,
    reply_to: String,
    correlation_id: String,
    result_expires: i64,
    priority: i8,
    time_limit: i64,
    soft_time_limit: i64,
    eta: Option<i64>,
    retries: u8,
    lang: String,
    shadow: Option<String>,
}


/// Implementation for the task config
impl TaskConfig{

    /// Get the number of retries `std::i8`
    pub fn get_retries(&self) -> u8{
        self.retries.clone()
    }

    /// Get the eta date `std::String`
    pub fn get_eta(&self) -> Option<i64>{
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
    pub fn get_result_expires(&self) -> i64{
        self.result_expires.clone()
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
    pub fn get_kwargs(&self) -> Option<KwArgs>{
        self.kwargs.clone()
    }

    /// Get the `crate::argparse::args::Args`
    pub fn get_args(&self) -> Option<Args>{
        self.args.clone()
    }

    /// Get the task name `std::String`
    pub fn get_task_name(&self) -> String{
        self.task_name.clone()
    }

    pub fn get_task_lang(&self) -> String{
        self.lang.clone()
    }

    /// Get the logging nickname `std::String`
    pub fn get_shadow(&self) -> Option<String>{self.shadow.clone()}

    /// Convert to a message body
    ///
    /// # Arguments
    /// * `config` - configuration for non-overidden variables
    /// * `message_body` - Message body to add to the message
    /// * `root_id` - Originating id for the chain
    pub fn to_amqp_message(&self, config: &CannonConfig, message_body: Option<MessageBody>, root_id: Option<String>) -> Message{
        let properties = Properties::new(
            self.correlation_id.clone(),
            config.accept_content.clone(),
            config.encoding_type.clone(),
            Some(self.reply_to.clone()));
        let mut root = self.correlation_id.clone();
        if root_id.is_some(){
            root = root_id.unwrap();
        }
        let mut mbody = MessageBody::new(None,None,None, None);
        if message_body.is_some(){
            mbody = message_body.unwrap();
        }
        let headers = Headers::new(self.lang.clone(), self.task_name.clone(), self.correlation_id.clone(), root);
        let m = Message::new(properties, headers, mbody, self.args, self.kwargs);
        m
    }

    /// Convert to a kafka message
    ///
    /// # Arugments
    /// * `config` - Configuration for hte application
    /// * `message_body` - Optional Message body
    pub fn to_kafka_message(&self, config: &CannonConfig, message_body: Option<MessageBody>){
        unimplemented!()
    }

    /// Create a new configuration
    ///
    /// # Arguments
    /// * `config` - The `crate::config::CannonConfig`
    /// * `task_name` - Name of the task
    /// * `args` - The `crate::argparse::args::Args` for the task
    /// * `kwargs` - The `crate::argparse::kwargs:KwArgs` for the task
    /// * `reply_to` - Queue to reply to
    /// * `correlation_id` - The correlation id
    /// * `result_expires` - Result expiration time to live
    /// * `priority` - Result priority
    /// * `time_limit` - Hard time limit for the task (TTL)
    /// * `soft_time_limit` - Soft time limit ttl
    /// * `eta` - Estimated time of arrival
    /// * `retries` - Number of times to retry the task
    /// * `lang` - Language for the task
    pub fn new(
        config: &CannonConfig,
        task_name: String,
        args: Option<Args>,
        kwargs: Option<KwArgs>,
        reply_to: Option<String>,
        correlation_id: Option<String>,
        result_expires: Option<i64>,
        priority: Option<i8>,
        time_limit: Option<i64>,
        soft_time_limit: Option<i64>,
        eta: Option<i64>,
        retries: Option<u8>,
        lang: Option<String>,
        shadow: Option<String>) -> TaskConfig{

        let mut targs: Option<Args> = None;
        if args.is_some(){
            targs = Some(args.unwrap());
        }

        let mut tkwargs: Option<Kwargs> = None;
        if kwargs.is_some(){
            tkwargs = Some(kwargs.unwrap());
        }

        let mut corrid = format!("{}", Uuid::new_v4());
        if correlation_id.is_some(){
            corrid = correlation_id.clone().unwrap();
        }

        let mut reply = format!("{}", Uuid::new_v4());
        if reply_to.is_none() && correlation_id.is_some(){
            reply = corrid.clone();
        }else if reply_to.is_some(){
            reply = reply_to.unwrap();
        }

        let mut texpires = 36000000;
        if result_expires.is_some(){
            texpires = result_expires.unwrap();
        }

        let mut tpriority = config.task_default_priority.clone();
        if priority.is_some(){
            tpriority = priority.unwrap();
        }

        let mut ttime_limit = 36000000;
        if time_limit.is_some(){
            ttime_limit = time_limit.unwrap();
        }

        let mut tsoft_time_limit = 36000000;
        if soft_time_limit.is_some(){
            tsoft_time_limit = soft_time_limit.unwrap();
        }

        let mut tretries = config.task_retries.clone();
        if retries.is_some(){
            tretries = retries.unwrap();
        }

        let mut tlang = config.default_lang.clone();
        if lang.is_some(){
            tlang = lang.unwrap();
        }

        TaskConfig{
            task_name: task_name,
            args: targs,
            kwargs: tkwargs,
            reply_to: reply,
            correlation_id: corrid,
            result_expires: texpires,
            priority: tpriority,
            time_limit: ttime_limit,
            soft_time_limit: tsoft_time_limit,
            eta: eta,
            retries: tretries,
            lang: tlang,
            shadow: shadow,
        }
    }
}
