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

    pub fn with_description<S: Into<String>>(mut self, desc: S) -> Self {
        let Tool::Function { description, .. } = &mut self;
        *description = Some(desc.into());
        self
    }

    pub fn with_parameters(mut self, params: serde_json::Value) -> Self {
        let Tool::Function { parameters, .. } = &mut self;
        *parameters = Some(params);
        self
    }

    pub fn strict(mut self, strict: bool) -> Self {
        let Tool::Function { strict: s, .. } = &mut self;
        *s = Some(strict);
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
