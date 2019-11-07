/*
Broker configuration

Author Andrew Evans
*/

use crate::connection::pool::Pool;
use crate::broker::queues::Queues;


/// Broker configuration
pub struct BrokerConfig{
    conn_pool: Pool,
    queues: Queues,
    routers:
}


/// broker configuration implementation
impl BrokerConfig{

    fn new(conn_pool: Pool) -> BrokerConfig{

    }
}
