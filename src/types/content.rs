use super::enums::ImageDetail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "input_text")]
    InputText { text: String },
    #[serde(rename = "input_image")]
    InputImage {
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(default)]
        detail: ImageDetail,
    },
    #[serde(rename = "input_file")]
    InputFile {
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_url: Option<String>,
    },
    #[serde(rename = "input_video")]
    InputVideo { video_url: String },
    #[serde(rename = "output_text")]
    OutputText {
        text: String,
        #[serde(default)]
        annotations: Vec<Annotation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<LogProb>>,
    },
    #[serde(rename = "refusal")]
    Refusal { refusal: String },
    #[serde(rename = "text")]
    PlainText { text: String },
    #[serde(rename = "summary_text")]
    SummaryText { text: String },
    #[serde(rename = "reasoning_text")]
    ReasoningText { text: String },
}

pub type InputContent = MessageContent;
pub type OutputContent = MessageContent;

impl MessageContent {
    pub fn input_text<S: Into<String>>(text: S) -> Self {
        MessageContent::InputText { text: text.into() }
    }

    pub fn output_text<S: Into<String>>(text: S) -> Self {
        MessageContent::OutputText {
            text: text.into(),
            annotations: Vec::new(),
            logprobs: None,
        }
    }

    pub fn refusal<S: Into<String>>(text: S) -> Self {
        MessageContent::Refusal {
            refusal: text.into(),
        }
    }

    // Compatibility helpers
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self::input_text(text)
    }

    pub fn image_url<S: Into<String>>(url: S) -> Self {
        MessageContent::InputImage {
            image_url: Some(url.into()),
            detail: ImageDetail::default(),
        }
    }

    pub fn image_url_with_detail<S: Into<String>>(url: S, detail: ImageDetail) -> Self {
        MessageContent::InputImage {
            image_url: Some(url.into()),
            detail,
        }
    }

    pub fn file_url<S: Into<String>>(url: S) -> Self {
        MessageContent::InputFile {
            filename: None,
            file_data: None,
            file_url: Some(url.into()),
        }
    }

    pub fn file_data<S: Into<String>>(data: S, filename: Option<String>) -> Self {
        MessageContent::InputFile {
            filename,
            file_data: Some(data.into()),
            file_url: None,
        }
    }

    pub fn video_url<S: Into<String>>(url: S) -> Self {
        MessageContent::InputVideo {
            video_url: url.into(),
        }
    }

    pub fn summary<S: Into<String>>(text: S) -> Self {
        MessageContent::SummaryText { text: text.into() }
    }

    pub fn reasoning<S: Into<String>>(text: S) -> Self {
        MessageContent::ReasoningText { text: text.into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Annotation {
    #[serde(rename = "url_citation")]
    UrlCitation {
        url: String,
        title: String,
        start_index: i32,
        end_index: i32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Vec<u8>,
    pub top_logprobs: Vec<TopLogProb>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopLogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Content {
    Part(MessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ContentParam {
    Array(Vec<MessageContent>),
    Single(String),
}