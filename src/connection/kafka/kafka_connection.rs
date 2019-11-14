/*
A kafka connection.

Author Andrew Evans
*/

#[derive(Clone, Debug)]
pub struct KafkaConnectionInf {
    pub ack_timeout: i32,
    pub num_acks: i8,
    pub host: String,
    pub port: String,
}