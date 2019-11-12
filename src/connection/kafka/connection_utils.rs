/*
Connection utilities to kafka

Author Andrew Evans
*/

pub struct KafkaConnectionUtils{
    pub ack_timeout: i32,
    pub num_acks: i8,
    pub host: String,
    pub port: String,
}
