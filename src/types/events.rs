use super::{content::*, items::*, responses::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum StreamingEvent {
    #[serde(rename = "response.created")]
    ResponseCreated {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.queued")]
    ResponseQueued {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.in_progress")]
    ResponseInProgress {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.failed")]
    ResponseFailed {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        sequence_number: i32,
        output_index: i32,
        item: Item,
    },
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        sequence_number: i32,
        output_index: i32,
        item: Item,
    },
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        part: OutputContent,
    },
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        part: OutputContent,
    },
    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<LogProb>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<LogProb>>,
    },
    #[serde(rename = "response.refusal.delta")]
    RefusalDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
    },
    #[serde(rename = "response.refusal.done")]
    RefusalDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        refusal: String,
    },
    #[serde(rename = "response.reasoning.delta")]
    ReasoningDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.reasoning.done")]
    ReasoningDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        text: String,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ReasoningSummaryDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        delta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ReasoningSummaryDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        text: String,
    },
    #[serde(rename = "response.reasoning_summary_part.added")]
    ReasoningSummaryPartAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        part: OutputContent,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ReasoningSummaryPartDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        part: OutputContent,
    },
    #[serde(rename = "response.output_text.annotation.added")]
    OutputTextAnnotationAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        annotation_index: i32,
        annotation: Annotation,
    },
    #[serde(rename = "response.function_call_arguments.delta")]
    FunctionCallArgumentsDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        delta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    FunctionCallArgumentsDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        arguments: String,
    },
    #[serde(rename = "error")]
    Error {
        sequence_number: i32,
        error: ErrorPayload,
    },
    #[serde(rename = "[DONE]")]
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorPayload {
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

use std::collections::HashMap;
