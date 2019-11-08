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
}


/// broker configuration implementation
impl BrokerConfig{

    fn new(conn_pool: Pool, queues: Queues) -> BrokerConfig {
        BrokerConfig{
            conn_pool: conn_pool,
            queues: queues,
        }
    }
}
