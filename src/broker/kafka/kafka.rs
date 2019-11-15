/*
Broker to manage kafka Queues. This is a celery implementation working much
like Kafka Streams.

Implementation

Author Andrew Evans
*/

use std::collections::HashMap;

use tokio::sync::mpsc::Sender;
use tokio::runtime::Runtime;

use crate::argparse::argtype::ArgType;
use crate::broker::broker::Broker;
use crate::config::config::CannonConfig;
use crate::message_protocol::message::Message;
use crate::router::router::Router;
use crate::message_structure::queues::Queues;

pub struct KafkaBroker{
    config: CannonConfig,
    routers: Option<HashMap<String, Router>>,
    queues: Option<Queues>,
    topics: Option<HashMap<String, Router>>,
}


impl Broker for KafkaBroker{

    fn setup(&mut self, rt: Runtime) {
        unimplemented!()
    }

    fn teardown(&mut self) {
        unimplemented!()
    }

    fn close(&mut self) {
        unimplemented!()
    }

    fn send_task(&mut self, task: String, args: Vec<ArgType>, app_sender: Sender<Message>) {
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
