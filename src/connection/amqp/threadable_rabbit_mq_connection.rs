//! Threadable rabbit mq connection
//!
//! ---
//! author: Andrew Evans
//! ---


use amiquip::Connection;

use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::error::connection_failed::ConnectionFailed;


/// A threadable rabbitmq connection
pub struct ThreadableRabbitMQConnection{
    pub connection: Connection,
}


/// Implementation of the connection
impl ThreadableRabbitMQConnection {

    /// Create a new connection or return a connection failed error
    ///
    /// # Arguments
    /// * `url` - The connection url
    /// * `conn_inf` - A reference to the `crate::connection::amqp::connection_inf::AMQPConnectionInf`
    pub fn new(url: String, conn_inf: &AMQPConnectionInf) -> Result<ThreadableRabbitMQConnection, ConnectionFailed> {
        let conn_result = Connection::insecure_open(url.as_str());
        if (conn_result.is_ok()) {
            let mut conn = conn_result.unwrap();
            let channel_result = conn.open_channel(None);
            if (channel_result.is_ok()) {
                let channel = channel_result.unwrap();
                let conn_object = ThreadableRabbitMQConnection {
                    connection: conn,
                };
                Ok(conn_object)
            } else {
                match channel_result {
                    Err(e) => {
                        println!("{}", e);
                    }
                    _ => {}
                }
                Err(ConnectionFailed)
            }
        } else {
            match conn_result {
                Err(e) => {
                    println!("{}", e);
                }
                _ => {}
            }
            Err(ConnectionFailed)
        }
    }
}
