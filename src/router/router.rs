//! Router for tasks. The router maintains a key and a list of available queues or topics.
//!
//! ---
//! author: Andrew Evans
//! ---

use regex::Regex;
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
    is_regex: bool,
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
    /// * `is_regex` - Whether to use direct matching or regex
    pub fn new( routing_key: String, queues: Vec<GenericQueue>, exchange: String, is_regex: bool) -> Router{
        Router{
            routing_key: routing_key,
            queues: queues,
            exchange: exchange,
            is_regex: is_regex
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
    pub fn filter_routers(&self, filter: String) -> &Router{
        self.routers.get(&filter).unwrap()
    }

    /// Filter routers to create a subset of `Vec<Router>` with regular expressions
    ///
    /// # Arguments
    /// * `filter` - Regular expresion filter
    pub fn match_routers(&self, filter: &str) -> Vec<&Router>{
        let re = regex::Regex::new(filter).unwrap();
        let mut routers = Vec::<&Router>::new();
        let k = self.routers.keys();
        k.filter(|k| re.is_match(k)).for_each(|k|{
            let mut val = self.routers.get(k).clone().unwrap();
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

    /*
    fn get_router() -> Router{
        routing_key: routing_key,
        queues: queues,
        exchange: exchange,
        is_regex: is_regex
        let qs = Queues::
        //let router = Router::new(
        //   "test_key".to_string(), )
    }
    */

    #[test]
    fn should_add_router(){

    }

    #[test]
    fn should_add_same_router(){

    }

    #[test]
    fn should_filter_routers(){

    }

    #[test]
    fn should_match_routers(){

    }
}
