use serde::{Serialize, Deserialize};
use serde::{de, de::Unexpected};


#[derive(Debug, Clone)]
pub enum Role {
    System,
    User,
    Assistant
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::System => String::from("system"),
            Role::User => String::from("user"),
            Role::Assistant => String::from("assistant")
        }
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        S::serialize_str(serializer, &self.to_string()[..])
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        match &s[..] {
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            "system" => Ok(Role::System),
            _ => Err(de::Error::invalid_value(Unexpected::Str(&s), &"a valid custom string"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub content: String,
    pub role: Role
}

impl Message {
    pub fn new(content: String, role: Role) -> Message {
        Message {content, role}
    }
}