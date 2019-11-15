//! A RabbitMQ channel
//!
//! ---
//! author: Andrew Evans
//! ---

use amiquip::{Channel, ExchangeDeclareOptions, ExchangeType, FieldTable, QueueDeclareOptions, QueueDeleteOptions};

use crate::connection::amqp::threadable_rabbit_mq_connection::ThreadableRabbitMQConnection;


/// Create the exchange from an existing channel
///
/// # Arugments
/// * `channel` - The `amiquip::Channel` to use
/// * `exchange_name` - The exchange name to use
/// * `exchange_options` - Any `amiquip::ExchangeDeclareOptions` to use
fn create_exchange(channel: &Channel,  exchange_name: String, exchange_type: ExchangeType, exchange_options: ExchangeDeclareOptions){
    channel.exchange_declare(exchange_type, exchange_name, exchange_options);
}


/// Create an exchange with default options
///
/// # Arguments
/// * `channel` - The `amiquip::Channel` reference to use
/// * `exchange_name` - The exchange name to use
/// * `exchange_type` - the `amiquip::ExchangeType`
pub fn create_default_exchange(channel: &Channel, exchange_name: String, exchage_type: ExchangeType){
    channel.exchange_declare(exchage_type, exchange_name, ExchangeDeclareOptions::default());
}

/// Drop the exchange
///
/// # Arguments
/// * `channel` - The `amiquip::Channel` reference to use
/// * `exchange_name` - The exchagne name
/// * `if_unused` - Drop if unused
pub fn drop_exchange(channel: &Channel, exchange_name: String, if_unused: bool){
    channel.exchange_delete(exchange_name, if_unused);
}

/// Force drop the exchange
///
/// # Arguments
/// * `channel` - The `amiquip::Channel` reference to use
/// * `exchange_name` - Exchange to drop
pub fn force_drop_exchange(channel: &Channel, exchange_name: String){
    channel.exchange_delete(exchange_name, true);
}

/// Bind a queue to an exchange
///
/// # Arguments
/// * `channel` - The `amiquip::Channel` reference to use
/// * `source` - Source queue
/// * `destination` - Destination exchange
/// * `fieldTable` - FieldTable `std::collections::BTreeMap<std::String, amiquip::AMQPType>` containing arguments
pub fn exchange_bind(channel: &Channel, source: String, destination: String, routing_key: String, fieldTable: FieldTable){
    channel.exchange_bind(destination, source, routing_key, fieldTable);
}

/// Create a queue if not exists
///
/// # Arguments
/// * `channel` - The `amiquip::Channel` reference to use
/// * `queue_name` - The queue name
/// * `options` - Any `amiquip::QueueDeclareOptions`
/// * `nowait` - Whether to block on the creation
pub fn create_queue(channel: &Channel, queue_name: String, options: QueueDeclareOptions, nowait: bool){
    if(nowait == true) {
        channel.queue_declare_nowait(queue_name, options);
    }else{
        channel.queue_declare(queue_name, options);
    }
}

///Bind to a queue
///
/// # Arguments
/// * `channel` - The `amiquip::Channel` reference to use
/// * `queue_name` - The queue name
/// * `exchange` - Name of the exchange
/// * `routing_key` - The routing key
/// * `fieldTable` - FieldTable `std::collections::BTreeMap<std::String, amiquip::AMQPType>` containing arguments
pub fn bind_queue(channel: &Channel, queue_name: String, exchange: String, routing_key: String, field_table: FieldTable, nowait: bool){
    if(nowait == true){
        channel.queue_bind_nowait(queue_name, exchange, routing_key, field_table);
    }else{
        channel.queue_bind(queue_name, exchange, routing_key, field_table);
    }
}

/// Drop the queue
/// * `channel` - The `amiquip::Channel` reference to use
/// * `queue_name` - The queue_name to use
/// * `options` - Any `amiquip::QueueDeleteOptions`
/// * `nowait` - Wehther to block on the call
pub fn drop_queue(channel: &Channel, queue_name: String, options: QueueDeleteOptions, nowait: bool){
    if(nowait == true) {
        channel.queue_delete_nowait(queue_name, options);
    }else{
        channel.queue_delete(queue_name, options);
    }
}
