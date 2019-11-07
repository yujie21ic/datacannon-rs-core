/// Handles a connection to rabbit mq.
/// Author: Andrew Evans
use amiquip::{Channel, Connection, Exchange, Publish, Result, ConnectionOptions};
use std::sync::{Arc, Mutex};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use crate::amqp::amqp::AMQPConnectionInf;


/// Connection object
pub struct RabbitMQConnection{
    pub connection: Connection,
    pub channel: Channel,
}


/// Implementation
impl RabbitMQConnection {

    /// Create the new connection
    pub fn new(url: String, conn_inf: AMQPConnectionInf) -> Result<RabbitMQConnection, &'static str> {
        let conn_result = Connection::insecure_open(url.as_str());
        if conn_inf.is_ssl() {
            let copts = ConnectionOptions::default();
            Connection::open_tls_stream()
        }
        if(conn_result.is_ok()){
            let mut conn = conn_result.unwrap();
            let channel_result = conn.open_channel(None);
            if(channel_result.is_ok()) {
                let channel = channel_result.unwrap();
                let conn_object = RabbitMQConnection {
                    connection: conn,
                    channel: channel,
                };
                Ok(conn_object)
            }else{
                match channel_result{
                    Err(e) =>{
                        println!("{}", e);
                    }
                    _ => {}
                }
                Err("Failed to Establish a Channel")
            }
        }else {
            match conn_result{
                Err(e) =>{
                    println!("{}", e);
                }
                _ => {}
            }
            Err("Failed to Establish a Connection")
        }
    }
}
