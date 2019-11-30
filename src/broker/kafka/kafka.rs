//! Broker to manage kafka Queues. This is a celery implementation working much
//! like Kafka Streams.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::HashMap;

use tokio::runtime::Runtime;

use crate::broker::broker::Broker;
use crate::config::config::CannonConfig;
use crate::router::router::Router;
use crate::task::config::TaskConfig;
use crate::message_protocol::message_body::MessageBody;
use crate::app::context::Context;
use crate::error::future_creation_error::FutureCreationError;


/// Kafka Broker
pub struct KafkaBroker{
    config: CannonConfig,
    topics: Option<HashMap<String, Router>>,
}


/// Kafka Broker Implementation
impl Broker for KafkaBroker{

    /// Create and store a future fielding network calls
    fn create_fut(&mut self, runtime: &mut Context) -> Result<bool, FutureCreationError>{
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
    fn send_task(&mut self, runtime: &mut Context, task: TaskConfig, message_body: Option<MessageBody>) {
        unimplemented!()
    }

    /// Subscribe to queues to consume from. Consumers should be futures.
    fn subscribe_to_queues(&mut self, runtime: &mut Context, config: &CannonConfig){

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
