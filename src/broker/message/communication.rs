//! Communications Events
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::statistics::message::Statistics;


/// For receiving a communication event
#[derive(Clone, Debug)]
pub enum CommunicationEvent{
    ACKNOWLEDGMENT,
    STATISTICS(Statistics),
    GETSTATISTCS,
    COMPLETE,
    PING,
    PONG,
}