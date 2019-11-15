//! Mapped Kwargs for RPC in the framework
//!
//! ---
//! author: Andrew Evans
//! ---


use std::collections::{BTreeMap, HashMap};
use std::vec::Vec;

use amiquip::AmqpValue;
use amq_protocol::types::AMQPType;
use serde_json::{Map, Value};
use crate::argparse::argtype::{ArgType, arg_to_amqp_value, arg_to_value};


/// Keyword argument
#[derive(Clone, Debug)]
pub struct KwArg{
    pub key: String,
    pub arg: ArgType,
}


/// Structure storing the arguments
#[derive(Clone, Debug)]
pub struct KwArgs{
    pub kwargs: Vec<KwArg>,
}


/// Argument implementation
impl KwArg{

    /// Create a new argument
    fn new(key: String, arg: ArgType) -> KwArg{
        KwArg{
            key: key,
            arg: arg,
        }
    }
}


/// Implementation of arguments list
impl KwArgs{

    /// Convert to a btree map
    pub fn convert_to_btree_map(&self) -> BTreeMap<String, AmqpValue>{
        let vkwarr = self.kwargs.clone();
        let mut vm = BTreeMap::<String, AmqpValue>::new();
        for i in 0..vkwarr.len(){
            let kwarg = vkwarr.get(i).unwrap().clone();
            let kwv = arg_to_amqp_value(kwarg.arg);
            vm.insert(kwarg.key, kwv);
        }
        vm
    }

    /// Covnert kwargs to map
    pub fn convert_to_map(&self) -> Map<String, Value>{
        let vkwarr = self.kwargs.clone();
        let mut vm = Map::new();
        for i in 0..vkwarr.len(){
            let kwarg = vkwarr.get(i).unwrap().clone();
            let val = arg_to_value(kwarg.arg);
            let key = kwarg.key;
            vm.insert(key, val);
        }
        vm
    }

    /// size of the list
    pub fn size(&self) -> usize{
        self.kwargs.len()
    }

    /// add an argument
    pub fn add_kwarg(&mut self, kwarg: KwArg){
        self.kwargs.push(kwarg);
    }

    /// create a new arguments list
    pub fn new() -> KwArgs{
        KwArgs{
            kwargs: Vec::<KwArg>::new(),
        }
    }

}


#[cfg(test)]
mod tests{
    use amq_protocol::types::AMQPType;
    use serde_json::Value;

    use super::*;

    #[test]
    fn should_create_an_kwarg_argument(){
        let kstr = String::from("test_key");
        let kvalstr = String::from("test_val");
        let kval = Value::from(kvalstr);
        let arg = ArgType::String("test".to_string());
        let kwarg = KwArg{
            key: kstr.to_owned(),
            arg: arg.to_owned(),
        };
        let k = kwarg.key;
        let v = arg_to_value(kwarg.arg);
        assert!(k.eq("test_key"));
        assert!(v.as_str().unwrap().eq("test"));
    }

    #[test]
    fn should_create_a_kwargs_list(){
        let kstr = String::from("test_key");
        let arg= ArgType::String("test_val".to_string());
        let kwarg = KwArg{
            key: kstr.to_owned(),
            arg: arg.to_owned(),
        };
        let kstrb = String::from("test_keyb");
        let argb = ArgType::String("test_val".to_string());
        let kwargb = KwArg{
            key: kstrb.to_owned(),
            arg: argb.clone(),
        };
        let mut kwargs = KwArgs::new();
        kwargs.add_kwarg(kwarg.clone());
        kwargs.add_kwarg(kwargb.clone());
        assert!(kwargs.size() == 2);
        assert!(kwargs.kwargs.len() == 2);
    }
}
