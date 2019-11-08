/*
Arg types for rust

Author Andrew Evans
*/


use std::collections::HashMap;

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
}


