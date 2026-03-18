use super::enums::ToolChoice;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Tool {
    Function {
        name: String,
        description: Option<String>,
        parameters: Option<Value>,
        strict: Option<bool>,
    },
    Mcp {
        server_label: String,
        server_url: String,
        allowed_tools: Option<Vec<String>>,
    },
    Extension {
        tool_type: String,
        extra: Map<String, Value>,
    },
}

impl Tool {
    pub fn function<S: Into<String>>(name: S) -> Self {
        Tool::Function {
            name: name.into(),
            description: None,
            parameters: None,
            strict: None,
        }
    }

    pub fn mcp<S1: Into<String>, S2: Into<String>>(label: S1, url: S2) -> Self {
        Tool::Mcp {
            server_label: label.into(),
            server_url: url.into(),
            allowed_tools: None,
        }
    }

    pub fn extension<S: Into<String>>(tool_type: S, extra: Map<String, Value>) -> Self {
        Tool::Extension {
            tool_type: tool_type.into(),
            extra,
        }
    }

    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        if let Tool::Mcp { allowed_tools, .. } = &mut self {
            *allowed_tools = Some(tools);
        }
        self
    }

    pub fn with_description<S: Into<String>>(mut self, desc: S) -> Self {
        if let Tool::Function { description, .. } = &mut self {
            *description = Some(desc.into());
        }
        self
    }

    pub fn with_parameters(mut self, params: Value) -> Self {
        if let Tool::Function { parameters, .. } = &mut self {
            *parameters = Some(params);
        }
        self
    }

    pub fn strict(mut self, strict: bool) -> Self {
        if let Tool::Function { strict: value, .. } = &mut self {
            *value = Some(strict);
        }
        self
    }
}

impl Serialize for Tool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match tool_to_value(self) {
            Ok(value) => value.serialize(serializer),
            Err(error) => Err(serde::ser::Error::custom(error)),
        }
    }
}

impl<'de> Deserialize<'de> for Tool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        tool_from_value(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize)]
struct FunctionToolRaw {
    name: String,
    description: Option<String>,
    parameters: Option<Value>,
    strict: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct McpToolRaw {
    server_label: String,
    server_url: String,
    allowed_tools: Option<Vec<String>>,
}

fn tool_to_value(tool: &Tool) -> Result<Value, String> {
    match tool {
        Tool::Function {
            name,
            description,
            parameters,
            strict,
        } => {
            let mut object = Map::new();
            object.insert("type".to_string(), Value::String("function".to_string()));
            object.insert("name".to_string(), Value::String(name.clone()));
            if let Some(description) = description {
                object.insert("description".to_string(), Value::String(description.clone()));
            }
            if let Some(parameters) = parameters {
                object.insert("parameters".to_string(), parameters.clone());
            }
            if let Some(strict) = strict {
                object.insert("strict".to_string(), Value::Bool(*strict));
            }
            Ok(Value::Object(object))
        }
        Tool::Mcp {
            server_label,
            server_url,
            allowed_tools,
        } => {
            let mut object = Map::new();
            object.insert("type".to_string(), Value::String("mcp".to_string()));
            object.insert(
                "server_label".to_string(),
                Value::String(server_label.clone()),
            );
            object.insert("server_url".to_string(), Value::String(server_url.clone()));
            if let Some(allowed_tools) = allowed_tools {
                object.insert(
                    "allowed_tools".to_string(),
                    serde_json::to_value(allowed_tools).map_err(|e| e.to_string())?,
                );
            }
            Ok(Value::Object(object))
        }
        Tool::Extension { tool_type, extra } => {
            let mut object = extra.clone();
            object.insert("type".to_string(), Value::String(tool_type.clone()));
            Ok(Value::Object(object))
        }
    }
}

fn tool_from_value(value: Value) -> Result<Tool, String> {
    let object = match value {
        Value::Object(object) => object,
        _ => return Err("Tool must be a JSON object".to_string()),
    };

    let tool_type = match object.get("type").and_then(Value::as_str) {
        Some(tool_type) => tool_type.to_string(),
        None => return Err("Tool is missing a string `type` field".to_string()),
    };

    match tool_type.as_str() {
        "function" => {
            let raw: FunctionToolRaw =
                serde_json::from_value(Value::Object(object)).map_err(|e| e.to_string())?;
            Ok(Tool::Function {
                name: raw.name,
                description: raw.description,
                parameters: raw.parameters,
                strict: raw.strict,
            })
        }
        "mcp" => {
            let raw: McpToolRaw =
                serde_json::from_value(Value::Object(object)).map_err(|e| e.to_string())?;
            Ok(Tool::Mcp {
                server_label: raw.server_label,
                server_url: raw.server_url,
                allowed_tools: raw.allowed_tools,
            })
        }
        _ => {
            let mut extra = object;
            extra.remove("type");
            Ok(Tool::Extension { tool_type, extra })
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolChoiceParam {
    Simple(ToolChoice),
    Specific {
        #[serde(rename = "type")]
        tool_type: String,
        name: String,
    },
    Allowed {
        #[serde(rename = "type")]
        allowed_type: String,
        tools: Vec<SpecificTool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<ToolChoice>,
    },
}

impl Default for ToolChoiceParam {
    fn default() -> Self {
        ToolChoiceParam::Simple(ToolChoice::Auto)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpecificTool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionToolParam {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(rename = "type")]
    pub tool_type: String,
}
