use eventsource_stream::{Event, Eventsource};
use futures::{Stream, StreamExt};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
    Client as ReqwestClient,
};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

use crate::types::{
    ApiErrorResponse, CreateResponseBody, Item, MessageStatus, StreamingEvent,
};

const DEFAULT_BASE_URL: &str = "https://api.openai.com";

#[derive(Error, Debug)]
pub enum StreamingError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Event stream error: {0}")]
    StreamError(String),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error ({status_code}): {raw_body}")]
    ApiError {
        status_code: u16,
        error: Option<crate::types::ApiErrorDetail>,
        raw_body: String,
    },

    #[error("SSE event name `{sse_event}` does not match payload type `{body_type}`")]
    EventTypeMismatch { sse_event: String, body_type: String },

    #[error("Streaming payload type `{body_type}` is missing a matching SSE event name")]
    MissingEventType { body_type: String },

    #[error("Invalid event lifecycle: {message}")]
    LifecycleError { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawSseEvent {
    pub event: Option<String>,
    pub data: String,
}

pub struct StreamingClientBuilder {
    api_key: String,
    base_url: Option<String>,
}

impl StreamingClientBuilder {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: None,
        }
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn build(self) -> StreamingClient {
        let mut base_url = self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        if base_url.ends_with('/') {
            base_url.pop();
        }

        if !base_url.ends_with("/v1") {
            base_url.push_str("/v1");
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(ACCEPT, HeaderValue::from_static("text/event-stream"));

        let inner = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        StreamingClient {
            inner,
            base_url,
            api_key: self.api_key,
        }
    }
}

#[derive(Clone)]
pub struct StreamingClient {
    inner: ReqwestClient,
    base_url: String,
    api_key: String,
}

impl StreamingClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        StreamingClientBuilder::new(api_key).build()
    }

    pub fn builder(api_key: impl Into<String>) -> StreamingClientBuilder {
        StreamingClientBuilder::new(api_key)
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        StreamingClientBuilder::new(api_key)
            .base_url(base_url)
            .build()
    }

    pub async fn stream_response(
        &self,
        mut request: CreateResponseBody,
    ) -> Result<impl Stream<Item = Result<StreamingEvent, StreamingError>>, StreamingError> {
        request.stream = Some(true);

        let url = format!("{}/responses", self.base_url);

        let response = self
            .inner
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(StreamingError::ApiError {
                status_code: status.as_u16(),
                error: ApiErrorResponse::parse(&error_text),
                raw_body: error_text,
            });
        }

        let stream = response.bytes_stream();
        let eventsource = stream.eventsource();
        let mut validator = StreamValidator::default();

        let event_stream = eventsource.map(move |event| match event {
            Ok(event) => parse_streaming_event(event, &mut validator),
            Err(error) => Err(StreamingError::StreamError(error.to_string())),
        });

        Ok(event_stream)
    }

    pub async fn stream_response_lines(
        &self,
        mut request: CreateResponseBody,
    ) -> Result<impl Stream<Item = Result<RawSseEvent, StreamingError>>, StreamingError> {
        request.stream = Some(true);

        let url = format!("{}/responses", self.base_url);

        let response = self
            .inner
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(StreamingError::ApiError {
                status_code: status.as_u16(),
                error: ApiErrorResponse::parse(&error_text),
                raw_body: error_text,
            });
        }

        let stream = response.bytes_stream();
        let eventsource = stream.eventsource();

        let line_stream = eventsource.map(|event| match event {
            Ok(event) => {
                let event_name = if event.data == "[DONE]" || event.event.is_empty() {
                    None
                } else {
                    Some(event.event)
                };

                Ok(RawSseEvent {
                    event: event_name,
                    data: event.data,
                })
            }
            Err(error) => Err(StreamingError::StreamError(error.to_string())),
        });

        Ok(line_stream)
    }
}

fn parse_streaming_event(
    event: Event,
    validator: &mut StreamValidator,
) -> Result<StreamingEvent, StreamingError> {
    if event.data == "[DONE]" {
        validator.observe(&StreamingEvent::Done)?;
        return Ok(StreamingEvent::Done);
    }

    let raw_value: Value = serde_json::from_str(&event.data)?;
    let body_type = raw_value
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| StreamingError::StreamError("Streaming payload is missing `type`".to_string()))?
        .to_string();

    if event.event.is_empty() {
        return Err(StreamingError::MissingEventType { body_type });
    }

    if event.event != body_type {
        return Err(StreamingError::EventTypeMismatch {
            sse_event: event.event,
            body_type,
        });
    }

    let parsed = serde_json::from_value::<StreamingEvent>(raw_value)?;
    validator.observe(&parsed)?;
    Ok(parsed)
}

#[derive(Default)]
struct StreamValidator {
    outputs: HashMap<i32, OutputItemState>,
    item_to_output: HashMap<String, i32>,
    terminal_incomplete_output: Option<i32>,
    saw_stream_error: bool,
}

#[derive(Default)]
struct OutputItemState {
    done: bool,
    content_parts: HashMap<i32, ContentPartState>,
}

#[derive(Default)]
struct ContentPartState {
    done: bool,
}

impl StreamValidator {
    fn observe(&mut self, event: &StreamingEvent) -> Result<(), StreamingError> {
        if let Some(terminal_output) = self.terminal_incomplete_output {
            if let Some(output_index) = output_index_for_event(event) {
                if output_index > terminal_output {
                    return Err(StreamingError::LifecycleError {
                        message: format!(
                            "received output event for index {output_index} after incomplete terminal item {terminal_output}"
                        ),
                    });
                }
            }
        }

        match event {
            StreamingEvent::OutputItemAdded {
                output_index,
                item,
                ..
            } => {
                if self.outputs.contains_key(output_index) {
                    return Err(StreamingError::LifecycleError {
                        message: format!("output item {output_index} was added more than once"),
                    });
                }

                self.outputs.insert(*output_index, OutputItemState::default());
                if let Some(item) = item {
                    self.register_item(item, *output_index)?;
                }
            }
            StreamingEvent::OutputItemDone {
                output_index,
                item,
                ..
            } => {
                if let Some(item) = item {
                    self.register_item(item, *output_index)?;
                    if item_is_incomplete(item) {
                        self.terminal_incomplete_output = Some(*output_index);
                    }
                }

                let state = self.output_mut(*output_index)?;
                if state.content_parts.values().any(|part| !part.done) {
                    return Err(StreamingError::LifecycleError {
                        message: format!(
                            "output item {output_index} finished before all content parts were closed"
                        ),
                    });
                }
                state.done = true;
            }
            StreamingEvent::ContentPartAdded {
                item_id,
                output_index,
                content_index,
                ..
            } => {
                let state = self.output_for_item(item_id, *output_index)?;
                if state.done {
                    return Err(StreamingError::LifecycleError {
                        message: format!(
                            "content part {content_index} was added after output item {output_index} finished"
                        ),
                    });
                }
                if state.content_parts.contains_key(content_index) {
                    return Err(StreamingError::LifecycleError {
                        message: format!(
                            "content part {content_index} for output item {output_index} was added more than once"
                        ),
                    });
                }
                state.content_parts.insert(*content_index, ContentPartState::default());
            }
            StreamingEvent::ContentPartDone {
                item_id,
                output_index,
                content_index,
                ..
            } => {
                let part = self.content_part_mut(item_id, *output_index, *content_index)?;
                part.done = true;
            }
            StreamingEvent::OutputTextDelta {
                item_id,
                output_index,
                content_index,
                ..
            }
            | StreamingEvent::OutputTextDone {
                item_id,
                output_index,
                content_index,
                ..
            }
            | StreamingEvent::RefusalDelta {
                item_id,
                output_index,
                content_index,
                ..
            }
            | StreamingEvent::RefusalDone {
                item_id,
                output_index,
                content_index,
                ..
            }
            | StreamingEvent::ReasoningDelta {
                item_id,
                output_index,
                content_index,
                ..
            }
            | StreamingEvent::ReasoningDone {
                item_id,
                output_index,
                content_index,
                ..
            }
            | StreamingEvent::OutputTextAnnotationAdded {
                item_id,
                output_index,
                content_index,
                ..
            } => {
                self.content_part_mut(item_id, *output_index, *content_index)?;
            }
            StreamingEvent::FunctionCallArgumentsDelta {
                item_id,
                output_index,
                ..
            }
            | StreamingEvent::FunctionCallArgumentsDone {
                item_id,
                output_index,
                ..
            } => {
                let state = self.output_for_item(item_id, *output_index)?;
                if state.done {
                    return Err(StreamingError::LifecycleError {
                        message: format!(
                            "function call arguments arrived after output item {output_index} finished"
                        ),
                    });
                }
            }
            StreamingEvent::Error { .. } => {
                self.saw_stream_error = true;
            }
            StreamingEvent::ResponseFailed { .. } => {
                self.saw_stream_error = false;
            }
            StreamingEvent::Done => {
                if self.saw_stream_error {
                    return Err(StreamingError::LifecycleError {
                        message: "stream ended after `error` without a matching `response.failed` event"
                            .to_string(),
                    });
                }
            }
            StreamingEvent::Unknown { .. }
            | StreamingEvent::ResponseCreated { .. }
            | StreamingEvent::ResponseQueued { .. }
            | StreamingEvent::ResponseInProgress { .. }
            | StreamingEvent::ResponseCompleted { .. }
            | StreamingEvent::ResponseIncomplete { .. }
            | StreamingEvent::ReasoningSummaryDelta { .. }
            | StreamingEvent::ReasoningSummaryDone { .. }
            | StreamingEvent::ReasoningSummaryPartAdded { .. }
            | StreamingEvent::ReasoningSummaryPartDone { .. } => {}
        }

        Ok(())
    }

    fn register_item(&mut self, item: &Item, output_index: i32) -> Result<(), StreamingError> {
        if let Some(item_id) = item_id(item) {
            if let Some(existing) = self.item_to_output.get(item_id) {
                if *existing != output_index {
                    return Err(StreamingError::LifecycleError {
                        message: format!(
                            "item `{item_id}` was associated with output index {existing}, not {output_index}"
                        ),
                    });
                }
            } else {
                self.item_to_output.insert(item_id.to_string(), output_index);
            }
        }

        Ok(())
    }

    fn output_mut(&mut self, output_index: i32) -> Result<&mut OutputItemState, StreamingError> {
        self.outputs
            .get_mut(&output_index)
            .ok_or_else(|| StreamingError::LifecycleError {
                message: format!(
                    "received an event for output item {output_index} before `response.output_item.added`"
                ),
            })
    }

    fn output_for_item(
        &mut self,
        item_id: &str,
        output_index: i32,
    ) -> Result<&mut OutputItemState, StreamingError> {
        if let Some(existing) = self.item_to_output.get(item_id) {
            if *existing != output_index {
                return Err(StreamingError::LifecycleError {
                    message: format!(
                        "item `{item_id}` was associated with output index {existing}, not {output_index}"
                    ),
                });
            }
        } else {
            self.item_to_output.insert(item_id.to_string(), output_index);
        }

        self.output_mut(output_index)
    }

    fn content_part_mut(
        &mut self,
        item_id: &str,
        output_index: i32,
        content_index: i32,
    ) -> Result<&mut ContentPartState, StreamingError> {
        let state = self.output_for_item(item_id, output_index)?;
        if state.done {
            return Err(StreamingError::LifecycleError {
                message: format!(
                    "content update arrived after output item {output_index} finished"
                ),
            });
        }

        let part = state
            .content_parts
            .get_mut(&content_index)
            .ok_or_else(|| StreamingError::LifecycleError {
                message: format!(
                    "received a content update for output item {output_index} / part {content_index} before `response.content_part.added`"
                ),
            })?;

        if part.done {
            return Err(StreamingError::LifecycleError {
                message: format!(
                    "received a content update for output item {output_index} / part {content_index} after `response.content_part.done`"
                ),
            });
        }

        Ok(part)
    }
}

fn output_index_for_event(event: &StreamingEvent) -> Option<i32> {
    match event {
        StreamingEvent::OutputItemAdded { output_index, .. }
        | StreamingEvent::OutputItemDone { output_index, .. }
        | StreamingEvent::ContentPartAdded { output_index, .. }
        | StreamingEvent::ContentPartDone { output_index, .. }
        | StreamingEvent::OutputTextDelta { output_index, .. }
        | StreamingEvent::OutputTextDone { output_index, .. }
        | StreamingEvent::RefusalDelta { output_index, .. }
        | StreamingEvent::RefusalDone { output_index, .. }
        | StreamingEvent::ReasoningDelta { output_index, .. }
        | StreamingEvent::ReasoningDone { output_index, .. }
        | StreamingEvent::ReasoningSummaryDelta { output_index, .. }
        | StreamingEvent::ReasoningSummaryDone { output_index, .. }
        | StreamingEvent::ReasoningSummaryPartAdded { output_index, .. }
        | StreamingEvent::ReasoningSummaryPartDone { output_index, .. }
        | StreamingEvent::OutputTextAnnotationAdded { output_index, .. }
        | StreamingEvent::FunctionCallArgumentsDelta { output_index, .. }
        | StreamingEvent::FunctionCallArgumentsDone { output_index, .. } => Some(*output_index),
        _ => None,
    }
}

fn item_id(item: &Item) -> Option<&str> {
    match item {
        Item::Message { id, .. }
        | Item::FunctionCall { id, .. }
        | Item::FunctionCallOutput { id, .. }
        | Item::Reasoning { id, .. }
        | Item::Extension { id, .. } => id.as_deref(),
        Item::ItemReference { id } => Some(id.as_str()),
    }
}

fn item_is_incomplete(item: &Item) -> bool {
    match item {
        Item::Message { status, .. } => matches!(status, Some(MessageStatus::Incomplete)),
        Item::FunctionCall { status, .. } => {
            matches!(status, Some(crate::types::FunctionCallStatus::Incomplete))
        }
        Item::FunctionCallOutput { status, .. } => {
            matches!(status, Some(crate::types::FunctionCallOutputStatus::Incomplete))
        }
        Item::Reasoning { status, .. } => matches!(status, Some(MessageStatus::Incomplete)),
        Item::Extension { status, .. } => status.as_deref() == Some("incomplete"),
        Item::ItemReference { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_client_creation() {
        let client = StreamingClient::new("test-api-key");
        assert_eq!(client.api_key, "test-api-key");
        assert_eq!(client.base_url, "https://api.openai.com/v1");
    }

    #[test]
    fn test_streaming_client_with_base_url_normalization() {
        let client = StreamingClient::with_base_url("test-key", "https://openrouter.ai/api");
        assert_eq!(client.base_url, "https://openrouter.ai/api/v1");
    }
}
