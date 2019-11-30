//! Implementation of available brokers in a non-asynchronous manner.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::{BTreeMap, HashMap};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use amq_protocol_types::{AMQPValue, FieldTable, ShortUInt};
use amq_protocol_types::AMQPType::ShortString;
use lapin::{Channel, ExchangeKind};
use lapin::options::*;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::app::context::Context;
use crate::broker::broker::{Broker, BrokerEvent, CommunicationEvent};
use crate::broker::message::communication::CommunicationEvent;
use crate::broker::message::protocol::BrokerMessage;
use crate::config::config::CannonConfig;
use crate::error::broker_type_error::BrokerTypeError;
use crate::error::exchange_error::ExchangeError;
use crate::error::publish_error::PublishError;
use crate::error::qos_error::QOSError;
use crate::error::queue_error::QueueError;
use crate::message_protocol::headers::Headers;
use crate::message_protocol::message::Message;
use crate::message_protocol::message_body::MessageBody;
use crate::message_protocol::properties::Properties;
use crate::router::router::Router;
use crate::statistics::message::Statistics;
use crate::task::config::TaskConfig;
use crate::connection::amqp::rabbit_mq_connection_pool::RabbitMQConnectionPool;


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
    connection_pool: RabbitMQConnectionPool,
}


/// Broker implementation
impl Broker for RabbitMQBroker{

    /// Restart a future when another future completes (called when another terminates)
    ///
    /// # Arguments
    /// * `runtime` - Tokio runtime to create the future on
    fn create_fut(&mut self, runtime: &Runtime) {
        let channel = self.connection_pool.get_channel();
        if channel.is_ok()
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


/// Rabbit MQ broker
impl RabbitMQBroker{

    /// Start a future that receives and sends messages
    ///
    /// # Arguments
    /// * `channel` - A dedicated `lapin::Channel`
    /// * `task_receiver` - A dedicated `tokio::sync::mpsc::Receiver` for tasks
    /// * `comm_receiver` - A dedicated `tokio::sync::mpsc::Receiver` for communications
    /// * `sender` - A `tokio::sync::mpsc::Sender` for responding to communications events and notifying of completion
    async fn start_future(channel: Channel, task_receiver: Receiver<BrokerMessage>, comm_receiver: Receiver<CommunicationEvent>, sender: Sender<CommunicationEvent>){
        let mut messages_processed = 0;
        let mut messages_sent = 0;
        loop{
            let task_result  = task_receiver.recv_timeout(Duration::from_millis(5000));
            if task_result.is_ok(){

            }
            let event_result = comm_receiver.try_recv();
            if event_result.is_ok(){
                let event = event_result.unwrap();
                if let CommunicationEvent::GETSTATISTCS = event{
                    let stats = Statistics{
                        messages_sent: messages_sent.clone(),
                        messages_received: messages_processed.clone(),
                    };
                    let response = CommunicationEvent::STATISTICS(response);
                    sender.send(response);
                }else if let CommunicationEvent::COMPLETE = event{
                    let response = CommunicationEvent::COMPLETE;
                    sender.send(response);
                    break;
                }else if let CommunicationEvent::PING = event{
                    let response = CommunicationEvent::PONG;
                    sender.send(response);
                }
            }
        }
    }

    /// Set the prefetch limit asynchronously
    ///
    /// # Arguments
    /// * `channel` - The channel to use in setting the limit
    /// * `limit` - Maximum number of messages to prefetch
    /// * `global` - Whether the limit applies to all channels without a specified limit
    async fn set_prefetch_limit(channel: &Channel, limit: usize, global: bool) -> Result<bool, QosError>{
        let amq_limit = amq_protocol_types::ShortShortInt::from(limit);
        let mut opts = BasicQosOptions::default();
        opts.global = global;
        let res = channel.basic_qos(amq_limit, opts);
        if res.is_ok(){
            Ok(true)
        }else{
            Err(QOSError)
        }
    }

    /// Create the exchange
    ///
    /// # Arguments
    /// * `channel` - Reference to a `amiquip::Channel`
    /// * `durabe` - Whether the exchange persists
    /// * `exchange` - Name of the exchange
    /// * `exchange_type` - The exchange type
    /// * `nowait` - Whether to wait for the exchange to be created
    async fn do_create_exchange(channel: &Channel, durable: bool, exchange: String, exchange_type: ExchangeKind, nowait: bool) -> Result<bool, ExchangeError> {
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

    /// Create the exchange
    ///
    /// # Arguments
    /// * `channel` - Reference to a `lapin::Channel`
    /// * `exchange` - Name of the exchange
    /// * `nowait` - Whether to wait for the queue to drop
    async fn do_drop_exchange(channel: &Channel, exchange: String, nowait: bool) -> Result<bool, ExchangeError> {
        let mut opts = ExchangeDeleteOptions::default();
        opts.nowait = nowait;
        let res = channel.exchange_delete(exchange, opts);
        if res.is_ok() {
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// Create a queue
    ///
    /// # Arguments
    /// * `channel` - A reference to a `amiquip::Channel`
    /// * `queue` - The name of the queue
    /// * `nowait` - Drop the queue
    async fn do_create_queue(channel: &Channel, durable: bool, queue: String, declare_exchange: bool, uuid: String, exchange: Option<String>, routing_key: Option<String>) -> Result<bool, QueueError>{
        let opts = QueueDeleteOptions::default();
        opts.nowait = nowait;
        let res = channel.queue_delete(queue, opts).await;
        if res.is_ok(){
            Ok(true)
        }else{
            Err(QueueError)
        }
    }


    ///Purge all messages from the queue
    ///
    /// # Arguments
    /// * `channel` - A reference to a `amiquip::Channel`
    /// * `queue` - The name of the queue
    /// * `nowait` - Drop the queue
    async fn purge_queue(channel: &Channel, queue: String, nowait: bool) -> Result<bool, QueueError>{
        let mut opts = QueuePurgeOptions::default();
        opts.nowait = nowait;
        let res = channel.queue_purge(queue, opts);
        if res.is_ok(){
            Ok(true)
        }else{
            Err(QueueError)
        }
    }

    /// Create a queue
    ///
    /// # Arguments
    /// * `channel` - A reference to a `amiquip::Channel`
    /// * `queue` - The name of the queue
    /// * `nowait` - Whether to wait for the queue to drop
    async fn do_drop_queue(channel: &Channel, queue: String, nowait: bool) -> Result<bool, QueueError>{
        let mut opts = QueueDeleteOptions::default();
        opts.nowait = nowait;
        let res = channel.queue_delete(queue, opts);
        if res.is_ok(){
            Ok(true)
        }else{
            Err(QueueError)
        }
    }

    /// Bind a queue to an exchange
    ///
    /// # Arguments
    /// * `channel` - A `amiquip::Channel`
    /// * `exchange` - Name of the exchange
    /// * `queue` - Name of the queue to bind
    /// * `routing_key` - Name of the routing key
    /// * `nowait` - Whether to wait for the binding function
    async fn do_bind_to_exchange(channel: &Channel, exchange: String, queue: String, routing_key: String, nowait: bool) -> Result<bool, ExchangeError> {
        let mut opts = ExchangeBindOptions::default();
        opts.nowait = nowait;
        let bmap = BTreeMap::<ShortString, AMQPValue>::new();
        let args = FieldTable(bmap);
        let res = channel.exchange_bind(exchange, queue, routing_key, opts, args);
        if res.is_ok(){
            Ok(true)
        }else{
            Err(ExchangeError)
        }
    }

    /// Send a task to the broker
    ///
    /// # Arguments
    /// * `channel` - A `lapin::Channel`
    /// * `exchange` - Name of the exchange to use
    /// * `routing_key` - Name of the routing key
    /// * `message` - Message containing relevant properties
    async fn do_send(channel: &Channel, exchange: Option<String>, routing_key: Option<String>, message: Message) -> Result<bool, PublishError> {
        let opts = BasicPublishOptions::default();
        let amq_props = message.properties.clone().convert_to_amqp_properties();
        let payload = message.get_message_payload();
        let chan_result = channel.basic_publish(exchange, routing_key, options, amq_props).await;
        if chan_result.is_ok(){
            Ok(true)
        }else{
            Err(PublishError)
        }
    }

    /// Create a new broker
    ///
    /// # Arguments
    /// * `config` - The configuration for the application
    /// * `routers` - Routers maintained and checked by the broker
    pub fn new(config: &mut CannonConfig, routers: Option<HashMap<String, Router>>) -> Result<RabbitMQBroker, BrokerTypeError>{
        unimplemented!()
    }
}


#[cfg(test)]
mod tests{

    #[test]
    fn should_create_broker(){

    }

    #[test]
    fn should_create_router_queues_and_exchanges(){

    }

    #[test]
    fn should_drop_queues_when_requested(){

    }

    #[test]
    fn should_purge_queue_on_request(){

    }

    #[test]
    fn should_send_tasks(){

    }
}
