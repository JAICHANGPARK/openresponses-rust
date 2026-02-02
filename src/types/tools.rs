use super::enums::ToolChoice;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Tool {
    #[serde(rename = "function")]
    Function {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
    #[serde(rename = "mcp")]
    Mcp {
        server_label: String,
        server_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_tools: Option<Vec<String>>,
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

    pub fn with_parameters(mut self, params: serde_json::Value) -> Self {
        if let Tool::Function { parameters, .. } = &mut self {
            *parameters = Some(params);
        }
        self
    }

    pub fn strict(mut self, strict: bool) -> Self {
        if let Tool::Function { strict: s, .. } = &mut self {
            *s = Some(strict);
        }
        self
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
        mode: ToolChoice,
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
    pub parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(rename = "type")]
    pub tool_type: String,
}
