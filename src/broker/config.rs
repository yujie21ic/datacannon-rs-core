/*
Broker configuration

Author Andrew Evans
*/

use crate::connection::pool::Pool;
use crate::broker::queues::Queues;
use crate::router::router::Router;
use std::collections::HashMap;


/// Broker configuration
pub struct BrokerConfig{
    conn_pool: Pool,
    queues: Option<Queues>,
    routers: Option<HashMap<String, Router>>,
}


/// broker configuration implementation
impl BrokerConfig{

    fn new(conn_pool: Pool, queues: Option<Queues>, routers: Option<HashMap<String, Router>>) -> BrokerConfig {
        BrokerConfig{
            conn_pool: conn_pool,
            queues: queues,
            routers: routers,
        }
    }
}
