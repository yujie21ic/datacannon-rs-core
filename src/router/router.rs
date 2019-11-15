//! Router for tasks. The router maintains a key and a list of available queues or topics.
//!
//! ---
//! author: Andrew Evans
//! ---

use regex::Regex;

use crate::message_structure::queues::{Queue, Queues};
use std::collections::HashMap;


/// Router storing keys and queues
///
/// # Arguments
///
/// * `routing_key` - The routing key for the queue
/// * `queue` - Queue for the router
pub struct Router{
    routing_key: String,
    queues: Queues,
    exchange: String,
    is_regex: bool,
}


/// Stores `Router` structures
///
/// # Arguments
/// * `routers` - A vector containing `Router` objects
pub struct Routers{
    routers: HashMap<String, Router>,
}


/// Router implementation
impl Routers{

    /// Adds a router to the map
    pub fn add_router(&mut self, routing_key: String, router: Router){
        let item = self.routers.get(&routing_key);
        if item.is_none(){

        }else{

        }
    }

    /// Filter routers to create a subset of `Vec<Router>` with exact matching
    ///
    /// # Arguments
    ///
    /// * `filter` - exact matching pattern
    pub fn filter_routers(filter: String){

    }

    /// Filter routers to create a subset of `Vec<Router>` with regular expressions
    ///
    /// # Arguments
    /// * `filter` - Regular expresion filter
    pub fn match_routers(filter: String){


    }
}
