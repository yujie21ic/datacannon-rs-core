//! Router for tasks. The router maintains a key and a list of available queues or topics.
//!
//! ---
//! author: Andrew Evans
//! ---

use regex::Regex;

use crate::message_structure::queues::Queue;


/// Router storing keys and queues
///
/// # Arguments
///
/// * `routing_key` - The routing key for the queue
/// * `queue` - Queue for the router
pub struct Router{
    pub routing_key: String,
    pub queue: Queue,
}


/// Stores `Router` structures
///
/// # Arguments
/// * `routers` - A vector containing `Router` objects
pub struct Routers{
    pub routers: Vec<Router>,
}


/// Router implementation
impl Routers{

    /// Filter routers to create a subset of `Vec<Router>` with exact matching
    ///
    /// # Arguments
    ///
    /// * `filter` - exact matching pattern
    fn filter_routers(filter: String){

    }

    /// Filter routers to create a subset of `Vec<Router>` with regular expressions
    ///
    /// # Arguments
    /// * `filter` - Regular expresion filter
    fn match_routers(filter: String){


    }
}
