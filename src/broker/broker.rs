/*
Enumeration for storing a broker type

Author Andrew Evans
*/

use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::argparse::argtype::ArgType;
use crate::connection::pool::Pool;
use crate::message_protocol::message::Message;
use tokio::sync::mpsc::Sender;


/// An enumeration of available brokers
pub enum AvailableBroker{
    RabbitMQ(RabbitMQBroker),
}


/// the broker
pub trait Broker{
    /// start the broker
    fn setup(&mut self, rt: tokio::runtime::Runtime);

    /// tear down the broker
    fn teardown(&mut self);

    /// close the broker which should call teardown
    fn close(&mut self);

    /// send a task
    fn send_task(&mut self, task: String, args: Vec<ArgType>,  app_sender: Sender<Message>);
}
