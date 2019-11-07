/*
RPC backend handler

Author Andrew Evans
*/

use crate::amqp::amqp::AMQPConnectionInf;
use crate::security::ssl::SSLConfig;
use crate::security::uaa::UAAConfig;
use crate::backend::backend_trait::Backend;


#[derive(Clone, Debug)]
pub struct RPCHandler{
    pub conn_inf: AMQPConnectionInf,
    pub exchange: Option<String>,
    pub exchange_type: Option<String>,
    pub persistent: bool,
    pub serializer: Option<String>,
    pub auto_delete: bool,
    pub ssl_config: Option<SSLConfig>,
    pub uaa_config: Option<UAAConfig>,
}


/// trait implementation
impl Backend for RPCHandler {

    /// get the result blocking as necessary
    fn get(&self){

    }

    /// send a result to the backend
    fn send_result(&self){

    }
}


/// contains the constructor and unique functions
impl RPCHandler{

    fn new() -> RPCHandler{

    }
}
