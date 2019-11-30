//! RabbitMQ Connection Pool
//!
//! ---
//! author: Andrew Evans
//! ---


use lapin::Channel;
use tokio::runtime::Runtime;

use crate::config::config::CannonConfig;
use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::connection::amqp::rabbitmq_connection::RabbitMQConnection;
use crate::connection::connection::ConnectionConfig;
use crate::connection::pool_trait::Pool;
use crate::error::pool_creation_error::PoolCreationError;
use crate::error::channel_creation_failed::ChannelCreationError;
use crate::app::context::Context;


/// RabbitMQ Connection Pool
pub struct RabbitMQConnectionPool{
    connections: Vec<RabbitMQConnection>,
    connection_config: AMQPConnectionInf,
    min_connections: usize,
    current_connection: usize,
}

/// Implementation of the pool
impl Pool for RabbitMQConnectionPool{

    /// Drop all connections
    fn drop_connections(&mut self, context: &mut Context) {
        for i in 0..self.connections.len(){
            let conn = self.connections.get(i).unwrap();
            let r = context.get_runtime().block_on( async move {
                conn.connection.close(200, "Complete").await
            });
        }
        self.connections = vec![];
    }

    /// Close the pool
    fn close(&mut self, context: &mut Context) {
        self.drop_connections(context);
    }
}


/// Implementation of the RabbitMQ Connection Pool
impl RabbitMQConnectionPool{

    /// Get the size of the pool in `std::usize`
    pub fn get_pool_size(&mut self) -> usize{
        self.connections.len()
    }

    /// Get the channel
    pub fn get_channel(&mut self, context: &mut Context) -> Result<Channel, ChannelCreationError>{
        let c: &RabbitMQConnection = self.connections.get(self.current_connection).unwrap();
        let ch: Result<Channel, ChannelCreationError> = context.get_runtime().block_on(async move {
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
    /// * `context` - Contains the runtime and runtime meta for the application
    /// * `is_broker` - Whether the connection pool belongs to a broker
    pub fn new(cannon_config: &CannonConfig, context: &mut Context, is_broker: bool) -> Result<RabbitMQConnectionPool, PoolCreationError> {
        let conn_inf = cannon_config.connection_inf.clone();
        let mut conn_vec = Vec::<RabbitMQConnection>::new();
        let mut num_conn = cannon_config.num_broker_connections;
        if is_broker == false{
            num_conn = cannon_config.num_backend_connections;
        }
        let conn_inf = cannon_config.connection_inf.clone();
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            for i in 0..num_conn {
                let c = RabbitMQConnection::new(&conn_inf, context.get_runtime());
                if c.is_ok(){
                    conn_vec.push(c.ok().unwrap());
                }
            }
            if conn_vec.len() == num_conn {
                let rmq = RabbitMQConnectionPool {
                    connections: conn_vec,
                    connection_config: conn_inf,
                    min_connections: num_conn,
                    current_connection: 0,
                };
                Ok(rmq)
            }else{
                Err(PoolCreationError)
            }
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

    fn get_rmq_pool_for_fail(context: &mut Context) -> Result<RabbitMQConnectionPool, PoolCreationError>{
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
        let rmq_pool = RabbitMQConnectionPool::new(&cannon_conf, context, true);
        rmq_pool
    }

    fn do_get_rmq_pool(ctx: &mut Context) -> Result<RabbitMQConnectionPool, PoolCreationError>{
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
        let rmq_pool = RabbitMQConnectionPool::new(&cannon_conf, ctx, true);
        rmq_pool
    }

    fn get_rmq_pool() -> Result<RabbitMQConnectionPool, PoolCreationError> {
        let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
        let mut ctx = Context::new(rt);
        do_get_rmq_pool(&mut ctx)
    }

    #[test]
    fn should_create_connection_pool(){
        let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
        let mut context = Context::new(rt);
        let pool_res = do_get_rmq_pool(&mut context);
        assert!(pool_res.is_ok());
        pool_res.unwrap().close(&mut context);
    }

    #[test]
    fn should_setup_pool(){
        let p = panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            let mut context = Context::new(rt);
            let pool_res = do_get_rmq_pool(&mut context);
            assert!(pool_res.is_ok());
            let mut pool = pool_res.ok().unwrap();
            pool.close(&mut context);
        });
        assert!(p.is_ok());
    }

    #[test]
    fn should_close_pool(){
        let p = panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            let mut context = Context::new(rt);
            let pool_res = do_get_rmq_pool(&mut context);
            assert!(pool_res.is_ok());
            let mut pool = pool_res.ok().unwrap();
            pool.close(&mut context);
            assert!(pool.get_pool_size() == 0);
        });
        assert!(p.is_ok());
    }

    #[test]
    fn should_get_channel(){
        let p = panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            let mut context = Context::new(rt);
            let pool_res = do_get_rmq_pool(&mut context);
            assert!(pool_res.is_ok());
            let mut pool = pool_res.ok().unwrap();

            let ch = pool.get_channel(&mut context);
            pool.close(&mut context);
            assert!(ch.is_ok());
            assert!(pool.get_pool_size() == 0);
        });
        assert!(p.is_ok());
    }

    #[test]
    fn should_fail_gracefully(){
        let p = panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            let mut context = Context::new(rt);
            let pool_res = get_rmq_pool_for_fail(&mut context);
            assert!(pool_res.is_ok());
            let mut pool = pool_res.ok().unwrap();
            pool.close(&mut context);
            assert!(pool.get_pool_size() == 0);
        });
        assert!(p.is_ok());
    }
}
