/*
Trait for implementing a backend

Author Andrew Evans
*/

pub trait Backend{
    fn get(&self);
    fn send_result(&self);
}
