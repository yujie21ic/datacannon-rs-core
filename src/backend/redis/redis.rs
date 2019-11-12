/*
Redis information handler

Author Andrew Evans
*/


use crate::backend::backend_trait::Backend;
use crate::backend::config::BackendConfig;
use crate::security::ssl::SSLConfig;


#[derive(Clone, Debug)]
pub struct RedisHandler{
    pub backend_inf: BackendConfig,
    pub max_connections: usize,
    pub ssl_config: Option<SSLConfig>,
}


impl Backend for RedisHandler {

    fn get(&self){

    }

    fn send_result(&self) {

    }
}


impl RedisHandler {

    fn new() {

    }
}
