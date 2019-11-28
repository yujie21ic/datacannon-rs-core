//! General configuration for the framework
//!
//! ---
//! author: Andrew Evans
//! ---

use lapin::ExchangeKind;
use std::collections::HashMap;

use num_cpus;

use crate::argparse::argtype::ArgType;
use crate::backend::config::BackendConfig;
use crate::connection::connection::ConnectionConfig;
use crate::message_structure::queues::{GenericQueue};
use crate::replication::replication::HAPolicy;
use crate::router::router::Routers;

/// Queue persistance type
///
/// # Arguments
/// * `PERSISTENT` - When possible, queue will persist
/// * `NONPERSISTENT` - When possible, queue will drop
#[derive(Clone, Debug)]
pub enum QueuePersistenceType{
    PERSISTENT,
    NONPERSISTENT
}


/// Backend types available
///
/// # Arguments
/// * `RABBITMQ` - Ues RabbitMQ
/// * `REDIS` - Uses Redis
/// * `KAFKA` - Uses Kafka
#[derive(Clone, Debug)]
pub enum BackendType{
    RABBITMQ,
    REDIS,
    KAFKA,
}


/// Broker type
///
/// # Arguments
/// * `RABBITMQ` - Use RabbitMQ
/// * `KAFKA` - Uses Kafka
#[derive(Clone, Debug)]
pub enum BrokerType{
    RABBITMQ,
    KAFKA,
}


/// Admin information
///
/// # Arguments
/// * `name` - Administrator name
/// * `email` - Administrator email
#[derive(Clone, Debug)]
pub struct Admin{
    name: String,
    email: String,
}


/// Configuration for the application with all public variables
///
/// # Arguments
/// * `connection_inf` - The relevant `crate::connection::connection::ConnectionConfig`
/// * `broker_connection_retry` - Whether to retry a connection
/// * `result_backend` - The relevant `crate::backend::config::BackendConfig`
/// * `routers` - Router objects stored in `crate::router::router::Router`
/// * `cannon_cache_backend` - Backend for caching
/// * `send_events` - Whether to send events
/// * `default_exchange_type` - Default exchange name
/// * `default_queue` - Default queue when message queue not provided
/// * `event_queue`- Event queue from specified queues
/// * `event_exchange` - Exchange for events
/// * `event_exchange_type` - Type of event exchange
/// * `event_routing_key` - Default routing key for events
/// * `result_exchange` - Default result exchange or topic
/// * `accept_content` - Type of content to accept
/// * `worker_prefetch_multiplier` - Number of messages to prefetch on a single consumer
/// * `default_deilvery_mod` - Persistence type for delivery
/// * `default_routing_key` - Default message routing key
/// * `broker_connection_timeout` - Timeout for a broker connection
/// * `broker_connection_max_retries` - Maximum number of times to attempt to retry a connection
/// * `cannon_send_task_error_emails` - Whether to email on error
/// * `admins` - A vector of Admins
/// * `server_email` - Sender email address
/// * `mail_host` - Server to send mail from
/// * `mail_host_user` - Username on the mail server
/// * `mail_host_password` - Password for the mail server
/// * `mail_port` - Port to send mail from
/// * `track_started` - Send a notification on the events queue/topic that processing started
/// * `acks_late` - Whether to acknowledge after executing a task
/// * `store_errors_even_if_ignored` - Whether to store errors if ignored
/// * `task_result_expires` - Expiration date for the task result
/// * `ignore_result` - Do not send result on backend even if provided
/// * `max_cached_results` - Maximum number of cached results
/// * `result_persistent` - Persistence type for backend if allowed
/// * `result_serializer` - The result serialization type
/// * `database_engine_options` - If storing results in a database, these options are for the engine
/// * `default_rate_limit` - Maximum rate of generating tasks in tasks per second
/// * `disable_rate_limits` - Whether to ignore all rate limits which is the default
/// * `num_connections` - Maximum number of open broker and backend connections to allow
/// * `ha_policy` - High availability polcies for rabbitmq or kafka
/// * `create_missing_queues` - Tell the system to create missing queues which it does by default
/// * `worker_direct` - Create a queue for each worker
/// * `broker_login_method` - Default login method such as basic or oauth where possible
/// * `task_queue_max_priority` - max priority for rabbitmq
/// * `task_default_priority` - default priority for rabbitmq
/// * `maximum_allowed_failures` - allowed number of failures per n
/// * `maximum_allowed_failures_per_n_calls` - check the failures after n calls
/// * `task_retries` - Maximum number of task retries
/// * `default_lang` - The default language
/// * `app_lang` - The application language
#[derive(Clone, Debug)]
pub struct CannonConfig{
    pub connection_inf: ConnectionConfig,
    pub result_backend: BackendConfig,
    pub routers: Routers,
    pub cache_backend: Option<BackendConfig>,
    pub send_events: bool,
    pub default_exchange: String,
    pub default_exchange_type: ExchangeKind,
    pub default_queue: String,
    pub event_queue: String,
    pub event_exchange: String,
    pub event_exchange_type: ExchangeKind,
    pub event_routing_key: String,
    pub result_exchange: String,
    pub accept_content: String,
    pub worker_prefetch_multiplier: i8,
    pub default_delivery_mode: QueuePersistenceType,
    pub default_routing_key: String,
    pub broker_connection_timeout: i64,
    pub broker_connection_max_retries: i64,
    pub acks_late: bool,
    pub task_result_expires: i64,
    pub ignore_result: bool,
    pub max_cached_results: i32,
    pub result_persistent: QueuePersistenceType,
    pub result_serializer: String,
    pub database_engine_options: Option<HashMap<String, String>>,
    pub num_broker_connections: usize,
    pub num_broker_threads: usize,
    pub num_backend_connections: usize,
    pub num_backend_threads: usize,
    pub ha_policy: Option<HAPolicy>,
    pub create_missing_queues: bool,
    pub broker_transport_options: Option<HashMap<String, ArgType>>,
    pub task_queue_max_priority: Option<i8>,
    pub task_default_priority: i8,
    pub maximum_allowed_failures: u8,
    pub maximum_allowed_failuires_per_n_calls: u8,
    pub task_retries: u8,
    pub default_lang: String,
    pub app_lang: String,
    pub encoding_type: String,
}


/// Implementation of Celery configuration
impl CannonConfig{

    /// Create a new configuration
    ///
    /// # Arguments
    /// * `connection_inf` - The relevant `crate::connection::connection::ConnectionConfig`
    /// * `broker_connection_retry` - Whether to retry a connection
    /// * `result_backend` - The relevant `crate::backend::config::BackendConfig`
    /// * `routers` - Router objects stored in `crate::router::router::Router`
    /// * `cannon_cache_backend` - Backend for caching
    /// * `send_events` - Whether to send events
    /// * `default_exchange_type` - Default exchange name
    /// * `default_queue` - Default queue when message queue not provided
    /// * `event_queue`- Event queue from specified queues
    /// * `event_exchange` - Exchange for events
    /// * `event_exchange_type` - Type of event exchange
    /// * `event_routing_key` - Default routing key for events
    /// * `result_exchange` - Default result exchange or topic
    /// * `accept_content` - Type of content to accept
    /// * `worker_prefetch_multiplier` - Number of messages to prefetch on a single consumer
    /// * `default_deilvery_mod` - Persistence type for delivery
    /// * `default_routing_key` - Default message routing key
    /// * `broker_connection_timeout` - Timeout for a broker connection
    /// * `broker_connection_max_retries` - Maximum number of times to attempt to retry a connection
    /// * `cannon_send_task_error_emails` - Whether to email on error
    /// * `admins` - A vector of Admins
    /// * `server_email` - Sender email address
    /// * `mail_host` - Server to send mail from
    /// * `mail_host_user` - Username on the mail server
    /// * `mail_host_password` - Password for the mail server
    /// * `mail_port` - Port to send mail from
    /// * `track_started` - Send a notification on the events queue/topic that processing started
    /// * `acks_late` - Whether to acknowledge after executing a task
    /// * `store_errors_even_if_ignored` - Whether to store errors if ignored
    /// * `task_result_expires` - Expiration date for the task result
    /// * `ignore_result` - Do not send result on backend even if provided
    /// * `max_cached_results` - Maximum number of cached results
    /// * `result_persistent` - Persistence type for backend if allowed
    /// * `result_serializer` - The result serialization type
    /// * `database_engine_options` - If storing results in a database, these options are for the engine
    /// * `default_rate_limit` - Maximum rate of generating tasks in tasks per second
    /// * `disable_rate_limits` - Whether to ignore all rate limits which is the default
    /// * `num_connections` - Maximum number of open broker and backend connections to allow
    /// * `ha_policy` - High availability polcies for rabbitmq or kafka
    /// * `create_missing_queues` - Tell the system to create missing queues which it does by default
    /// * `worker_direct` - Create a queue for each worker
    /// * `broker_login_method` - Default login method such as basic or oauth where possible
    /// * `task_queue_max_priority` - max priority for rabbitmq
    /// * `task_default_priority` - default priority for rabbitmq
    /// * `maximum_allowed_failures` - allowed number of failures per n
    /// * `maximum_allowed_failures_per_n_calls` - check the failures after n calls
    /// * `task_retries` - Maximum number of task retries
    /// * `default_lang` - The default language
    /// * `app_lang` - The application language
    /// * `encoding_Type` - String encoding type
    pub fn new(conn_inf: ConnectionConfig, backend: BackendConfig, routers: Routers) -> CannonConfig{
        CannonConfig{
            connection_inf: conn_inf,
            result_backend: backend,
            cache_backend: None,
            send_events: false,
            routers: routers,
            default_exchange: String::from("cannon"),
            default_exchange_type: ExchangeKind::Direct,
            default_queue: String::from("cannon"),
            event_queue: String::from("cannon"),
            event_exchange: String::from("cannon_event"),
            event_exchange_type: ExchangeKind::Topic,
            event_routing_key: String::from("cannonevent"),
            result_exchange: String::from("cannonresult"),
            accept_content: String::from("application/json"),
            worker_prefetch_multiplier: 4,
            default_delivery_mode: QueuePersistenceType::PERSISTENT,
            default_routing_key: String::from("cannon"),
            broker_connection_timeout: 5000,
            broker_connection_max_retries: 1000,
            acks_late: true,
            task_result_expires: 600000,
            ignore_result: false,
            max_cached_results: 100,
            result_persistent: QueuePersistenceType::NONPERSISTENT,
            result_serializer: String::from("json"),
            database_engine_options: None,
            num_broker_connections: num_cpus::get(),
            num_broker_threads: (num_cpus::get()/2) as usize,
            num_backend_connections: num_cpus::get(),
            num_backend_threads: (num_cpus::get()/2) as usize,
            ha_policy: None,
            create_missing_queues: true,
            broker_transport_options: None,
            task_queue_max_priority: None,
            task_default_priority: 0,
            maximum_allowed_failures: 2,
            maximum_allowed_failuires_per_n_calls: 10,
            task_retries: 3,
            default_lang: "rs".to_string(),
            app_lang: "rs".to_string(),
            encoding_type: "utf-8".to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::connection::amqp::connection_inf::AMQPConnectionInf;
    use crate::connection::kafka::connection_inf::KafkaConnectionInf;

    use super::*;

    #[test]
    fn should_create_a_configuration(){
        let broker_conf = AMQPConnectionInf::new(
            String::from("amqp"),
            String::from("127.0.0.1"),
            5672,
            Some(String::from("test")),
            Some(String::from("dev")),
            Some(String::from("rtp*4500")),
            false,
            None,
            None,
            10000,
        );
        let b = BackendConfig{
            url: "fake".to_string(),
            username: None,
            password: None,
            transport_options: None,
        };
        let kinf = KafkaConnectionInf{
            ack_timeout: 0,
            num_acks: 0,
            host: "".to_string(),
            port: "".to_string()};
        let r  = Routers::new();
        let c = CannonConfig::new(ConnectionConfig::RabbitMQ(broker_conf), b,r);
        let conn_inf = c.connection_inf;
        if let ConnectionConfig::RabbitMQ(conn_inf) = conn_inf {
            let url = conn_inf.to_url();
            assert!(url.eq("amqp://dev:rtp*4500@127.0.0.1:5672/test"))
        }
    }
}
