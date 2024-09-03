use std::{error::Error as StdError, env};
use chat_completion::Choice;

pub mod chat_config;
pub mod chat_completion;

pub use chat_config::{GroqChatOptions, GroqModelType};
pub use chat_completion::ChatCompletion;
pub use llm_core::model::Model;
pub use llm_core::message::{Message, Role};

pub struct GroqModel {
    pub model: GroqModelType,
    pub chat_options: GroqChatOptions,
    chat_completion_url: String,
    api_key: String
}

impl GroqModel {

    fn link_and_api() -> Result<(String, String), std::env::VarError> {
        let chat_completion_url = "https://api.groq.com/openai/v1/chat/completions".to_string();
        let api_key = env::var("GROQ_API_KEY")?; 

        Ok((chat_completion_url, api_key))
    }

    pub fn build(model: GroqModelType) -> Result<Self, Box<dyn StdError>> {
        let mut chat_options = GroqChatOptions::default();
        chat_options.model = model;

        let (chat_completion_url, api_key) = GroqModel::link_and_api()?;

        Ok(GroqModel {model , chat_options, chat_completion_url, api_key})
    }

    pub fn build_with_options(model: GroqModelType, chat_options: GroqChatOptions) -> Result<Self, Box<dyn StdError>> {
        
        let (chat_completion_url, api_key) = GroqModel::link_and_api()?;

        Ok(GroqModel {model , chat_options, chat_completion_url, api_key})
    }
}

impl Model for GroqModel {
    fn invoke(&mut self, message:  Message) -> Result<Message, Box<dyn StdError>> {
        // create a request
        let client = reqwest::blocking::Client::new();
        self.chat_options.messages.push(message);

        let response = client.post(self.chat_completion_url.clone())
        .bearer_auth(self.api_key.clone())
        .json(&self.chat_options)
        .send()?;
        
        // let mut result = String::new();
        // let result = response.read_to_string(&mut result);
        let mut result: ChatCompletion = response.json()?;
        let choice = result.choices.pop().unwrap_or_else(
            || Choice::empty());
        
        let assistant_message = choice.message;

        self.chat_options.messages.push(assistant_message.clone());

        Ok(assistant_message)

    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use llm_core::message::Role;

    #[test]
    fn test_model_invoke() {
        let mut model = GroqModel::build(GroqModelType::Llama3_8b8192).unwrap();
        let message = Message::new("Hello, how are you doing?".to_string(), Role::User);
        let response = model.invoke(message).unwrap();
        println!("The returned message is: {:?}", response);
    }
}