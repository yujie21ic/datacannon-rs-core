//! Implementation of available brokers in a non-asynchronous manner.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::HashMap;

use lapin::{Channel, ExchangeKind};
use lapin::options::*;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};

use crate::broker::broker::{Broker, BrokerEvent, CommunicationEvent};
use crate::config::config::CannonConfig;
use crate::error::broker_type_error::BrokerTypeError;
use crate::error::exchange_error::ExchangeError;
use crate::error::publish_error::PublishError;
use crate::error::queue_error::QueueError;
use crate::message_protocol::headers::Headers;
use crate::message_protocol::message_body::MessageBody;
use crate::message_protocol::properties::Properties;
use crate::router::router::Router;
use crate::task::config::TaskConfig;
use crate::app::context::Context;


/// RabbitMQ Broker
///
/// # Arguments
/// * `config` - Canon configuration
/// * `pool` - RabbitMQ connectionpool
/// * `broker_futures` - Active futures for monitoring
/// * `event_senders` - For sending events to futures
/// * `comm_receivers` - Receivers for communications from futures
/// * `num_failures` - Number of failures
pub struct RabbitMQBroker{
    config: CannonConfig,
    event_senders: Vec<UnboundedSender<BrokerEvent>>,
    comm_receivers: Vec<UnboundedReceiver<CommunicationEvent>>,
    num_failure: i32,
    current_future: i8,
}


/// Broker implementation
impl Broker for RabbitMQBroker{

    /// Restart a future when another future completes (called when another terminates)
    ///
    /// # Arguments
    /// * `runtime` - Tokio runtime to create the future on
    fn create_fut(&mut self, runtime: &Runtime) {
        unimplemented!()
    }

    /// Start the broker futures
    fn setup(&mut self, runtime: &Runtime){
        unimplemented!()
    }

    /// Drop the future
    ///
    /// # Arguments
    /// * `idx` - Index of the future to drop
    fn drop_future(&mut self, idx: usize) {
        unimplemented!()
    }

    /// Teardown the broker
    fn teardown(&mut self){
        unimplemented!()
    }

    /// Close all connections and thus the broker. shut down futures first
    fn close(&mut self){
        unimplemented!()
    }

    /// Send a task
    ///
    /// # Arguments
    /// * `runtime` - Tokio runtime to send task on
    /// * `task` - The task config
    fn send_task(&mut self, runtime: &Runtime, task: TaskConfig, message_body: Option<MessageBody>) {
        unimplemented!()
    }

    /// Allows workers to subscribe to the broker which may also invoek send_task for chains and chords
    ///
    /// # Arguments
    /// * `runtime` - Runtime for the application
    /// * `config` - The Cannon Configuration for the application
    fn subscribe_to_queues(&mut self, runtime: &Runtime, config: &CannonConfig){
        unimplemented!()
    }
}

async fn start_future(config: CannonConfig, channel: Channel){

}

/// Rabbit MQ broker
impl RabbitMQBroker{

    /// Create the exchange
    ///
    /// # Arguments
    /// * `config` - The reference to the application `crate::config::config::CannonConfig`
    /// * `channel` - Reference to a `amiquip::Channel`
    /// * `durabe` - Whether the exchange persists
    /// * `exchange` - Name of the exchange
    /// * `exchange_type` - The exchange type
    /// * `nowait` - Whether to wait for the exchange to be created
    async fn do_create_exchange(config: &CannonConfig, channel: &Channel, durable: bool, exchange: String, exchange_type: ExchangeKind, nowait: bool) -> Result<bool, ExchangeError> {
        let mut opts = ExchangeDeclareOptions::default();
        opts.durable = durable;
        opts.nowait = nowait;
        let ftable = lapin::types::FieldTable::default();
        let res = channel.exchange_declare(exchange.as_str(), exchange_type, opts, ftable).await;
        if res.is_ok() {
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// Create a queue
    ///
    /// # Arguments
    /// * `config` - Reference to the application `crate::config::config::CannonConfig`
    /// * `channel` - A reference to a `amiquip::Channel`
    /// * `durable` - Whether the channel should persist
    /// * `queue` - The name of the queue
    /// * `declare_exchange` - Whether to declare the exchange
    /// * `uuid` - Unique id for the message e
    /// * `exchange` - Name of the exchange
    /// * `routing_key` - Routing key for the exchange
    async fn do_create_queue(config: &CannonConfig, channel: &Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, QueueError>{
        unimplemented!()
    }

    /// Bind a queue to an exchange
    ///
    /// # Arguments
    /// * `config` - The application `crate::config::config::CannonConfig`
    /// * `channel` - A `amiquip::Channel`
    /// * `exchange` - Name of the exchange
    /// * `queue` - Name of the queue to bind
    /// * `routing_key` - Name of the routing key
    async fn do_bind_to_exchange(config: &CannonConfig, channel: &Channel, exchange: String, queue: String, routing_key: String) -> Result<bool, ExchangeError> {
        unimplemented!()
    }

    /// Send a task to the broker
    ///
    /// # Arguments
    /// * `config` - The application `crate::config::config::CannonConfig`
    /// * `channel` - A `amiquip::Channel`
    /// * `props` - The `amiquip::Properties` for the message
    /// * `headers` - Relevant message `crate::message_protocol::headers::Headers`
    /// * `body` - Relevant message `crate::message_protocol::body::MessageBody`\
    /// * `exchange` - Name of the exchange to use
    /// * `routing_key` - Name of the routing key
    pub fn do_send(config: &CannonConfig, channel: &Channel, props: Properties, headers: Headers, body: MessageBody, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, PublishError> {
        unimplemented!()
    }

    /// Drop the queue
    ///
    /// # Arguments
    /// * `config` - Reference to the application `crate::config::config::CannonConfig`
    /// * `channel` - Reference to an `amiquip::Channel`
    /// * `name` - Name of the string
    pub fn do_drop_queue(config: &CannonConfig, channel: &Channel, queue: String) -> Result<bool, QueueError> {
        unimplemented!()
    }

    /// get a channel from the pool
    fn get_channel(&mut self) -> Channel{
        unimplemented!()
    }

    /// Create a new broker
    pub fn new(config: &mut CannonConfig, routers: Option<HashMap<String, Router>>, min_connections: Option<usize>, num_futures: usize) -> Result<RabbitMQBroker, BrokerTypeError>{
        unimplemented!()
    }
}
