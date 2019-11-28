//! For creating and maintaining connection information
//!
//! ---
//! author: Andrew Evans
//! ---

use lapin::Connection;
use tokio::runtime::Runtime;
use crate::connection::amqp::rabbitmq_connection_utils;
use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::error::connection_failed::ConnectionFailed;
use crate::connection::connection::ConnectionConfig::RabbitMQ;


/// Connection object containing a connection and a channel
pub struct RabbitMQConnection{
    pub connection: Connection,
}


///RabbitMQ connection implmentation
impl RabbitMQConnection{

    /// Create and return a new `crate::connection::amqp::rabbitmq_connection::RabbitMQConnection`
    ///
    /// # Arguments
    /// * `conn_inf` - Relevant `crate::connection::amqp::AMQPConnectionInf`
    /// * `runtime` - The application `tokio::runtime::Runtime`
    pub fn new(conn_inf: &AMQPConnectionInf, runtime: &mut Runtime) -> Result<RabbitMQConnection, ConnectionFailed>{
        let conn_res = rabbitmq_connection_utils::get_connection(conn_inf, runtime);
        if conn_res.is_ok(){
            let conn = conn_res.ok().unwrap();
            let rmq = RabbitMQConnection{
                connection: conn,
            };
            Ok(rmq)
        }else{
            Err(ConnectionFailed)
        }
    }
}


#[cfg(test)]
pub mod test{
    use super::*;
    use crate::security::ssl::SSLConfig;
    use crate::connection::amqp::connection_inf::AMQPConnectionInf;

    #[test]
    fn should_create_connection(){
        let ssl_conf = SSLConfig::new(None, false, "".to_string(), 5000,None);
        let amq_conf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 5672, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), false, Some(ssl_conf),None,5000);
        let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
        let conn = RabbitMQConnection::new(&amq_conf, &mut rt);
        assert!(conn.is_ok());
        let closed = rt.block_on(async move{
            conn.unwrap().connection.close(200, "").await
        });
        assert!(closed.is_ok())
    }

    #[test]
    fn should_create_tls_connection(){
        unimplemented!()
    }

    #[test]
    fn should_return_error_on_failure(){
        let ssl_conf = SSLConfig::new(None, false, "".to_string(), 5000,None);
        let amq_conf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 5673, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), false, Some(ssl_conf),None,5000);
        let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
        let conn = RabbitMQConnection::new(&amq_conf, &mut rt);
        assert!(conn.is_err());
        let cerr = conn.err().unwrap();
        if let ConnectionFailed = cerr {
            assert!(true);
        }else{
            assert!(false);
        }
    }
}