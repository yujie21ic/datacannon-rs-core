//! Stores join handle, senders, and receivers
//!
//! ---
//! author: Andrew Evans
//! ---

use tokio::sync::mpsc::{Sender, Receiver};
use tokio::task::JoinHandle;

use crate::broker::message::communication::CommunicationEvent;
use crate::app::context::Context;
use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::error::send_error::QueueSendError;
use crate::error::receive_error::QueueReceiveError;
use std::sync::{Mutex, Arc};
use std::borrow::Borrow;


/// Structure containing a running future in the broker and queues for communication
pub struct BrokerFuture{
    handle: JoinHandle<()>,
    sender: Sender<CommunicationEvent>,
    receiver: Arc<Mutex<Receiver<CommunicationEvent>>>,
}


/// Implementation of the future
impl BrokerFuture{

    /// Receive an event from the queue
    ///
    /// # Arguments
    /// * `context` - The application context
    /// * `event` - A communication event
    pub fn receive_event(&self, context: &mut Context, event: CommunicationEvent) -> Result<CommunicationEvent, QueueReceiveError>{
        let rec_ref = self.receiver.clone();
        let r = context.get_runtime().block_on(async move{
           rec_ref.lock().unwrap().recv().await
        });
        if r.is_some(){
            Ok(r.unwrap())
        }else{
            Err(QueueReceiveError)
        }
    }

    /// Send a task to the future
    ///
    /// # Arguments
    /// * `context` - The context for the application
    /// * `event` - The event to send
    pub fn send_task(&self, context: &mut Context, event: CommunicationEvent) -> Result<(), QueueSendError>{
        let mut nsender: Sender<CommunicationEvent> = self.sender.clone();
        let r = context.get_runtime().block_on(async move{
            nsender.send(event).await
        });
        if r.is_ok(){
            Ok(())
        }else{
            Err(QueueSendError)
        }
    }

    /// Create a new broker future
    ///
    /// # Arguments
    /// * `handle` - JoinHandle provided in tokio 0.2.2+
    /// * `sender` - The `tokio::sync::mpsc::Sender` whose receiver belongs to the future
    /// * `receiver` - The `tokio::sync::mpsc::Receiver` whose sender is in the future
    pub fn new(handle: JoinHandle<()>, sender: Sender<CommunicationEvent>, receiver: Receiver<CommunicationEvent>) -> BrokerFuture{
        BrokerFuture{
            handle: handle,
            sender: sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}
