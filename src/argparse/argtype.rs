//! Arg types for Rust. Allows generic argument storage.
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::{BTreeMap, HashMap};

use amq_protocol_types::*;
use serde_json::{Map, Value};

use crate::AmqpValue;


///Argument Type enum used to store generic values
#[derive(Clone, Debug)]
pub enum ArgType{
    Bool(bool),
    String(String),
    Char(char),
    Int(i64),
    Long(f64),
    Double(f32),
    Usize(usize),
    Bytes(Vec<u8>),
    StringArray(Vec<String>),
    IntArray(Vec<i64>),
    LongArray(Vec<f64>),
    DoubleArray(Vec<f32>),
    CharArray(Vec<char>),
    SizeArray(Vec<usize>),
    Map(HashMap<String, ArgType>),
    Timestamp(u64),
    NULL,
}


/// Convert an argument to an `AMQPValue`
pub fn arg_to_amqp_value(arg: ArgType) -> AmqpValue{
    match arg{
        ArgType::Map(arg) => {
            let mut val_map = BTreeMap::<ShortString, AmqpValue>::new();
            for (key, val) in &arg{
                let amqp_val = arg_to_amqp_value(val.clone());
                let short_key = ShortString::from(key.clone());
                val_map.insert(short_key, amqp_val);
            }
            AMQPValue::FieldTable(amq_protocol_types::FieldTable::from(val_map))
        },
        ArgType::Bool(arg) =>{
            if arg {
                AmqpValue::Boolean(true)
            }else{
                AmqpValue::Boolean(false)
            }
        },
        ArgType::String(arg) =>{
            AmqpValue::LongString(LongString::from(arg))
        },
        ArgType::Char(arg) => {
            let str = arg.to_string();
            AmqpValue::LongString(LongString::from(str))
        },
        ArgType::Int(arg) =>{
            AmqpValue::LongLongInt(amq_protocol_types::LongLongInt::from(arg))
        },
        ArgType::Long(arg) => {
            AmqpValue::Double(amq_protocol_types::Double::from(arg))
        },
        ArgType::Double(arg) =>{
            AmqpValue::Float(amq_protocol_types::Float::from(arg))
        },
        ArgType::Bytes(arg) =>{
            AmqpValue::ByteArray(amq_protocol_types::ByteArray::from(arg))
        },
        ArgType::StringArray(arg) => {
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let arg_str = arg.get(i).unwrap().clone();
                amq_vec.push(AmqpValue::LongString(amq_protocol_types::LongString::from(arg_str)));
            }
            AmqpValue::FieldArray(amq_protocol_types::FieldArray::from(amq_vec))
        },
        ArgType::IntArray(arg) => {
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(AmqpValue::LongLongInt(n));
            }
            AmqpValue::FieldArray(amq_protocol_types::FieldArray::from(amq_vec))
        },
        ArgType::LongArray(arg) => {
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(AmqpValue::Double(n));
            }
            AmqpValue::FieldArray(amq_protocol_types::FieldArray::from(amq_vec))
        },
        ArgType::DoubleArray(arg) =>{
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(AmqpValue::Float(n));
            }
            AmqpValue::FieldArray(amq_protocol_types::FieldArray::from(amq_vec))
        },
        ArgType::CharArray(arg) => {
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone().to_string();
                amq_vec.push(AmqpValue::LongString(amq_protocol_types::LongString::from(n)));
            }
            AmqpValue::FieldArray(amq_protocol_types::FieldArray::from(amq_vec))
        },
        ArgType::Timestamp(arg) =>{
            AmqpValue::Timestamp(amq_protocol_types::Timestamp::from(arg))
        },
        _ => {
            AmqpValue::Void
        },
    }
}


/// Convert an argument to a serde `Value`
pub fn arg_to_value(arg: ArgType) -> Value{
    match arg{
        ArgType::Map(arg) => {
            let mut val_map = Map::new();
            for (key, val) in &arg{
                let serde_val = arg_to_value(val.clone());
                val_map.insert(key.clone(), serde_val);
            }
            Value::Object(val_map)
        },
        ArgType::NULL => {
            Value::Null
        },
        ArgType::Timestamp(arg)  => {
          Value::from(arg)
        },
        ArgType::Bool(arg) =>{
            Value::from(arg)
        },
        ArgType::String(arg) =>{
            Value::from(arg)
        },
        ArgType::Char(arg) => {
            let str = arg.to_string();
            Value::from(str)
        },
        ArgType::Int(arg) =>{
            Value::from(arg)
        },
        ArgType::Long(arg) => {
            Value::from(arg)
        },
        ArgType::Double(arg) =>{
            Value::from(arg)
        },
        ArgType::Bytes(arg) =>{
            let mut amqp_vec = Vec::<Value>::new();
            for i in 0..arg.len(){
                let ssui = arg.get(i).unwrap().clone();
                amqp_vec.push(Value::from(ssui));
            }
            Value::Array(amqp_vec)
        },
        ArgType::StringArray(arg) => {
            let mut amq_vec = Vec::<Value>::new();
            for i in 0..arg.len() {
                let str = arg.get(i).unwrap().clone();
                amq_vec.push(Value::from(str));
            }
            Value::from(amq_vec)
        },
        ArgType::IntArray(arg) => {
            let mut amq_vec = Vec::<Value>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(Value::from(n));
            }
            Value::from(amq_vec)
        },
        ArgType::LongArray(arg) => {
            let mut amq_vec = Vec::<Value>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(Value::from(n));
            }
            Value::from(amq_vec)
        },
        ArgType::DoubleArray(arg) =>{
            let mut amq_vec = Vec::<Value>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(Value::from(n));
            }
            Value::from(amq_vec)
        },
        ArgType::CharArray(arg) => {
            let mut amq_vec = Vec::<Value>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone().to_string();
                amq_vec.push(Value::from(n));
            }
            Value::from(amq_vec)
        },
        _ =>{
            Value::Null
        }
    }
}
