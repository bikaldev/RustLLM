use serde::{Deserialize, Serialize};
use llm_core::message::{Message, Role};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
    pub logprobs: Option<LogProbs>,
}

impl Choice {
    pub fn empty() -> Choice {
        Choice 
        { 
            index: 0, 
            message: Message::new("".to_string(), Role::Assistant),
            finish_reason: "No Result Returned".to_string(),
            logprobs: None
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
    prompt_time: f64,
    completion_time: f64,
    total_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogProbs {
    value: Option<i32> // Placeholder type until it gets defined in the documentation
}