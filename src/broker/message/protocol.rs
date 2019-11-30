//! Message protocol for broker tasks
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::task::config::TaskConfig;
use crate::message_protocol::message::Message;


/// Broker Message
#[derive(Clone, Debug)]
pub enum BrokerMessage{
    TASK(Task),
    POISONPILL,

}


/// Task types common to queue based brokers
#[derive(Clone, Debug)]
pub enum TaskType{
    CREATEQUEUE(Task),
    CREATEEXCHANGE(Task),
    SENDTASK(Task),
    BINDTOEXCHANGE(Task),
}


/// Task structure
#[derive(Clone, Debug)]
pub struct Task{
    task: TaskConfig,
    message: Message,
}


/// Task implementation
impl Task{

    /// Create a new task
    ///
    /// # Arguments
    /// * `task` - The task configuration
    /// * `message` - The message to send
    pub fn new(task: TaskConfig, message: Message) -> Task{
        Task{
            task: task,
            message: message,
        }
    }
}
