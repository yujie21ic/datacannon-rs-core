//! Trait for all connections and enum for storing generically.
//!
//! ---
//! author: Andrew Evans
//! ---


use crate::connection::kafka::connection_inf::KafkaConnectionInf;
use crate::connection::amqp::connection_inf::AMQPConnectionInf;


/// Trait all connections should implement
trait Connection{
    fn open(self);
    fn send(self);
    fn close(self);
}


/// Connection Type for generically storing connections
#[derive(Clone, Debug)]
pub enum ConnectionConfig{
    RabbitMQ(AMQPConnectionInf),
    Kafka(KafkaConnectionInf),
}
