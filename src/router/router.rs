/*
Router for tasks. The router maintains a key and a list of available queues or topics.

Author Andrew Evans
*/


use crate::broker::queues::Queues;

pub struct Router{
    routing_key: String,
    queues: Vec<Queues>,
}


impl Router{

}
