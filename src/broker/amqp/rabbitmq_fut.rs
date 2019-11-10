/*
RabbitMQ future

Author Andrew Evans
*/


use tokio::sync::mpsc::{Receiver, Sender};
use crate::message_protocol::message::Message;
use std::future::Future;
use crate::broker::amqp::rabbitmq::RabbitMQBroker;
use crate::config::config::CeleryConfig;
use crate::config::config::BrokerType::RABBITMQ;
use crate::broker::queues::Queues;
use std::collections::HashMap;
use crate::router::router::Router;
use amiquip::Channel;


/// create a rabbitmq broker future
async fn send_task_future(channel: Channel, celery_config: CeleryConfig, queues: Option<Queues>, routers: Option<HashMap<String, Router>>, min_connections: Option<usize>, sender: Option<Sender<Message>>, receiver: &mut Receiver<Message>, num_futures: usize) -> Result<(), &'static str> {
    ///config: CeleryConfig, queues: Option<Queues>, routers: Option<HashMap<String, Router>>, min_connections: Option<usize>, sender: Option<Sender<Message>>, receiver: Receiver<Message>, num_futures: usize
    let conf = celery_config.clone();
    loop{
        let m = receiver.recv().await.unwrap();
        RabbitMQBroker::do_send()
    }
    Ok(())
}
