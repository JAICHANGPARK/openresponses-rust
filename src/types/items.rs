use super::{content::*, enums::*};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Message {
        id: Option<String>,
        status: Option<MessageStatus>,
        role: MessageRole,
        content: Vec<MessageContent>,
    },
    FunctionCall {
        id: Option<String>,
        call_id: String,
        name: String,
        arguments: String,
        status: Option<FunctionCallStatus>,
    },
    FunctionCallOutput {
        id: Option<String>,
        call_id: String,
        output: FunctionOutput,
        status: Option<FunctionCallOutputStatus>,
    },
    Reasoning {
        id: Option<String>,
        status: Option<MessageStatus>,
        content: Option<Vec<MessageContent>>,
        summary: Vec<MessageContent>,
        encrypted_content: Option<String>,
    },
    ItemReference {
        id: String,
    },
    Extension {
        id: Option<String>,
        item_type: String,
        status: Option<String>,
        extra: Map<String, Value>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionOutput {
    Text(String),
    Content(Vec<MessageContent>),
}

impl Serialize for FunctionOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FunctionOutput::Text(text) => serializer.serialize_str(text),
            FunctionOutput::Content(content) => content.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for FunctionOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        parse_function_output(value).map_err(serde::de::Error::custom)
    }
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

    pub fn extension<S: Into<String>>(item_type: S, extra: Map<String, Value>) -> Self {
        Item::Extension {
            id: None,
            item_type: item_type.into(),
            status: None,
            extra,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        match self {
            Item::Message { role, content, .. } => validate_message_content(role, content),
            Item::FunctionCallOutput { output, .. } => validate_function_output(output),
            _ => Ok(()),
        }
    }
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match item_to_value(self) {
            Ok(value) => value.serialize(serializer),
            Err(error) => Err(serde::ser::Error::custom(error)),
        }
    }
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        item_from_value(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize)]
struct MessageItemRaw {
    id: Option<String>,
    status: Option<MessageStatus>,
    role: MessageRole,
    content: Value,
}

#[derive(Debug, Deserialize)]
struct FunctionCallRaw {
    id: Option<String>,
    call_id: String,
    name: String,
    arguments: String,
    status: Option<FunctionCallStatus>,
}

#[derive(Debug, Deserialize)]
struct FunctionCallOutputRaw {
    id: Option<String>,
    call_id: String,
    output: Value,
    status: Option<FunctionCallOutputStatus>,
}

#[derive(Debug, Deserialize)]
struct ReasoningRaw {
    id: Option<String>,
    status: Option<MessageStatus>,
    content: Option<Vec<MessageContent>>,
    summary: Vec<MessageContent>,
    encrypted_content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ItemReferenceRaw {
    id: String,
}

fn item_to_value(item: &Item) -> Result<Value, String> {
    match item {
        Item::Message {
            id,
            status,
            role,
            content,
        } => {
            validate_message_content(role, content)?;

            let mut object = Map::new();
            object.insert("type".to_string(), Value::String("message".to_string()));
            if let Some(id) = id {
                object.insert("id".to_string(), Value::String(id.clone()));
            }
            if let Some(status) = status {
                object.insert("status".to_string(), serde_json::to_value(status).map_err(|e| e.to_string())?);
            }
            object.insert("role".to_string(), serde_json::to_value(role).map_err(|e| e.to_string())?);
            object.insert(
                "content".to_string(),
                serde_json::to_value(content).map_err(|e| e.to_string())?,
            );
            Ok(Value::Object(object))
        }
        Item::FunctionCall {
            id,
            call_id,
            name,
            arguments,
            status,
        } => {
            let mut object = Map::new();
            object.insert("type".to_string(), Value::String("function_call".to_string()));
            if let Some(id) = id {
                object.insert("id".to_string(), Value::String(id.clone()));
            }
            object.insert("call_id".to_string(), Value::String(call_id.clone()));
            object.insert("name".to_string(), Value::String(name.clone()));
            object.insert("arguments".to_string(), Value::String(arguments.clone()));
            if let Some(status) = status {
                object.insert("status".to_string(), serde_json::to_value(status).map_err(|e| e.to_string())?);
            }
            Ok(Value::Object(object))
        }
        Item::FunctionCallOutput {
            id,
            call_id,
            output,
            status,
        } => {
            validate_function_output(output)?;

            let mut object = Map::new();
            object.insert(
                "type".to_string(),
                Value::String("function_call_output".to_string()),
            );
            if let Some(id) = id {
                object.insert("id".to_string(), Value::String(id.clone()));
            }
            object.insert("call_id".to_string(), Value::String(call_id.clone()));
            object.insert("output".to_string(), serde_json::to_value(output).map_err(|e| e.to_string())?);
            if let Some(status) = status {
                object.insert("status".to_string(), serde_json::to_value(status).map_err(|e| e.to_string())?);
            }
            Ok(Value::Object(object))
        }
        Item::Reasoning {
            id,
            status,
            content,
            summary,
            encrypted_content,
        } => {
            let mut object = Map::new();
            object.insert("type".to_string(), Value::String("reasoning".to_string()));
            if let Some(id) = id {
                object.insert("id".to_string(), Value::String(id.clone()));
            }
            if let Some(status) = status {
                object.insert("status".to_string(), serde_json::to_value(status).map_err(|e| e.to_string())?);
            }
            if let Some(content) = content {
                object.insert(
                    "content".to_string(),
                    serde_json::to_value(content).map_err(|e| e.to_string())?,
                );
            }
            object.insert(
                "summary".to_string(),
                serde_json::to_value(summary).map_err(|e| e.to_string())?,
            );
            if let Some(encrypted_content) = encrypted_content {
                object.insert(
                    "encrypted_content".to_string(),
                    Value::String(encrypted_content.clone()),
                );
            }
            Ok(Value::Object(object))
        }
        Item::ItemReference { id } => {
            let mut object = Map::new();
            object.insert("type".to_string(), Value::String("item_reference".to_string()));
            object.insert("id".to_string(), Value::String(id.clone()));
            Ok(Value::Object(object))
        }
        Item::Extension {
            id,
            item_type,
            status,
            extra,
        } => {
            let mut object = extra.clone();
            object.insert("type".to_string(), Value::String(item_type.clone()));
            if let Some(id) = id {
                object.insert("id".to_string(), Value::String(id.clone()));
            }
            if let Some(status) = status {
                object.insert("status".to_string(), Value::String(status.clone()));
            }
            Ok(Value::Object(object))
        }
    }
}

fn item_from_value(value: Value) -> Result<Item, String> {
    let object = match value {
        Value::Object(object) => object,
        _ => return Err("Item must be a JSON object".to_string()),
    };

    let item_type = match object.get("type").and_then(Value::as_str) {
        Some(item_type) => item_type.to_string(),
        None => return Err("Item is missing a string `type` field".to_string()),
    };

    match item_type.as_str() {
        "message" => {
            let raw: MessageItemRaw =
                serde_json::from_value(Value::Object(object)).map_err(|e| e.to_string())?;
            let content = parse_message_content(raw.content, &raw.role)?;
            validate_message_content(&raw.role, &content)?;

            Ok(Item::Message {
                id: raw.id,
                status: raw.status,
                role: raw.role,
                content,
            })
        }
        "function_call" => {
            let raw: FunctionCallRaw =
                serde_json::from_value(Value::Object(object)).map_err(|e| e.to_string())?;
            Ok(Item::FunctionCall {
                id: raw.id,
                call_id: raw.call_id,
                name: raw.name,
                arguments: raw.arguments,
                status: raw.status,
            })
        }
        "function_call_output" => {
            let raw: FunctionCallOutputRaw =
                serde_json::from_value(Value::Object(object)).map_err(|e| e.to_string())?;
            Ok(Item::FunctionCallOutput {
                id: raw.id,
                call_id: raw.call_id,
                output: parse_function_output(raw.output)?,
                status: raw.status,
            })
        }
        "reasoning" => {
            let raw: ReasoningRaw =
                serde_json::from_value(Value::Object(object)).map_err(|e| e.to_string())?;
            Ok(Item::Reasoning {
                id: raw.id,
                status: raw.status,
                content: raw.content,
                summary: raw.summary,
                encrypted_content: raw.encrypted_content,
            })
        }
        "item_reference" => {
            let raw: ItemReferenceRaw =
                serde_json::from_value(Value::Object(object)).map_err(|e| e.to_string())?;
            Ok(Item::ItemReference { id: raw.id })
        }
        _ => {
            let mut extra = object;
            extra.remove("type");
            let id = take_optional_string(&mut extra, "id")?;
            let status = take_optional_string(&mut extra, "status")?;

            Ok(Item::Extension {
                id,
                item_type,
                status,
                extra,
            })
        }
    }
}

fn parse_message_content(value: Value, role: &MessageRole) -> Result<Vec<MessageContent>, String> {
    match value {
        Value::String(text) => Ok(vec![string_content_for_role(role, text)]),
        Value::Array(_) => serde_json::from_value(value).map_err(|e| e.to_string()),
        _ => Err("Message content must be a string or an array of content parts".to_string()),
    }
}

fn parse_function_output(value: Value) -> Result<FunctionOutput, String> {
    if let Some(text) = value.as_str() {
        return Ok(FunctionOutput::Text(text.to_string()));
    }

    if value.is_array() {
        let content = serde_json::from_value(value).map_err(|e| e.to_string())?;
        let output = FunctionOutput::Content(content);
        validate_function_output(&output)?;
        return Ok(output);
    }

    Err("Function output must be a string or an array of content parts".to_string())
}

fn validate_function_output(output: &FunctionOutput) -> Result<(), String> {
    if let FunctionOutput::Content(content) = output {
        for part in content {
            if !matches!(
                part,
                MessageContent::InputText { .. }
                    | MessageContent::InputImage { .. }
                    | MessageContent::InputFile { .. }
                    | MessageContent::InputVideo { .. }
            ) {
                return Err(format!(
                    "Content type `{}` is not valid for `function_call_output` items",
                    content_type_name(part)
                ));
            }
        }
    }

    Ok(())
}

fn string_content_for_role(role: &MessageRole, text: String) -> MessageContent {
    match role {
        MessageRole::Assistant => MessageContent::output_text(text),
        MessageRole::User | MessageRole::System | MessageRole::Developer => {
            MessageContent::input_text(text)
        }
    }
}

fn validate_message_content(role: &MessageRole, content: &[MessageContent]) -> Result<(), String> {
    for part in content {
        if !is_allowed_message_content(role, part) {
            return Err(format!(
                "Content type `{}` is not valid for `{}` messages",
                content_type_name(part),
                role_name(role)
            ));
        }
    }

    Ok(())
}

fn is_allowed_message_content(role: &MessageRole, content: &MessageContent) -> bool {
    match role {
        MessageRole::User => matches!(
            content,
            MessageContent::InputText { .. }
                | MessageContent::InputImage { .. }
                | MessageContent::InputFile { .. }
                | MessageContent::InputVideo { .. }
        ),
        MessageRole::System | MessageRole::Developer => {
            matches!(content, MessageContent::InputText { .. })
        }
        MessageRole::Assistant => matches!(
            content,
            MessageContent::OutputText { .. } | MessageContent::Refusal { .. }
        ),
    }
}

fn content_type_name(content: &MessageContent) -> &'static str {
    match content {
        MessageContent::InputText { .. } => "input_text",
        MessageContent::InputImage { .. } => "input_image",
        MessageContent::InputFile { .. } => "input_file",
        MessageContent::InputVideo { .. } => "input_video",
        MessageContent::OutputText { .. } => "output_text",
        MessageContent::Refusal { .. } => "refusal",
        MessageContent::PlainText { .. } => "text",
        MessageContent::SummaryText { .. } => "summary_text",
        MessageContent::ReasoningText { .. } => "reasoning_text",
    }
}

fn role_name(role: &MessageRole) -> &'static str {
    match role {
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::System => "system",
        MessageRole::Developer => "developer",
    }
}

fn take_optional_string(object: &mut Map<String, Value>, key: &str) -> Result<Option<String>, String> {
    match object.remove(key) {
        Some(Value::String(value)) => Ok(Some(value)),
        Some(Value::Null) | None => Ok(None),
        Some(_) => Err(format!("`{key}` must be a string when present")),
    }
}
