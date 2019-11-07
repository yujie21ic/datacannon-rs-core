/*
Queue trait

Author Andrew Evans
*/


use crate::broker::queues::QueueOptions;


/// Queue handler
pub trait QueueHandler{
    /// create the queue
    fn create(&self, name: String, qopts: QueueOptions);

    /// send
    fn send(&self);
}
