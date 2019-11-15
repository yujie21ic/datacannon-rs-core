//! Arg types for Rust. Allows generic argument storage.
//!
//! ---
//! author: Andrew Evans
//! ---


use std::collections::{HashMap, BTreeMap};
use serde_json::{Value, Map};
use amiquip::{AmqpValue, FieldTable};


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
            let mut val_map: FieldTable = FieldTable::new();
            for (key, val) in &arg{
                let amqp_val = arg_to_amqp_value(val.clone());
                val_map.insert(key.clone(), amqp_val);
            }
            AmqpValue::FieldTable(val_map)
        },
        ArgType::Bool(arg) =>{
            AmqpValue::Boolean(arg)
        },
        ArgType::String(arg) =>{
            AmqpValue::LongString(arg)
        },
        ArgType::Char(arg) => {
            let str = arg.to_string();
            AmqpValue::LongString(str)
        },
        ArgType::Int(arg) =>{
            AmqpValue::LongLongInt(arg)
        },
        ArgType::Long(arg) => {
            AmqpValue::Double(arg)
        },
        ArgType::Double(arg) =>{
            AmqpValue::Float(arg)
        },
        ArgType::Bytes(arg) =>{
            let mut amqp_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len(){
                let ssui = arg.get(i).unwrap();
                amqp_vec.push(AmqpValue::ShortShortUInt(ssui.clone()));
            }
            AmqpValue::FieldArray(amqp_vec)
        },
        ArgType::StringArray(arg) => {
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let str = arg.get(i).unwrap().clone();
                amq_vec.push(AmqpValue::LongString(str));
            }
            AmqpValue::FieldArray(amq_vec)
        },
        ArgType::IntArray(arg) => {
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(AmqpValue::LongLongInt(n));
            }
            AmqpValue::FieldArray(amq_vec)
        },
        ArgType::LongArray(arg) => {
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(AmqpValue::Double(n));
            }
            AmqpValue::FieldArray(amq_vec)
        },
        ArgType::DoubleArray(arg) =>{
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone();
                amq_vec.push(AmqpValue::Float(n));
            }
            AmqpValue::FieldArray(amq_vec)
        },
        ArgType::CharArray(arg) => {
            let mut amq_vec = Vec::<AmqpValue>::new();
            for i in 0..arg.len() {
                let n = arg.get(i).unwrap().clone().to_string();
                amq_vec.push(AmqpValue::LongString(n));
            }
            AmqpValue::FieldArray(amq_vec)
        },
        ArgType::Timestamp(arg) =>{
            AmqpValue::Timestamp(arg)
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
