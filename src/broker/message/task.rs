//! Message protocol for broker tasks
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::broker::message::configs::*;
use crate::task::config::TaskConfig;
use crate::message_protocol::message::Message;
use crate::message_protocol::message_body::MessageBody;


/// Task types common to queue based brokers
#[derive(Clone, Debug)]
pub enum TaskType{
    CREATEQUEUE(queue_declare::CreateQueue),
    CREATEEXCHANGE(exchange_create::ExchangeCreate),
    SENDTASK(Task),
    BINDTOEXCHANGE(exchange_bind::ExchangeBind),
    SETPREFETCHLIMIT(prefetch_set::SetPrefetchLimit),
    DROPQUEUE(queue_drop::DropQueue),
    DROPEXCHANGE(exchange_drop::DropExchange),
}


/// Task structure
#[derive(Clone, Debug)]
pub struct Task{
    task: TaskConfig,
    body: Option<MessageBody>,
    root_id: Option<String>,
    exchange: String,
    routing_key: String,
}


/// Task implementation
impl Task{

    pub fn get_routing_key(&self) -> String{
        self.routing_key.clone()
    }

    /// Obtain the exchange
    pub fn get_exchange(&self) -> String{
        self.exchange.clone()
    }

    /// Get the task config
    pub fn get_task_config(&self) -> &TaskConfig{
        &self.task
    }

    /// Ge the message body
    pub fn get_body(&self) -> Option<MessageBody>{
        self.body.clone()
    }

    /// Get the root id
    pub fn get_root_id(&self) -> Option<String>{
        self.root_id.clone();
    }

    /// Create a new task
    ///
    /// # Arguments
    /// * `task` - The task configuration
    /// * `exchange` - Exchange to send the task to
    /// * `routing_key` - Routing key to send the task to
    /// * `message_body` - The message body for the task (managed by chords and chain handlers)
    /// * `root_id` - Root or parent id managed with chords and chains
    pub fn new(task: TaskConfig, exchange: String, routing_key: String, message_body: Option<MessageBody>, root_id: Option<String>) -> Task{
        Task{
            task: task,
            exchange: exchange,
            routing_key: routing_key,
            body: message_body,
            root_id: root_id,
        }
    }
}
