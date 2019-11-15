//! # Structures for sending to RPC
//!
//! RPC structure for calling do send in a broker
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::message_protocol::message::Message;
use std::collections::HashMap;
use crate::argparse::argtype::ArgType;

/// Arguments for the send function with all publicly accessible variables.
///
///  # Arguments
/// * `message` - A message holding the components to send
/// * `exchange` - Override the default exchange
/// * `routing_key` - Override the default routing key
/// * `shadow` - Overridden name for logging
/// * `options` - List of Options following known argument types
pub struct SendArgs{
    pub message: Message,
    pub exchange: Option<String>,
    pub routing_key: Option<String>,
    pub shadow: Option<String>,
    pub options: Option<HashMap<String, ArgType>>,
}
