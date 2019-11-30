//! Communications Events
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::statistics::message::Statistics;
use crate::broker::message::task::TaskType;


/// For receiving a communication event
#[derive(Clone, Debug)]
pub enum CommunicationEvent{
    ACKNOWLEDGMENT,
    STATISTICS(Statistics),
    TASK(TaskType),
    GETSTATISTCS,
    COMPLETE,
    PING,
    PONG,
}