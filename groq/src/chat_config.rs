use serde;
use serde::{Serialize, Deserialize};
use serde::{de,de::Unexpected};
use llm_core::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct GroqChatOptions {

    pub messages: Vec<Message>,  // Required

    pub model: GroqModelType,  // Required

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,  // Number between -2.0 and 2.0, default 0

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,  // Return log probabilities, default false

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,  // Maximum number of tokens that can be generated

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,  // Defaults to 1

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,  // Enable parallel function calling, default true

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,  // Number between -2.0 and 2.0, default 0

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // response_format: Option<ResponseFormat>,  // Specify output format

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,  // Best effort to sample deterministically

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,  // Up to 4 sequences where generation will stop

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,  // If set, partial message deltas will be sent, default false

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // stream_options: Option<StreamOptions>,  // Options for streaming response

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,  // Sampling temperature, default 1

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,  // Integer between 0 and 20 for the number of top tokens to return

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,  // Nucleus sampling, default 1

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,  // Unique identifier for the end-user
}

impl GroqChatOptions {
    pub fn default() -> Self {
        GroqChatOptions {
            frequency_penalty: Some(0.0),  // Default value 0.0
            logprobs: Some(false),  // Default value false
            max_tokens: None,  // Default to None
            messages: Vec::new(),  // Default to an empty list
            model: GroqModelType::Llama3_8b8192,  // Default model ID
            n: Some(1),  // Default value 1
            parallel_tool_calls: Some(false), // Defaults to false
            presence_penalty: Some(0.0),  // Default value 0.0
            seed: None,  // Default to None
            stop: None,  // Default to None
            stream: Some(false),  // Default value false
            temperature: Some(1.0),  // Default value 1.0
            top_logprobs: None,  // Default to None
            top_p: Some(1.0),  // Default value 1.0
            user: None,  // Default to None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GroqModelType {
    Llama3_70b8192,
    Llama3_8b8192
}

impl ToString for GroqModelType {
    fn to_string(&self) -> String {
        match self {
            GroqModelType::Llama3_8b8192 => String::from("llama3-8b-8192"),
            GroqModelType::Llama3_70b8192 => String::from("llama3-70b-8192")
        }
    }
}

impl Serialize for GroqModelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        S::serialize_str(serializer, &self.to_string()[..])
    }
}

impl<'de> Deserialize<'de> for GroqModelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        match &s[..] {
            "llama3-8b-8192" => Ok(GroqModelType::Llama3_8b8192),
            "llama3-70b-8192" => Ok(GroqModelType::Llama3_70b8192),
            _ => Err(de::Error::invalid_value(Unexpected::Str(&s), &"a valid custom string"))
        }
    }
}


// Currently Not supported.

// #[derive(Serialize, Deserialize, Debug)]
// struct ResponseFormat {
//     // Define response format properties here
//     format_type: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct StreamOptions {
//     // Define stream options properties here
//     option: String,
// }


#[cfg(test)]
mod tests {
    use super::*;
    use llm_core::message::Role;
    use serde_json;

    #[test]
    fn test_chat_options_to_json() {
        let chat_options = GroqChatOptions {
            frequency_penalty: Some(0.0),
            logprobs: Some(false),
            max_tokens: Some(100),
            messages: vec![Message::new("Hello".to_string(), Role::User)],
            model: GroqModelType::Llama3_8b8192,
            n: Some(1),
            parallel_tool_calls: Some(true),
            presence_penalty: Some(0.0),
            seed: None,
            stop: None,
            stream: Some(false),
            temperature: Some(1.0),
            top_logprobs: None,  // Optional integer between 0 and 20
            top_p: Some(1.0),  // Default value 1
            user: Some("user_123".to_string()),  // Optional unique user identifier
        };

        let json = serde_json::to_string(&chat_options).unwrap();
        assert_eq!(json, String::from("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"llama3-8b-8192\",\"frequency_penalty\":0.0,\"logprobs\":false,\"max_tokens\":100,\"n\":1,\"parallel_tool_calls\":true,\"presence_penalty\":0.0,\"stream\":false,\"temperature\":1.0,\"top_p\":1.0,\"user\":\"user_123\"}"));
    }
}