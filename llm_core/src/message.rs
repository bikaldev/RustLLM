use serde::{Serialize, Deserialize};


#[derive(Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    content: String,
    role: Role
}

impl Message {
    pub fn new(content: String, role: Role) -> Message {
        Message {content, role}
    }
}