//! Arguments for RPC in the framework
//!
//! ---
//! author: Andrew Evans
//! ---

use std::vec::Vec;

use crate::AmqpValue;
use amq_protocol::types::AMQPType;
use serde_json::Value;

use crate::serde_utils::val_handler::value_to_amqp_value;
use crate::argparse::argtype::{ArgType, arg_to_amqp_value, arg_to_value};


/// Structure storing the arguments
///
/// # Arguments
/// * `args` - A list of arguments
#[derive(Clone, Debug)]
pub struct Args{
    pub args: Vec<ArgType>,
}


/// Implementation of arguments list
impl Args{

    /// Convert args to a vector of `AmqpValue`
    pub fn args_to_amqp_vec(&self) -> Vec<AmqpValue>{
        let mut val_vec = Vec::<AmqpValue>::new();
        for i in 0..self.args.len(){
            let val = self.args.get(i).unwrap().clone();
            let amqval = arg_to_amqp_value(val);
            val_vec.push(amqval);
        }
        val_vec
    }

    /// Convert args to a vector of serde `Value` items
    pub fn args_to_vec(&self) -> Vec<Value>{
        let mut val_vec = Vec::<Value>::new();
        for i in 0..self.args.len(){
            let val = self.args.get(i).unwrap().clone();
            let serde_val = arg_to_value(val);
            val_vec.push(serde_val);
        }
        val_vec
    }

    /// Return the `usize` of the args
    pub fn size(&self) -> usize{
        self.args.len()
    }

    /// Add an argument (`Arg`)
    ///
    /// # Arguments
    /// * `arg` - An `Arg` representing an argument for rpc messages
    ///
    pub fn add_arg(&mut self, arg: ArgType){
        self.args.push(arg);
    }

    /// Create a new vector of arguments, an `Args` structure
    pub fn new() -> Args{
        Args{
           args: Vec::<ArgType>::new(),
        }
    }

}


#[cfg(test)]
mod tests{
    use amq_protocol::types::AMQPType;
    use serde_json::Value;

    use super::*;

    #[test]
    fn should_create_an_argument_list(){
        let test_string = String::from("test");
        let test_val = Value::from(test_string);
        let arg = ArgType::Double(32.0);
        let mut args = Args::new();
        args.args.push(arg);
        assert!(args.size() == 1);
        assert!(args.args.len() == 1);
        let at = args.args.get(0).unwrap().clone();
        let mut p = false;
        match at{
            ArgType::Double(at) => {
                p = true;
            }
            _ => {
                p = false;
            }
        }
        assert!(p);
    }
}
