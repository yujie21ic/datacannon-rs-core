//! A rabbit mq connection factory for obtaining rabbit mq based connections.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::fmt;
use std::sync::{Arc, Mutex};

use amiquip::{Channel, Connection};

use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::connection::amqp::rabbitmq_connection;
use crate::connection::amqp::rabbitmq_connection::RabbitMQConnection;
use crate::connection::amqp::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::error::connection_failed::ConnectionFailed;


/// Credentials object
///
/// # Arguments
/// * `username` - Username
/// * `password` - Password
pub struct Credential{
    pub username: String,
    pub password: String,
}


///Overarching Rabbit MQ Connection Factory
///
/// # Arguments
/// * `conn_inf` - The relevant `crate::connection::amqp::connection_inf::AMQPConnectionInf`
pub struct RabbitMQConnectionFactory{
    conn_inf: AMQPConnectionInf,
}


///Implementation of the Rabbit MQ Connection Factory
impl RabbitMQConnectionFactory {

    /// Open a connection and channel. Return a `crate::connection::amqp::RabbitMQConnection` or connection error
    ///
    /// # Arguments
    /// * `url` - Connetion url`
    fn create_connection_object(&self, url: String) -> Result<RabbitMQConnection, ConnectionFailed> {
        let cinf = &self.conn_inf;
        RabbitMQConnection::new(url, cinf)
    }

    /// Create a threadable connection object returning a `crate::connection::amqp::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection` or error
    ///
    /// # Arguments
    /// * `url` - The connection string
    fn create_threadable_connection_object(&self, url: String) -> Result<ThreadableRabbitMQConnection,  ConnectionFailed> {
        let cinf = &self.conn_inf;
        ThreadableRabbitMQConnection::new(url, cinf)
    }

    /// Create a RabbitMQ Connection returning a `crate::connection::amqp::RabbitMQConnection or error
    ///
    /// # Arguments
    /// * `is_ssl` - Whether to use ssl
    pub fn create_connection(&self, is_ssl: bool) -> Result<RabbitMQConnection,  ConnectionFailed> {
        let url = self.conn_inf.to_url();
        self.create_connection_object(url)
    }


    /// Create a threadable connection object returning a `crate::connection::amqp::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection` or error
    pub fn create_threadable_connection(&self) -> Result<ThreadableRabbitMQConnection,  ConnectionFailed> {
        let url= self.conn_inf.to_url();
        self.create_threadable_connection_object(url)
    }

    /// Create a new connection factory
    ///
    /// # Arguments
    /// * `conn_inf` - The relevant `crate::connection::amqp::connection_inf::AMQPConnectionInf`
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

    use crate::connection::amqp::rabbitmq_connection;

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
