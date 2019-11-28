//! RabbitMQ Connection Pool
//!
//! ---
//! author: Andrew Evans
//! ---


use lapin::{Channel, Connection};
use tokio::runtime::Runtime;

use crate::config::config::CannonConfig;
use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::connection::amqp::rabbitmq_connection::RabbitMQConnection;
use crate::connection::connection::ConnectionConfig;
use crate::connection::pool_trait::Pool;
use crate::error::pool_creation_error::PoolCreationError;
use crate::error::channel_creation_failed::ChannelCreationError;


/// RabbitMQ Connection Pool
pub struct RabbitMQConnectionPool<'a>{
    connections: Vec<RabbitMQConnection>,
    runtime: &'a mut Runtime,
    connection_config: AMQPConnectionInf,
    min_connections: usize,
    current_connection: usize,
}

/// Implementation of the pool
impl <'a> Pool for RabbitMQConnectionPool<'a>{

    /// Setup the pool. Will fail if the `number_broker_connections` cannot be made.
    fn setup(&mut self) -> Result<(), PoolCreationError>{
        let mut left = self.min_connections - self.connections.len();
        let mut i = 0;
        while i < 3 && left > 0 {
            for i in 0..left {
                let conn_result = RabbitMQConnection::new(&self.connection_config, self.runtime);
                if conn_result.is_ok() {
                    let conn = conn_result.unwrap();
                    self.connections.push(conn);
                }else{
                    println!("Connection Failed");
                }
            }
            left = self.min_connections - self.connections.len();
            i += 1;
        }
        if self.connections.len() == self.min_connections{
            Ok(())
        }else{
            Err(PoolCreationError)
        }
    }

    /// Drop all connections
    fn drop_connections(&mut self) {
        for i in 0..self.connections.len(){
            let conn = self.connections.get(i).unwrap();
            self.runtime.block_on( async move {
                conn.connection.close(200, "Complete").await;
            });
        }
        self.connections = vec![];
    }

    /// Close the pool
    fn close(&mut self) {
        self.drop_connections();
    }
}


/// Implementation of the RabbitMQ Connection Pool
impl <'a> RabbitMQConnectionPool<'a>{

    /// Get the size of the pool in `std::usize`
    pub fn get_pool_size(&mut self) -> usize{
        self.connections.len()
    }

    /// Get the channel
    pub fn get_channel(&mut self) -> Result<Channel, ChannelCreationError>{
        let c: &RabbitMQConnection = self.connections.get(self.current_connection).unwrap();
        let ch: Result<Channel, ChannelCreationError> = self.runtime.block_on(async move {
            let channel_result = c.connection.create_channel().await;
            if channel_result.is_ok() {
                Ok(channel_result.unwrap())
            } else {
                Err(ChannelCreationError)
            }
        });
        self.current_connection = self.current_connection + 1;
        if self.current_connection >= self.connections.len() {
            self.current_connection = 0;
        }
        ch
    }

    /// Create a new RabbitMQ Connection Pool. Setup creates the connections
    ///
    /// # Arguments
    /// * `cannon_config` - The application `crate::config::config::CannonConfig`
    /// * `runtime` - The application `tokio::runtime::Runtime`
    pub fn new(cannon_config: &CannonConfig, runtime: &'a mut Runtime) -> Result<RabbitMQConnectionPool<'a>, PoolCreationError> {
        let conn_inf = cannon_config.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let rmq = RabbitMQConnectionPool {
                connections: Vec::<RabbitMQConnection>::new(),
                runtime,
                connection_config: conn_inf,
                min_connections: cannon_config.num_broker_connections,
                current_connection: 0,
            };
            Ok(rmq)
        }else{
            Err(PoolCreationError)
        }
    }
}


#[cfg(test)]
pub mod test{
    use std::panic;

    use crate::backend::config::BackendConfig;
    use crate::config::config::CannonConfig;
    use crate::connection::amqp::connection_inf::AMQPConnectionInf;
    use crate::connection::amqp::rabbit_mq_connection_pool::RabbitMQConnectionPool;
    use crate::connection::connection::ConnectionConfig;
    use crate::connection::pool_trait::Pool;
    use crate::error::pool_creation_error::PoolCreationError;
    use crate::router::router::Routers;
    use crate::security::ssl::SSLConfig;

    use super::*;

    fn get_rmq_pool_for_fail<'a>(rt: &'a mut Runtime) -> Result<RabbitMQConnectionPool<'a>, PoolCreationError>{
        let ssl_conf = SSLConfig::new(None, false, "".to_string(), 5000,None);
        let amq_conf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 5272, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), false, Some(ssl_conf),None,1000);
        let conn_conf = ConnectionConfig::RabbitMQ(amq_conf);
        let backend_conf = BackendConfig{
            url: "".to_string(),
            username: None,
            password: None,
            transport_options: None
        };
        let routers = Routers::new();
        let mut cannon_conf = CannonConfig::new(conn_conf,backend_conf, routers);
        cannon_conf.num_broker_connections = 1;
        let rmq_pool = RabbitMQConnectionPool::new(&cannon_conf, rt);
        rmq_pool
    }

    fn get_rmq_pool<'a>(rt: &'a mut Runtime) -> Result<RabbitMQConnectionPool<'a>, PoolCreationError>{
        let ssl_conf = SSLConfig::new(None, false, "".to_string(), 5000,None);
        let amq_conf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 5672, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), false, Some(ssl_conf),None,5000);
        let conn_conf = ConnectionConfig::RabbitMQ(amq_conf);
        let backend_conf = BackendConfig{
            url: "".to_string(),
            username: None,
            password: None,
            transport_options: None
        };
        let routers = Routers::new();
        let mut cannon_conf = CannonConfig::new(conn_conf,backend_conf, routers);
        cannon_conf.num_broker_connections = 2;
        let rmq_pool = RabbitMQConnectionPool::new(&cannon_conf, rt);
        rmq_pool
    }

    #[test]
    fn should_create_connection_pool(){
        let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
        let pool_res = get_rmq_pool(&mut rt);
        assert!(pool_res.is_ok());
        pool_res.unwrap().close();
    }

    #[test]
    fn should_setup_pool(){
        let p = panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            let pool_res = get_rmq_pool(&mut rt);
            assert!(pool_res.is_ok());
            let mut pool = pool_res.ok().unwrap();
            let ps = pool.setup();
            if ps.is_ok() {
                pool.close();
            }else{
                panic!("Pool Failed");
            }
        });
        assert!(p.is_ok());
    }

    #[test]
    fn should_close_pool(){
        let p = panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            let pool_res = get_rmq_pool(&mut rt);
            assert!(pool_res.is_ok());
            let mut pool = pool_res.ok().unwrap();
            let is_setup = pool.setup();
            assert!(is_setup.is_ok());
            pool.close();
            assert!(pool.get_pool_size() == 0);
        });
        assert!(p.is_ok());
    }

    #[test]
    fn should_get_channel(){
        let p = panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            let pool_res = get_rmq_pool(&mut rt);
            assert!(pool_res.is_ok());
            let mut pool = pool_res.ok().unwrap();
            let is_setup = pool.setup();
            let setup_panic = panic::catch_unwind(||{
                assert!(is_setup.is_ok());
            });
            let ch = pool.get_channel();
            pool.close();
            assert!(ch.is_ok());
            assert!(setup_panic.is_ok());
            assert!(pool.get_pool_size() == 0);
        });
        assert!(p.is_ok());
    }

    #[test]
    fn should_fail_gracefully(){
        let p = panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            let pool_res = get_rmq_pool_for_fail(&mut rt);
            assert!(pool_res.is_ok());
            let mut pool = pool_res.ok().unwrap();
            let is_setup = pool.setup();
            let setup_panic =panic::catch_unwind(||{
                assert!(is_setup.is_err());
            });
            pool.close();
            assert!(setup_panic.is_ok());
            assert!(pool.get_pool_size() == 0);
        });
        assert!(p.is_ok());
    }
}
