//! Connection Pool for RabbitMQ connection pooling. Requires the threadable struct since you cannot share utilities
//!
//! ---
//! author:  Andrew Evans
//! ---

use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use std::sync::{Arc, Mutex, RwLock};
use std::vec::Vec;

use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::connection::amqp::rabbitmq_connection_factory::{Credential, RabbitMQConnectionFactory};
use crate::connection::amqp::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;
use crate::error::pool_errors::PoolIsEmptyError;
use crate::error::connection_failed::ConnectionFailed;


/// Structure storing the pool
///
/// # Arguments
/// * `initial_size`- The initial size of the connection pool
/// * `current_size` - Current size of the connection pool
/// * `conn_inf` - relevant connection information
/// * `conn_factory` - The `crate::connection::amqp::rabbitmq_connection_factory::RabbitMQConnectionFactory`
/// * `active_connections` - Currently active `std::vec::Vec<crate::connection::amqp::threadable_rabbitmq_connection::ThreadableRabbitMQConnection>`
pub struct ThreadableRabbitMQConnectionPool{
    pub initial_size: usize,
    pub current_size: usize,
    conn_inf: AMQPConnectionInf,
    conn_factory: RabbitMQConnectionFactory,
    active_connections: Vec<ThreadableRabbitMQConnection>,
}


/// Structure implementation
impl ThreadableRabbitMQConnectionPool{

    /// Get the current pool size `std::usize`
    pub fn get_current_pool_size(&self) -> usize{
        self.current_size.clone()
    }

    /// Release the connection
    ///
    /// # Arguments
    /// * `conn` - The relevant `crate::connection::amqp::threadable_rabbitmq_connection::ThreadableRabbitMQConnection`
    fn release_connection(&mut self, conn: ThreadableRabbitMQConnection){
        self.active_connections.push(conn);
        self.current_size += 1;
    }

    /// Get a threadable connection from the pool or return a `crate::connection::pool_error::PoolIsEmptyError`
    pub fn get_connection(&mut self) -> Result<ThreadableRabbitMQConnection, PoolIsEmptyError>{
        if self.active_connections.is_empty(){
            Err(PoolIsEmptyError)
        }else{
            let conn = self.active_connections.pop().unwrap();
            self.current_size -= 1;
            Ok(conn)
        }
    }

    /// Create a connection for the pool. Obtain a threadable connection or return an Error
    pub fn create_connection(&mut self) -> Result<ThreadableRabbitMQConnection, ConnectionFailed>{
        self.conn_factory.create_threadable_connection()
    }

    /// Add a connection
    pub fn add_connection(&mut self){
        let rconn = self.create_connection();
        if rconn.is_ok(){
            let conn = rconn.ok().unwrap();
            self.active_connections.push(conn);
            self.current_size += 1;
        }
    }

    /// Start the connection
    pub fn start(&mut self) {
        if self.active_connections.len() == 0{
            for i in 0..self.initial_size{
                self.add_connection();
            }
        }
    }

    /// close the pool
    pub fn close_pool(&mut self){
        for i in 0 .. self.active_connections.len(){
            let conn = self.active_connections.pop();
            conn.unwrap().connection.close();
        }
    }

    /// Create a new connection pool
    pub fn new(conn_inf: &mut AMQPConnectionInf, min_connections: usize) -> ThreadableRabbitMQConnectionPool{
        let factory = RabbitMQConnectionFactory::new(conn_inf.clone());
        let active_connections: Vec<ThreadableRabbitMQConnection> = Vec::<ThreadableRabbitMQConnection>::new();
        ThreadableRabbitMQConnectionPool{
            initial_size: min_connections,
            current_size: 0,
            conn_inf: conn_inf.clone(),
            conn_factory: factory,
            active_connections: active_connections,
        }
    }
}


#[cfg(test)]
mod tests{
    use std::borrow::BorrowMut;
    use std::sync::{LockResult, Mutex, PoisonError};
    use std::thread;

    use amiquip::{ExchangeDeclareOptions, ExchangeType, FieldTable, Publish, QueueDeclareOptions, QueueDeleteOptions, Result};

    use super::*;

    fn get_amqp_conn_inf() -> AMQPConnectionInf {
        AMQPConnectionInf::new(
            String::from("amqp"),
            String::from("127.0.0.1"),
            5672,
            Some(String::from("test")),
            Some(String::from("dev")),
            Some(String::from("rtp*4500")),
            false,
            None,
            None
        )
    }

    #[test]
    fn should_start_and_close_pool(){
        let conn_inf = get_amqp_conn_inf();
        let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 3);
        println!("Starting pool");
        pool.start();
        assert!(pool.current_size == 3);
        assert!(pool.active_connections.len() == 3);
        println!("Closing Pool");
        pool.close_pool();
    }

    #[test]
    fn should_add_more_connections(){
        let conn_inf = get_amqp_conn_inf();
        let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 3);
        pool.start();
        assert!(pool.current_size == 3);
        assert!(pool.active_connections.len() == 3);
        pool.add_connection();
        assert!(pool.current_size == 4);
        assert!(pool.active_connections.len() == 4);
        pool.close_pool();
    }

    #[test]
    fn should_get_connection(){
        let conn_inf = get_amqp_conn_inf();
        let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 3);
        pool.start();
        let conn = pool.get_connection().unwrap();
        conn.connection.close();
        assert!(pool.current_size == 2);
        assert!(pool.active_connections.len() == 2);
        pool.close_pool();
    }

    #[test]
    fn should_release_connection(){
        let conn_inf = get_amqp_conn_inf();
        let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 3);
        pool.start();
        let conn = pool.get_connection().unwrap();
        assert!(pool.current_size == 2);
        assert!(pool.active_connections.len() == 2);
        pool.release_connection(conn);
        assert!(pool.current_size == 3);
        assert!(pool.active_connections.len() == 3);
        pool.close_pool();
    }

    #[test]
    fn should_perform_function_in_a_thread() -> Result<()>{
        let conn_inf = get_amqp_conn_inf();
        let mut pool = ThreadableRabbitMQConnectionPool::new(&mut conn_inf.clone(), 3);
        pool.start();
        let mut conn = pool.get_connection().unwrap();
        let channel = conn.connection.open_channel(None)?;
        let _t = thread::spawn(move || ->Result<()> {
            let queue = channel.queue_declare("hello",QueueDeclareOptions::default());
            queue.unwrap().delete(QueueDeleteOptions::default());
            Ok(())
        });
        _t.join();
        pool.release_connection(conn);
        pool.close_pool();
        Ok(())
    }
}
