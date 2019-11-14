/// A rabbit mq connection factory for obtaining rabbit mq based connections.
/// Author: Andrew Evans
use std::fmt;
use std::sync::{Arc, Mutex};

use amiquip::{Channel, Connection};

use crate::connection::amqp::rabbitmq_connection_utils;
use crate::connection::amqp::rabbitmq_connection_utils::RabbitMQConnection;
use crate::connection::amqp::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::protocol_configs::amqp::AMQPConnectionInf;

///Credentials object
pub struct Credential{
    pub username: String,
    pub password: String,
}


///Overarching Rabbit MQ Connection Factory
pub struct RabbitMQConnectionFactory{
    conn_inf: AMQPConnectionInf,
}


///Implementation of the Rabbit MQ Connection Factory
impl RabbitMQConnectionFactory {

    /// Open a connection and channel. Return a connection object with blocking access
    fn create_connection_object(&self, url: String) -> Result<RabbitMQConnection, &'static str> {
        let cinf = &self.conn_inf;
        RabbitMQConnection::new(url, cinf)
    }

    /// create a threadable connection object
    fn create_threadable_connection_object(&self, url: String) -> Result<ThreadableRabbitMQConnection, &'static str> {
        let cinf = &self.conn_inf;
        ThreadableRabbitMQConnection::new(url, cinf)
    }

    /// Create a RabbitMQ Connection
    pub fn create_connection(&self, is_ssl: bool) -> Result<RabbitMQConnection, &'static str> {
        let url = self.conn_inf.to_url();
        self.create_connection_object(url)
    }


    /// Create a thread safe connection from the factory
    pub fn create_threadable_connection(&self) -> Result<ThreadableRabbitMQConnection, &'static str> {
        let url= self.conn_inf.to_url();
        self.create_threadable_connection_object(url)
    }

    /// Create a new object
    pub fn new(conn_inf: AMQPConnectionInf) -> RabbitMQConnectionFactory{
        RabbitMQConnectionFactory{
            conn_inf: conn_inf,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::ops::DerefMut;
    use std::thread;

    use crate::connection::amqp::rabbitmq_connection_utils;

    use super::*;

    #[test]
    fn should_create_new(){
        let aci = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 5672, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), false, None, None);
        let f = RabbitMQConnectionFactory::new(aci);
        let conn_object = f.create_connection(false).ok().unwrap();
        conn_object.channel.close();
        conn_object.connection.close();
    }
}
