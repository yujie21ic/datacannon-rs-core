/*
Broker configuration

Author Andrew Evans
*/

use crate::connection::pool::Pool;
use crate::router::router::Router;
use std::collections::HashMap;


/// Broker configuration
pub struct BrokerConfig{
    conn_pool: Pool,
}


/// broker configuration implementation
impl BrokerConfig{

    fn new(conn_pool: Pool) -> BrokerConfig {
        BrokerConfig{
            conn_pool: conn_pool,
        }
    }
}
