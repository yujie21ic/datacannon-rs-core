//! Message body headers
//!
//! ---
//! author: Andrew Evans
//! ---

use std::collections::BTreeMap;

use crate::AmqpValue;
use serde_json::{Map, to_string, Value};

use crate::argparse::{args::Args, kwargs::KwArgs};
use crate::nodename::anon_name;


/// Soft and hard time limits
///
/// # Arguments
/// * `soft` - The soft time limit
/// * `hard` - The hard time limit
#[derive(Clone, Debug)]
pub struct TimeLimit {
    pub soft: i64,
    pub hard: i64,
}


/// Stored headers
///
/// # Arguments
/// * `lang` - The language extension name for running code
/// * `task` - The task name
/// * `id` - The unique task id
/// * `root_id` - The root id
/// * `parent_id` - The parent id
/// * `group` - Uuuid of the group being executed
/// * `meth` - Method name
/// * `shadow` - Loggin name alias
/// * `eta` - ETA for task arrival
/// * `expires` - When the task message expires (will not execute)
/// * `retries` - Number of times to retry sending the message
/// * `timelimit` - Hard and soft timelimit for the task
/// * `argsrepr` - Argument representation
/// * `kwargsrepr` - Mapped arguments
/// * `origin` - Option for the origin
#[derive(Clone, Debug)]
pub struct Headers{
    pub lang: String,
    pub task: String,
    pub id: String,
    pub root_id: String,
    pub parent_id: Option<String>,
    pub group: Option<String>,
    pub meth: Option<String>,
    pub shadow: Option<String>,
    pub eta: Option<String>,
    pub expires: Option<String>,
    pub retries: Option<i8>,
    pub timelimit: Option<TimeLimit>,
    pub argsrepr: Option<Args>,
    pub kwargsrepr: Option<KwArgs>,
    pub origin: Option<String>,
}


/// Headers implementation
impl Headers{

    /// Convert headers to a `std::collections::BTreeMap<std::string::String, crate::AmqpValue>`
    pub fn convert_to_btree_map(&self) -> BTreeMap<amq_protocol_types::ShortString, AmqpValue>{
        let mut jmap = BTreeMap::<amq_protocol_types::ShortString, AmqpValue>::new();
        jmap.insert(amq_protocol_types::ShortString::from(String::from("lang")), AmqpValue::LongString(amq_protocol_types::LongString::from(self.lang.clone())));
        jmap.insert(amq_protocol_types::ShortString::from(String::from("task")), AmqpValue::LongString(amq_protocol_types::LongString::from(self.task.clone())));
        jmap.insert(amq_protocol_types::ShortString::from(String::from("id")), AmqpValue::LongString(amq_protocol_types::LongString::from(self.id.clone())));
        jmap.insert(amq_protocol_types::ShortString::from(String::from("root_id")), AmqpValue::LongString(amq_protocol_types::LongString::from(self.root_id.clone())));

        if self.parent_id.is_some() {
            jmap.insert(amq_protocol_types::ShortString::from(String::from("parent_id")), AmqpValue::LongString(amq_protocol_types::LongString::from(self.parent_id.clone().unwrap())));
        }else {
            jmap.insert(amq_protocol_types::ShortString::from("parent_id".to_string()), AmqpValue::Void);
        }

        if self.group.is_some() {
            jmap.insert(amq_protocol_types::ShortString::from(String::from("group")), AmqpValue::LongString(amq_protocol_types::LongString::from(self.group.clone().unwrap())));
        }else{
            jmap.insert(amq_protocol_types::ShortString::from("group".to_string()), AmqpValue::Void);
        }

        if self.meth.is_some() {
            let v = self.meth.clone().unwrap();
            jmap.insert(amq_protocol_types::ShortString::from(String::from("meth")), AmqpValue::LongString(amq_protocol_types::LongString::from(v)));
        }

        if self.shadow.is_some(){
            let v = self.shadow.clone().unwrap();
            jmap.insert(amq_protocol_types::ShortString::from(String::from("shadow")), AmqpValue::LongString(amq_protocol_types::LongString::from(v)));
        }else{
            jmap.insert(amq_protocol_types::ShortString::from("shadow".to_string()), AmqpValue::Void);
        }

        if self.eta.is_some(){
            let v = self.eta.clone().unwrap();
            jmap.insert(amq_protocol_types::ShortString::from(String::from("eta")), AmqpValue::LongString(amq_protocol_types::LongString::from(v)));
        }else{
            jmap.insert(amq_protocol_types::ShortString::from("eta".to_string()), AmqpValue::Void);
        }

        if self.expires.is_some(){
            let v = self.expires.clone().unwrap();
            jmap.insert(amq_protocol_types::ShortString::from(String::from("expires")), AmqpValue::LongString(amq_protocol_types::LongString::from(v)));
        }else{
            jmap.insert(amq_protocol_types::ShortString::from("expires".to_string()), AmqpValue::Void);
        }

        if self.retries.is_some(){
            let v = self.retries.clone().unwrap();
            jmap.insert(amq_protocol_types::ShortString::from(String::from("retries")), AmqpValue::ShortShortInt(amq_protocol_types::ShortShortInt::from(v)));
        }else{
            jmap.insert(amq_protocol_types::ShortString::from("retries".to_string()), AmqpValue::Void);
        }

        if self.timelimit.is_some(){
            let v = self.timelimit.clone().unwrap();
            let vtup = vec![AmqpValue::LongLongInt(v.soft), AmqpValue::LongLongInt(v.hard)];
            jmap.insert(amq_protocol_types::ShortString::from(String::from("timelimit")), AmqpValue::FieldArray(amq_protocol_types::FieldArray::from(vtup)));
        }else{
            let vtup = vec![AmqpValue::Void, AmqpValue::Void];
            jmap.insert(amq_protocol_types::ShortString::from("timelimit".to_string()), AmqpValue::FieldArray(amq_protocol_types::FieldArray::from(vtup)));
        }

        if self.argsrepr.is_some(){
            let v = self.argsrepr.clone().unwrap();
            let val = v.args_to_vec();
            let jstr = to_string(&Value::Array(val));
            jmap.insert(amq_protocol_types::ShortString::from("argsrepr".to_string()), AmqpValue::LongString(amq_protocol_types::LongString::from(jstr.unwrap())));
        }else{
            let v = Value::from(Vec::<Value>::new());
            let vm = to_string(&v);
            jmap.insert(amq_protocol_types::ShortString::from("argsrepr".to_string()), AmqpValue::LongString(amq_protocol_types::LongString::from(vm.unwrap())));
        }

        if self.kwargsrepr.is_some(){
            let v = self.kwargsrepr.clone().unwrap();
            let vm = to_string(&Value::from(v.convert_to_map()));
            if vm.is_ok() {
                let jstr = vm.unwrap();
                jmap.insert(amq_protocol_types::ShortString::from(String::from("kwargsrepr")), AmqpValue::LongString(amq_protocol_types::LongString::from(jstr)));
            }
        }else{
            let v = Map::<String, Value>::new();
            let vm = to_string(&Value::from(v));
            if vm.is_ok() {
                let jstr = vm.unwrap();
                jmap.insert(amq_protocol_types::ShortString::from("kwargsrepr".to_string()), AmqpValue::LongString(amq_protocol_types::LongString::from(jstr)));
            }
        }


        if self.origin.is_some(){
            let v = self.origin.clone().unwrap();
            jmap.insert(amq_protocol_types::ShortString::from(String::from("origin")), AmqpValue::LongString(amq_protocol_types::LongString::from(v)));
        }else{
            let nodename = anon_name::get_anon_nodename(None, None);
            jmap.insert(amq_protocol_types::ShortString::from("origin".to_string()), AmqpValue::LongString(amq_protocol_types::LongString::from(nodename)));
        }
        jmap
    }

    /// Convert to a json capable map
    ///
    pub fn convert_to_json_map(&self) -> Map<String, Value>{
        let mut jmap = Map::new();
        jmap.insert(String::from("lang"), Value::String(self.lang.clone()));
        jmap.insert(String::from("task"), Value::String(self.task.clone()));
        jmap.insert(String::from("id"), Value::String(self.id.clone()));
        jmap.insert(String::from("root_id"), Value::String(self.root_id.clone()));

        if self.parent_id.is_some() {
            let parent_id = self.parent_id.clone().unwrap();
            jmap.insert(String::from("parent_id"), Value::String(parent_id));
        }

        if self.group.is_some() {
            let group = self.group.clone().unwrap();
            jmap.insert(String::from("group"), Value::String(group));
        }

        if self.meth.is_some() {
            let v = self.meth.clone().unwrap();
            jmap.insert(String::from("meth"), Value::from(v));
        }

        if self.shadow.is_some(){
            let v = self.shadow.clone().unwrap();
            jmap.insert(String::from("shadow"), Value::from(v));
        }

        if self.eta.is_some(){
            let v = self.eta.clone().unwrap();
            jmap.insert(String::from("eta"), Value::from(v));
        }

        if self.expires.is_some(){
            let v = self.expires.clone().unwrap();
            jmap.insert(String::from("expires"), Value::from(v));
        }

        if self.retries.is_some(){
            let v = self.retries.clone().unwrap();
            jmap.insert(String::from("retries"), Value::from(v));
        }

        if self.timelimit.is_some(){
            let v = self.timelimit.clone().unwrap();
            let vtup = vec![Value::from(v.soft), Value::from(v.hard)];
            jmap.insert(String::from("timelimit"), Value::Array(vtup));
        }

        if self.argsrepr.is_some(){
            let v = self.argsrepr.clone().unwrap();
            let argsrepr = v.args_to_vec();
            jmap.insert(String::from("args"), Value::Array(argsrepr));
        }

        if self.kwargsrepr.is_some(){
            let v = self.kwargsrepr.clone().unwrap();
            let vm = v.convert_to_map();
            jmap.insert(String::from("kwargsrepr"), Value::Object(vm));
        }


        if self.origin.is_some(){
            let v = self.origin.clone().unwrap();
            jmap.insert(String::from("origin"), Value::from(v));
        }
        jmap
    }

    /// create new headers
    pub fn new(lang: String, task: String, id: String, root_id: String) -> Headers{
        Headers{
            lang: lang,
            task: task,
            id: id,
            root_id: root_id,
            parent_id: None,
            group: None,
            meth: None,
            shadow: None,
            eta: None,
            expires: None,
            retries: None,
            timelimit: None,
            argsrepr: None,
            kwargsrepr: None,
            origin: None,
        }
    }
}


#[cfg(test)]
mod tests{
    use std::vec::Vec;

    use crate::argparse::args::{Args};
    use crate::message_protocol::headers::Headers;
    use crate::argparse::argtype::ArgType;

    #[test]
    fn should_convert_to_json_map(){
        let mut h = Headers::new(String::from("rs"), String::from("test_task"), String::from("id"), String::from("test_root"));
        let arep = Args{
            args: Vec::<ArgType>::new(),
        };
        h.argsrepr = Some(arep);
        let m = h.convert_to_json_map();
        let l = m.get("lang");
        assert!(String::from(l.unwrap().as_str().unwrap()).eq("rs"));
    }

    #[test]
    fn should_create_new_headers(){
        let mut h = Headers::new(String::from("rs"), String::from("test_task"), String::from("id"), String::from("test_root"));
        let arep = Args{
          args: Vec::<ArgType>::new(),
        };
        h.argsrepr = Some(arep);
        let lang = h.lang;
        let optrep = h.argsrepr;
        assert!(lang.eq("rs"));
        assert!(optrep.unwrap().args.len() == 0);
    }
}
