use crate::message;

pub trait Model {
    fn invoke(&mut self, message:  message::Message) -> Result<message::Message, Box<dyn std::error::Error>>; 
}