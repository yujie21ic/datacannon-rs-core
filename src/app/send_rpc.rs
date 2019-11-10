/*
RPC structure for calling do send in a broker
*/

use crate::message_protocol::message::Message;
use std::collections::HashMap;
use crate::argparse::argtype::ArgType;


pub struct SendArgs{
    pub message: Message,
    pub exchange: Option<String>,
    pub routing_key: Option<String>,
    pub shadow: Option<String>,
    pub options: Option<HashMap<String, ArgType>>,
}
