/*
RabbitMQ future

Author Andrew Evans
*/


use tokio::sync::mpsc::{Receiver, Sender};
use crate::message_protocol::message::Message;
use std::future::Future;
use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::config::config::CannonConfig;
use crate::config::config::BrokerType::RABBITMQ;
use std::collections::HashMap;
use crate::router::router::Router;
use amiquip::Channel;
use crate::app::send_rpc::SendArgs;
use crate::broker::amqp::broker_trait::AMQPBroker;
use crate::message_structure::queues::Queues;


/// create a rabbitmq broker future
async fn send_task_future(channel: Channel, celery_config: CannonConfig, queues: Option<Queues>, routers: Option<HashMap<String, Router>>, min_connections: Option<usize>, sender: Option<Sender<Message>>, receiver: &mut Receiver<SendArgs>, num_futures: usize) -> Result<(), &'static str> {

    let conf = celery_config.clone();
    loop{
        let args = receiver.recv().await.unwrap();
        let m = args.message;
        let props = m.properties.clone();
        let headers = m.headers.clone();
        let body = m.body.clone();
        let kwargs = m.kwargs.clone();
        let exchange = args.exchange;
        let routing_key = args.routing_key;
        RabbitMQBroker::do_send(&celery_config, &channel, props, headers, body, exchange, routing_key);
    }
    Ok(())
}
