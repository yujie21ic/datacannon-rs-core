/*
Broker configuration

Author Andrew Evans
*/

use crate::connection::pool::Pool;


/// Broker configuration
pub struct BrokerConfig{
    conn_pool: Pool,
}


/// broker configuration implementation
impl BrokerConfig{

    fn new() -> BrokerConfig{

    }
}
