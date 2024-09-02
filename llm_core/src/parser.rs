use crate::message::Message;

pub trait Parser<ReturnType = String>{
    fn parse(&self, message: Message) -> ReturnType; 
}