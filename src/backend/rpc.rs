/*
RPC backend handler

Author Andrew Evans
*/

use crate::amqp::amqp::AMQPConnectionInf;
use crate::security::ssl::SSLConfig;
use crate::security::uaa::UAAConfig;
use crate::backend::backend_trait::Backend;


/// The RPC handler
#[derive(Clone, Debug)]
pub struct RPCHandler{
    conn_inf: AMQPConnectionInf,
    exchange: Option<String>,
    exchange_type: Option<String>,
    persistent: bool,
    serializer: Option<String>,
    auto_delete: bool,
    ssl_config: Option<SSLConfig>,
    uaa_config: Option<UAAConfig>,
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

    fn do_send(&self){

    }

    /// get the connection information
    fn get_conn_inf(&self) -> &AMQPConnectionInf{
        &self.conn_inf
    }

    /// whether the data persists
    fn is_persistent(&self) -> bool{
        self.persistent
    }

    /// whether to automatically delete
    fn is_auto_delete(&self) -> bool{
        self.auto_delete
    }

    /// obtain the ssl config option
    fn get_ssl_config(&self) -> Option<SSLConfig>{
        self.ssl_config.clone()
    }

    /// obtain the uaa configuration
    fn get_uaa_config(&self) -> Option<UAAConfig>{
        self.uaa_config.clone()
    }

    /// create a new handler
    fn new(
        conn_inf: AMQPConnectionInf,
        exchange: Option<String>,
        exchange_type: Option<String>,
        persistent: bool,
        serializer: Option<String>,
        auto_delete: bool,
        ssl_config: Option<SSLConfig>,
        uaa_config: Option<UAAConfig>) -> RPCHandler{
        RPCHandler{
            conn_inf: conn_inf,
            exchange: exchange,
            exchange_type: exchange_type,
            persistent: persistent,
            serializer: serializer,
            auto_delete: auto_delete,
            ssl_config: ssl_config,
            uaa_config: uaa_config,
        }
    }
}
