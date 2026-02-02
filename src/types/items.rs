use super::{content::*, enums::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "message")]
    Message {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<MessageStatus>,
        role: MessageRole,
        #[serde(with = "content_serde")]
        content: Vec<MessageContent>,
    },
    #[serde(rename = "function_call")]
    FunctionCall {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        call_id: String,
        name: String,
        arguments: String,
        status: FunctionCallStatus,
    },
    #[serde(rename = "function_call_output")]
    FunctionCallOutput {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        call_id: String,
        #[serde(with = "output_serde")]
        output: FunctionOutput,
        status: FunctionCallOutputStatus,
    },
    #[serde(rename = "reasoning")]
    Reasoning {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Vec<MessageContent>>,
        summary: Vec<MessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        encrypted_content: Option<String>,
    },
    #[serde(rename = "item_reference")]
    ItemReference { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FunctionOutput {
    Text(String),
    Content(Vec<MessageContent>),
}

impl Item {
    pub fn user_message<S: Into<String>>(content: S) -> Self {
        Item::Message {
            id: None,
            status: None,
            role: MessageRole::User,
            content: vec![MessageContent::input_text(content)],
        }
    }

    pub fn user_message_with_content(content: Vec<MessageContent>) -> Self {
        Item::Message {
            id: None,
            status: None,
            role: MessageRole::User,
            content,
        }
    }

    pub fn assistant_message<S: Into<String>>(content: S) -> Self {
        Item::Message {
            id: None,
            status: None,
            role: MessageRole::Assistant,
            content: vec![MessageContent::output_text(content)],
        }
    }

    pub fn system_message<S: Into<String>>(content: S) -> Self {
        Item::Message {
            id: None,
            status: None,
            role: MessageRole::System,
            content: vec![MessageContent::input_text(content)],
        }
    }

    pub fn developer_message<S: Into<String>>(content: S) -> Self {
        Item::Message {
            id: None,
            status: None,
            role: MessageRole::Developer,
            content: vec![MessageContent::input_text(content)],
        }
    }

    pub fn reference<S: Into<String>>(id: S) -> Self {
        Item::ItemReference { id: id.into() }
    }
}

mod content_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        content: &Vec<MessageContent>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        content.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<MessageContent>, D::Error> {
        let content = Deserialize::deserialize(deserializer)?;
        Ok(content)
    }
}

mod output_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        output: &FunctionOutput,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match output {
            FunctionOutput::Text(s) => serializer.serialize_str(s),
            FunctionOutput::Content(c) => c.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<FunctionOutput, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;

        if let Some(s) = value.as_str() {
            return Ok(FunctionOutput::Text(s.to_string()));
        }

        if let Ok(contents) = serde_json::from_value::<Vec<MessageContent>>(value.clone()) {
            return Ok(FunctionOutput::Content(contents));
        }

        Err(serde::de::Error::custom("Invalid function output format"))
    }
}
