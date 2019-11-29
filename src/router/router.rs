//! Router for tasks. The router maintains a key and a list of available queues or topics.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::HashMap;
use crate::message_structure::queues::GenericQueue;


/// Router storing keys and queues
///
/// # Arguments
///
/// * `routing_key` - The routing key for the queue
/// * `queue` - Queue for the router
#[derive(Clone, Debug)]
pub struct Router{
    routing_key: String,
    queues: Vec<GenericQueue>,
    exchange: String,
}


/// Stores `Router` structures
///
/// # Arguments
/// * `routers` - A vector containing `Router` objects
#[derive(Clone, Debug)]
pub struct Routers{
    routers: HashMap<String, Router>,
}

/// Router Implementation
impl Router{

    /// Create a new router
    ///
    /// # Arguments
    /// * `routing_key` - The routing key for the queue
    /// * `queues` - Vector
    /// * `exchange` - Name of the exchange
    pub fn new( routing_key: String, queues: Vec<GenericQueue>, exchange: String) -> Router{
        Router{
            routing_key: routing_key,
            queues: queues,
            exchange: exchange,
        }
    }
}


/// Router storage implementation
impl Routers{

    /// Adds a router to the map
    pub fn add_router(&mut self, routing_key: String, router: Router) {
        self.routers.insert(routing_key, router);
    }

    /// Filter routers to create a subset of `Vec<Router>` with exact matching
    ///
    /// # Arguments
    ///
    /// * `filter` - exact matching pattern
    pub fn filter_routers(&self, filter: String) -> Option<&Router>{
        self.routers.get(&filter)
    }

    /// Filter routers to create a subset of `Vec<Router>` with regular expressions
    ///
    /// # Arguments
    /// * `filter` - Regular expresion filter
    pub fn match_routers(&self, filter: &str) -> Vec<Option<&Router>>{
        let re = regex::Regex::new(filter).unwrap();
        let mut routers = Vec::<Option<&Router>>::new();
        let k = self.routers.keys();
        k.filter(|k| re.is_match(k)).for_each(|k|{
            let mut val = self.routers.get(k).clone();
            routers.push(val);
        });
        routers
    }

    pub fn new() -> Routers {
        let router_map = HashMap::<String, Router>::new();
        Routers{
            routers: router_map,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::router::router::Router;
    use crate::message_structure::amqp::queue::AMQPQueue;
    use crate::replication::replication::HAPolicy;
    use crate::replication::rabbitmq::{RabbitMQHAPolicy, RabbitMQHAPolicies};
    use crate::connection::amqp::connection_inf::AMQPConnectionInf;

    fn get_router() -> Router {
        let ha_policy = RabbitMQHAPolicy::new(RabbitMQHAPolicies::ALL, 1);
        let exch = Some("test_exchange".to_string());
        let rkey = Some("test_routing_key".to_string());
        let max_priority = Some(1);
        let protocol = "amqp".to_string();
        let host = "127.0.0.1".to_string();
        let port = 5672;
        let vhost = Some("test".to_string());
        let conn_inf = AMQPConnectionInf::new(
            protocol,
            host,
            port,
            vhost,
        Some("dev".to_string()),
        Some("rtp*4500".to_string()),
        false,
        None,
        None,
        10000);
        let q = AMQPQueue::new("test_queue".to_string(), exch.clone(), rkey.clone(), max_priority.clone().unwrap(), HAPolicy::RabbitMQ(ha_policy), true, conn_inf);
        let mut qs = Vec::<GenericQueue>::new();
        qs.push(GenericQueue::AMQPQueue(q));
        Router::new(rkey.clone().unwrap(), qs, exch.clone().unwrap())
    }

    #[test]
    fn should_add_router(){
        let mut routers = Routers::new();
        let router = get_router();
        routers.add_router(router.routing_key.clone(), router);
        assert!(routers.routers.len() == 1);
    }

    #[test]
    fn should_add_same_router(){
        let mut routers = Routers::new();
        let router = get_router();
        let routerb = router.clone();
        routers.add_router(router.routing_key.clone(), router);
        routers.add_router(routerb.routing_key.clone(), routerb);
        assert!(routers.routers.len() == 1);
    }

    #[test]
    fn should_filter_routers(){
        let mut routers = Routers::new();
        let router = get_router();
        routers.add_router(router.routing_key.clone(), router.clone());
        let v = routers.filter_routers(router.routing_key.clone());
        assert!(v.is_some());
        assert!(v.unwrap().routing_key.eq(&router.routing_key.clone()));
    }

    #[test]
    fn should_match_routers(){
        let mut routers = Routers::new();
        let router = get_router();
        routers.add_router(router.routing_key.clone(), router.clone());
        let rex= "test_routing*".to_string();
        let v = routers.match_routers(&rex.clone());
        assert!(v.len() == 1);
        assert!(v.get(0).unwrap().unwrap().routing_key.eq(&router.routing_key.clone()));
    }
}
