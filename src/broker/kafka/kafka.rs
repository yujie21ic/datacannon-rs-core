//! Broker to manage kafka Queues. This is a celery implementation working much
//! like Kafka Streams.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::HashMap;

use tokio::sync::mpsc::Sender;
use tokio::runtime::Runtime;

use crate::argparse::argtype::ArgType;
use crate::broker::broker::Broker;
use crate::config::config::CannonConfig;
use crate::message_protocol::message::Message;
use crate::router::router::Router;


/// Kafka Broker
pub struct KafkaBroker{
    config: CannonConfig,
    topics: Option<HashMap<String, Router>>,
}


/// Kafka Broker Implementation
impl Broker for KafkaBroker{

    /// Create and store a future fielding network calls
    fn create_fut(&mut self) {
        unimplemented!()
    }

    /// Setup the broker
    fn setup(&mut self) {
        unimplemented!()
    }

    /// Teardown the broker
    fn teardown(&mut self) {
        unimplemented!()
    }

    /// Close the broker
    fn close(&mut self) {
        unimplemented!()
    }

    ///Send task to broker
    fn send_task(&mut self, task: String, args: Vec<ArgType>, app_sender: Sender<Message>) {
        unimplemented!()
    }

    /// Drop a future at the given index
    fn drop_future(&mut self, idx: usize) {
        unimplemented!()
    }
}


impl KafkaBroker{

    /// send message to a topic
    fn send_to_topic(){

    }

    ///
    fn get_topics(){

    }

    fn create_topics(){

    }

    fn new(){

    }
}


#[cfg(test)]
mod tests{

    #[test]
    fn should_retreive_list_of_topics_from_zookeeper(){

    }

    #[test]
    fn should_create_missing_topics(){

    }

    #[test]
    fn should_create_Kafka_as_needed(){

    }

    #[test]
    fn should_send_message_to_kafka_topci(){

    }
}
