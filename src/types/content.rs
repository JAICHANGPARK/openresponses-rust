use super::enums::ImageDetail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum InputContent {
    #[serde(rename = "input_text")]
    Text { text: String },
    #[serde(rename = "input_image")]
    Image {
        image_url: Option<String>,
        #[serde(default)]
        detail: ImageDetail,
    },
    #[serde(rename = "input_file")]
    File {
        filename: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_url: Option<String>,
    },
    #[serde(rename = "input_video")]
    Video { video_url: String },
}

impl InputContent {
    pub fn text<S: Into<String>>(text: S) -> Self {
        InputContent::Text { text: text.into() }
    }

    pub fn image_url<S: Into<String>>(url: S) -> Self {
        InputContent::Image {
            image_url: Some(url.into()),
            detail: ImageDetail::default(),
        }
    }

    pub fn image_url_with_detail<S: Into<String>>(url: S, detail: ImageDetail) -> Self {
        InputContent::Image {
            image_url: Some(url.into()),
            detail,
        }
    }

    pub fn file_url<S: Into<String>>(url: S) -> Self {
        InputContent::File {
            filename: None,
            file_data: None,
            file_url: Some(url.into()),
        }
    }

    pub fn file_data<S: Into<String>>(data: S, filename: Option<String>) -> Self {
        InputContent::File {
            filename,
            file_data: Some(data.into()),
            file_url: None,
        }
    }

    pub fn video_url<S: Into<String>>(url: S) -> Self {
        InputContent::Video {
            video_url: url.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum OutputContent {
    #[serde(rename = "output_text")]
    Text {
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

impl OutputContent {
    pub fn text<S: Into<String>>(text: S) -> Self {
        OutputContent::Text {
            text: text.into(),
            annotations: Vec::new(),
            logprobs: None,
        }
    }

    pub fn refusal<S: Into<String>>(text: S) -> Self {
        OutputContent::Refusal {
            refusal: text.into(),
        }
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
    Input(InputContent),
    Output(OutputContent),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ContentParam {
    Array(Vec<InputContent>),
    Single(String),
}
