use groq::*;
use llm_core::message::{Message, Role};

fn main() {
    let system_message = Message::new("You are a pirate!".to_string(), Role::System);
    let user_message = Message::new("How are you doing?".to_string(), Role::User);
    println!("[{:?}]: {}", user_message.role,user_message.content);
    
    let mut chat_options = GroqChatOptions::default();
    chat_options.messages.push(system_message);
    chat_options.temperature = Some(0.5);

    let mut model = GroqModel::build_with_options(GroqModelType::Llama3_70b8192, chat_options).unwrap();
    let response = model.invoke(user_message).unwrap();
    println!("[{:?}]: {}", response.role,response.content);
    let user_message = Message::new("Tell me about the Pirate King, Gol D. Roger.".to_string(), Role::User);
    println!("[{:?}]: {}", user_message.role,user_message.content);
    let response = model.invoke(user_message).unwrap();
    println!("[{:?}]: {}", response.role,response.content);
}
