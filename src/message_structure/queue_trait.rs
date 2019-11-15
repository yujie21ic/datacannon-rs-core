/*
Queue trait

Author Andrew Evans
*/

use crate::message_structure::queues::QueueOptions;


/// Queue handler
pub trait QueueHandler{
    /// create the queue
    fn create(&self, name: String, qopts: QueueOptions);

    /// send
    fn send(&self);
}
