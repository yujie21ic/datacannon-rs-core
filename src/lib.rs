//! # Datacannon Core
//!
//! `datacannon_rs_core` is a collection of building blocks for workers and clients in the
//! job queue system.
#![feature(box_into_pin)]
pub mod argparse;
pub mod app;
pub mod backend;
pub mod broker;
pub mod config;
pub mod connection;
pub mod message_protocol;
pub mod nodename;
pub mod serde_utils;
pub mod task;
pub mod security;
pub mod message_structure;
pub mod error;
pub mod replication;
pub mod router;
pub mod registry;
pub mod statistics;
pub mod time;
pub use amq_protocol::protocol::basic::AMQPProperties as AmqpProperties;
pub use amq_protocol::types::AMQPValue as AmqpValue;
pub use tokio;