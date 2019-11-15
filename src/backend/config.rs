//! Configuration for backends
//!
//! ---
//! author: Andrew Evans
//!

use crate::backend::rabbiitmq::handler::RabbitMQBackendHandler;
use crate::backend::redis::redis::RedisHandler;


/// Supported backend types with classes
#[derive(Clone, Debug)]
pub enum AvailableBackend{
    RabbitMQBackend(RabbitMQBackendHandler),
    RedisBackend(RedisHandler),
    Void,
}


#[derive(Clone, Debug)]
pub struct BackendConfig {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub transport_options: Option<String>,
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::backend::config::BackendConfig;
    use crate::backend::config::AvailableBackend::RedisBackend;

    #[test]
    fn should_obtain_accessible_backend(){
        let bc = BackendConfig{
            url: "test".to_string(),
            username: None,
            password: None,
            transport_options: None,
        };
        let rdh = RedisHandler{
            backend_inf: bc,
            max_connections: 2,
            ssl_config: None,
        };
        let be = AvailableBackend::RedisBackend(rdh);
        match be{
            AvailableBackend::RedisBackend(be) =>{
                assert_eq!(be.max_connections, 2);
            }
            AvailableBackend::RabbitMQBackend(be) =>{
                assert!(false)
            }
            AvailableBackend::Void =>{
                assert!(false);
            }
        }
    }
}