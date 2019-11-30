//! Implementation of available brokers in a non-asynchronous manner.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

use amq_protocol_types::{AMQPValue, ShortString, ShortUInt};
use lapin::{Channel, ExchangeKind};
use lapin::options::*;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc;

use crate::app::context::Context;
use crate::broker::broker::Broker;
use crate::broker::message::communication::CommunicationEvent;
use crate::broker::message::task::{BrokerMessage, TaskType};
use crate::broker::structs::future_struct::BrokerFuture;
use crate::config::config::CannonConfig;
use crate::connection::amqp::rabbit_mq_connection_pool::RabbitMQConnectionPool;
use crate::error::broker_type_error::BrokerTypeError;
use crate::error::exchange_error::ExchangeError;
use crate::error::future_creation_error::FutureCreationError;
use crate::error::pool_creation_error::PoolCreationError;
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
    broker_futures: Vec<BrokerFuture>,
    num_failure: u8,
    calls_per_failure: u8,
    current_future: i8,
    connection_pool: RabbitMQConnectionPool,
    message_size: usize,
}


/// Broker implementation
impl Broker for RabbitMQBroker{

    /// Restart a future when another future completes (called when another terminates)
    ///
    /// # Arguments
    /// * `context` - Context cantaining the Tokio runtime
    fn create_fut(&mut self, context: &mut Context) -> Result<bool, FutureCreationError>{
        let channel = self.connection_pool.get_channel(context);
        if channel.is_ok(){
            let (comm_sender, fut_comm_receiver): (Sender<CommunicationEvent>, Receiver<CommunicationEvent>) = tokio::sync::mpsc::channel(self.message_size);
            let (fut_comm_sender, comm_receiver) : (Sender<CommunicationEvent>, Receiver<CommunicationEvent>) = tokio::sync::mpsc::channel(self.message_size);
            let fut = RabbitMQBroker::start_future(channel.unwrap(), fut_comm_receiver, fut_comm_sender);
            let r = context.get_runtime().spawn(async move{
                fut.await
            });
            let bfut = BrokerFuture::new(r, comm_sender, comm_receiver);
            self.broker_futures.push(bfut);
            Ok(true)
        }else{
            Err(FutureCreationError)
        }
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
    fn send_task(&mut self, context: &mut Context, task: TaskConfig, message_body: Option<MessageBody>) {
        unimplemented!()
    }

    /// Allows workers to subscribe to the broker which may also invoek send_task for chains and chords
    ///
    /// # Arguments
    /// * `runtime` - Runtime for the application
    /// * `config` - The Cannon Configuration for the application
    fn subscribe_to_queues(&mut self, runtime: &mut Context, config: &CannonConfig){
        unimplemented!()
    }
}


/// Rabbit MQ broker
impl RabbitMQBroker{

    /// Start a future that receives and sends messages
    ///
    /// # Arguments
    /// * `config` - Application configuration
    /// * `channel` - A dedicated `lapin::Channel`
    /// * `comm_receiver` - A dedicated `tokio::sync::mpsc::Receiver` for communications
    /// * `sender` - A `tokio::sync::mpsc::Sender` for responding to communications events and notifying of completion
    async fn start_future(config: CannonConfig, channel: Channel, comm_receiver: Receiver<CommunicationEvent>, sender: Sender<CommunicationEvent>){
        let fut_config = config.clone();
        let mut messages_processed = 0;
        let mut messages_sent = 0;
        let mut fut_comm_receiver = comm_receiver;
        let mut fut_sender = sender.clone();
        loop{
            let event_result = fut_comm_receiver.recv().await;
            if event_result.is_some(){
                let event = event_result.unwrap();
                if let CommunicationEvent::TASK(event) = event {
                    match event{
                        TaskType::SENDTASK(event) => {
                            let cfg = event.get_task_config();
                            let message_body = event.get_body();
                            let message_root_id = event.get_root_id();
                            let msg = cfg.to_amqp_message(&fut_config, message_body, message_root_id);
                            let r= RabbitMQBroker::do_send(&channel, event.get_exchange(), event.get_routing_key(), msg).await;
                        },
                        TaskType::CREATEEXCHANGE(event) =>{
                            
                        },
                        TaskType::CREATEQUEUE(event) => {

                        },
                        TaskType::BINDTOEXCHANGE(event) => {

                        },
                        TaskType::SETPREFETCHLIMIT(event) =>{

                        },
                        TaskType::DROPQUEUE(event) =>{

                        },
                        TaskType::DROPEXCHANGE(event) =>{

                        },
                    }
                    let response = CommunicationEvent::ACKNOWLEDGMENT;
                    fut_sender.send(response).await;
                }else if let CommunicationEvent::GETSTATISTCS = event{
                    let stats = Statistics{
                        messages_sent: messages_sent.clone(),
                        messages_received: messages_processed.clone(),
                    };
                    let response = CommunicationEvent::STATISTICS(stats);
                    fut_sender.send(response).await;
                }else if let CommunicationEvent::COMPLETE = event{
                    let response = CommunicationEvent::COMPLETE;
                    fut_sender.send(response).await;
                    break;
                }else if let CommunicationEvent::PING = event{
                    let response = CommunicationEvent::PONG;
                    fut_sender.send(response).await;
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
    async fn set_prefetch_limit(channel: &Channel, limit: u16, global: bool) -> Result<bool, QOSError>{
        let mut opts = BasicQosOptions::default();
        opts.global = global;
        let res = channel.basic_qos(limit, opts).await;
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
        let res = channel.exchange_delete(exchange.as_str(), opts).await;
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
    /// * `durable` - Whether the queue is durable
    /// * `queue` - The name of the queue
    /// * `nowait` - Drop the queue
    async fn do_create_queue(channel: &Channel, durable: bool, queue: String, nowait: bool) -> Result<bool, QueueError>{
        let mut opts = QueueDeclareOptions::default();
        opts.nowait = nowait;
        opts.durable = durable;
        let args = lapin::types::FieldTable::default();
        let res = channel.queue_declare(queue.as_str(), opts, args).await;
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
        let res = channel.queue_purge(queue.as_str(), opts).await;
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
        let res = channel.queue_delete(queue.as_str(), opts).await;
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
        let args = lapin::types::FieldTable::default();
        let res = channel.exchange_bind(exchange.as_str(), queue.as_str(), routing_key.as_str(), opts, args).await;
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
    async fn do_send(channel: &Channel, exchange: String, routing_key: String, message: Message) -> Result<bool, PublishError> {
        let opts = BasicPublishOptions::default();
        let amq_props = message.properties.clone().convert_to_amqp_properties();
        let payload = message.get_message_payload();
        let chan_result = channel.basic_publish(exchange.as_str(), routing_key.as_str(), opts, payload, amq_props).await;
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
    pub fn new(config: &mut CannonConfig, routers: Option<HashMap<String, Router>>) -> Result<RabbitMQBroker, PoolCreationError>{
        let rt = tokio::runtime::Builder::new().num_threads(config.num_broker_threads.clone()).enable_all().build();
        if rt.is_ok() {
            let mut ctx = Context::new(rt.ok().unwrap());
            let conn_pool = RabbitMQConnectionPool::new(config, &mut ctx, true);
            if conn_pool.is_ok() {
                let rmq = RabbitMQBroker {
                    config: config.clone(),
                    broker_futures: Vec::<BrokerFuture>::new(),
                    num_failure: config.maximum_allowed_failures.clone(),
                    calls_per_failure: config.maximum_allowed_failures_per_n_calls,
                    current_future: 0,
                    connection_pool: conn_pool.ok().unwrap(),
                    message_size: config.message_size.clone(),
                };
                Ok(rmq)
            }else{
                Err(PoolCreationError)
            }
        }else{
            Err(PoolCreationError)
        }
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
