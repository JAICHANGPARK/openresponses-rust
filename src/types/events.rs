use super::{content::*, items::*, responses::*};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum StreamingEvent {
    ResponseCreated {
        sequence_number: i32,
        response: ResponseResource,
    },
    ResponseQueued {
        sequence_number: i32,
        response: ResponseResource,
    },
    ResponseInProgress {
        sequence_number: i32,
        response: ResponseResource,
    },
    ResponseCompleted {
        sequence_number: i32,
        response: ResponseResource,
    },
    ResponseFailed {
        sequence_number: i32,
        response: ResponseResource,
    },
    ResponseIncomplete {
        sequence_number: i32,
        response: ResponseResource,
    },
    OutputItemAdded {
        sequence_number: i32,
        output_index: i32,
        item: Option<Item>,
    },
    OutputItemDone {
        sequence_number: i32,
        output_index: i32,
        item: Option<Item>,
    },
    ContentPartAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        part: MessageContent,
    },
    ContentPartDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        part: MessageContent,
    },
    OutputTextDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
        logprobs: Option<Vec<LogProb>>,
        obfuscation: Option<String>,
    },
    OutputTextDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        text: String,
        logprobs: Option<Vec<LogProb>>,
    },
    RefusalDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
    },
    RefusalDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        refusal: String,
    },
    ReasoningDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
        obfuscation: Option<String>,
    },
    ReasoningDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        text: String,
    },
    ReasoningSummaryDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        delta: String,
        obfuscation: Option<String>,
    },
    ReasoningSummaryDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        text: String,
    },
    ReasoningSummaryPartAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        part: MessageContent,
    },
    ReasoningSummaryPartDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        part: MessageContent,
    },
    OutputTextAnnotationAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        annotation_index: i32,
        annotation: Option<Annotation>,
    },
    FunctionCallArgumentsDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        delta: String,
        obfuscation: Option<String>,
    },
    FunctionCallArgumentsDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        arguments: String,
    },
    Error {
        sequence_number: i32,
        error: ErrorPayload,
    },
    Unknown {
        event_type: String,
        sequence_number: Option<i32>,
        raw: Value,
    },
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
enum KnownStreamingEvent {
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
        item: Option<Item>,
    },
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        sequence_number: i32,
        output_index: i32,
        item: Option<Item>,
    },
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        part: MessageContent,
    },
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        part: MessageContent,
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
        part: MessageContent,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ReasoningSummaryPartDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        part: MessageContent,
    },
    #[serde(rename = "response.output_text.annotation.added")]
    OutputTextAnnotationAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        annotation_index: i32,
        annotation: Option<Annotation>,
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
}

impl Serialize for StreamingEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            StreamingEvent::Done => Err(serde::ser::Error::custom(
                "StreamingEvent::Done must be emitted as the raw `[DONE]` SSE marker, not JSON",
            )),
            StreamingEvent::Unknown { raw, .. } => raw.serialize(serializer),
            _ => match to_known_event(self) {
                Some(event) => event.serialize(serializer),
                None => Err(serde::ser::Error::custom("Unable to serialize streaming event")),
            },
        }
    }
}

impl<'de> Deserialize<'de> for StreamingEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        streaming_event_from_value(value).map_err(serde::de::Error::custom)
    }
}

impl From<KnownStreamingEvent> for StreamingEvent {
    fn from(event: KnownStreamingEvent) -> Self {
        match event {
            KnownStreamingEvent::ResponseCreated {
                sequence_number,
                response,
            } => StreamingEvent::ResponseCreated {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseQueued {
                sequence_number,
                response,
            } => StreamingEvent::ResponseQueued {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseInProgress {
                sequence_number,
                response,
            } => StreamingEvent::ResponseInProgress {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseCompleted {
                sequence_number,
                response,
            } => StreamingEvent::ResponseCompleted {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseFailed {
                sequence_number,
                response,
            } => StreamingEvent::ResponseFailed {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseIncomplete {
                sequence_number,
                response,
            } => StreamingEvent::ResponseIncomplete {
                sequence_number,
                response,
            },
            KnownStreamingEvent::OutputItemAdded {
                sequence_number,
                output_index,
                item,
            } => StreamingEvent::OutputItemAdded {
                sequence_number,
                output_index,
                item,
            },
            KnownStreamingEvent::OutputItemDone {
                sequence_number,
                output_index,
                item,
            } => StreamingEvent::OutputItemDone {
                sequence_number,
                output_index,
                item,
            },
            KnownStreamingEvent::ContentPartAdded {
                sequence_number,
                item_id,
                output_index,
                content_index,
                part,
            } => StreamingEvent::ContentPartAdded {
                sequence_number,
                item_id,
                output_index,
                content_index,
                part,
            },
            KnownStreamingEvent::ContentPartDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                part,
            } => StreamingEvent::ContentPartDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                part,
            },
            KnownStreamingEvent::OutputTextDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
                logprobs,
                obfuscation,
            } => StreamingEvent::OutputTextDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
                logprobs,
                obfuscation,
            },
            KnownStreamingEvent::OutputTextDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                text,
                logprobs,
            } => StreamingEvent::OutputTextDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                text,
                logprobs,
            },
            KnownStreamingEvent::RefusalDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
            } => StreamingEvent::RefusalDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
            },
            KnownStreamingEvent::RefusalDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                refusal,
            } => StreamingEvent::RefusalDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                refusal,
            },
            KnownStreamingEvent::ReasoningDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
                obfuscation,
            } => StreamingEvent::ReasoningDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
                obfuscation,
            },
            KnownStreamingEvent::ReasoningDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                text,
            } => StreamingEvent::ReasoningDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                text,
            },
            KnownStreamingEvent::ReasoningSummaryDelta {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                delta,
                obfuscation,
            } => StreamingEvent::ReasoningSummaryDelta {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                delta,
                obfuscation,
            },
            KnownStreamingEvent::ReasoningSummaryDone {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                text,
            } => StreamingEvent::ReasoningSummaryDone {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                text,
            },
            KnownStreamingEvent::ReasoningSummaryPartAdded {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                part,
            } => StreamingEvent::ReasoningSummaryPartAdded {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                part,
            },
            KnownStreamingEvent::ReasoningSummaryPartDone {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                part,
            } => StreamingEvent::ReasoningSummaryPartDone {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                part,
            },
            KnownStreamingEvent::OutputTextAnnotationAdded {
                sequence_number,
                item_id,
                output_index,
                content_index,
                annotation_index,
                annotation,
            } => StreamingEvent::OutputTextAnnotationAdded {
                sequence_number,
                item_id,
                output_index,
                content_index,
                annotation_index,
                annotation,
            },
            KnownStreamingEvent::FunctionCallArgumentsDelta {
                sequence_number,
                item_id,
                output_index,
                delta,
                obfuscation,
            } => StreamingEvent::FunctionCallArgumentsDelta {
                sequence_number,
                item_id,
                output_index,
                delta,
                obfuscation,
            },
            KnownStreamingEvent::FunctionCallArgumentsDone {
                sequence_number,
                item_id,
                output_index,
                arguments,
            } => StreamingEvent::FunctionCallArgumentsDone {
                sequence_number,
                item_id,
                output_index,
                arguments,
            },
            KnownStreamingEvent::Error {
                sequence_number,
                error,
            } => StreamingEvent::Error {
                sequence_number,
                error,
            },
        }
    }
}

fn to_known_event(event: &StreamingEvent) -> Option<KnownStreamingEvent> {
    match event {
        StreamingEvent::ResponseCreated {
            sequence_number,
            response,
        } => Some(KnownStreamingEvent::ResponseCreated {
            sequence_number: *sequence_number,
            response: response.clone(),
        }),
        StreamingEvent::ResponseQueued {
            sequence_number,
            response,
        } => Some(KnownStreamingEvent::ResponseQueued {
            sequence_number: *sequence_number,
            response: response.clone(),
        }),
        StreamingEvent::ResponseInProgress {
            sequence_number,
            response,
        } => Some(KnownStreamingEvent::ResponseInProgress {
            sequence_number: *sequence_number,
            response: response.clone(),
        }),
        StreamingEvent::ResponseCompleted {
            sequence_number,
            response,
        } => Some(KnownStreamingEvent::ResponseCompleted {
            sequence_number: *sequence_number,
            response: response.clone(),
        }),
        StreamingEvent::ResponseFailed {
            sequence_number,
            response,
        } => Some(KnownStreamingEvent::ResponseFailed {
            sequence_number: *sequence_number,
            response: response.clone(),
        }),
        StreamingEvent::ResponseIncomplete {
            sequence_number,
            response,
        } => Some(KnownStreamingEvent::ResponseIncomplete {
            sequence_number: *sequence_number,
            response: response.clone(),
        }),
        StreamingEvent::OutputItemAdded {
            sequence_number,
            output_index,
            item,
        } => Some(KnownStreamingEvent::OutputItemAdded {
            sequence_number: *sequence_number,
            output_index: *output_index,
            item: item.clone(),
        }),
        StreamingEvent::OutputItemDone {
            sequence_number,
            output_index,
            item,
        } => Some(KnownStreamingEvent::OutputItemDone {
            sequence_number: *sequence_number,
            output_index: *output_index,
            item: item.clone(),
        }),
        StreamingEvent::ContentPartAdded {
            sequence_number,
            item_id,
            output_index,
            content_index,
            part,
        } => Some(KnownStreamingEvent::ContentPartAdded {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            part: part.clone(),
        }),
        StreamingEvent::ContentPartDone {
            sequence_number,
            item_id,
            output_index,
            content_index,
            part,
        } => Some(KnownStreamingEvent::ContentPartDone {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            part: part.clone(),
        }),
        StreamingEvent::OutputTextDelta {
            sequence_number,
            item_id,
            output_index,
            content_index,
            delta,
            logprobs,
            obfuscation,
        } => Some(KnownStreamingEvent::OutputTextDelta {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            delta: delta.clone(),
            logprobs: logprobs.clone(),
            obfuscation: obfuscation.clone(),
        }),
        StreamingEvent::OutputTextDone {
            sequence_number,
            item_id,
            output_index,
            content_index,
            text,
            logprobs,
        } => Some(KnownStreamingEvent::OutputTextDone {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            text: text.clone(),
            logprobs: logprobs.clone(),
        }),
        StreamingEvent::RefusalDelta {
            sequence_number,
            item_id,
            output_index,
            content_index,
            delta,
        } => Some(KnownStreamingEvent::RefusalDelta {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            delta: delta.clone(),
        }),
        StreamingEvent::RefusalDone {
            sequence_number,
            item_id,
            output_index,
            content_index,
            refusal,
        } => Some(KnownStreamingEvent::RefusalDone {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            refusal: refusal.clone(),
        }),
        StreamingEvent::ReasoningDelta {
            sequence_number,
            item_id,
            output_index,
            content_index,
            delta,
            obfuscation,
        } => Some(KnownStreamingEvent::ReasoningDelta {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            delta: delta.clone(),
            obfuscation: obfuscation.clone(),
        }),
        StreamingEvent::ReasoningDone {
            sequence_number,
            item_id,
            output_index,
            content_index,
            text,
        } => Some(KnownStreamingEvent::ReasoningDone {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            text: text.clone(),
        }),
        StreamingEvent::ReasoningSummaryDelta {
            sequence_number,
            item_id,
            output_index,
            summary_index,
            delta,
            obfuscation,
        } => Some(KnownStreamingEvent::ReasoningSummaryDelta {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            summary_index: *summary_index,
            delta: delta.clone(),
            obfuscation: obfuscation.clone(),
        }),
        StreamingEvent::ReasoningSummaryDone {
            sequence_number,
            item_id,
            output_index,
            summary_index,
            text,
        } => Some(KnownStreamingEvent::ReasoningSummaryDone {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            summary_index: *summary_index,
            text: text.clone(),
        }),
        StreamingEvent::ReasoningSummaryPartAdded {
            sequence_number,
            item_id,
            output_index,
            summary_index,
            part,
        } => Some(KnownStreamingEvent::ReasoningSummaryPartAdded {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            summary_index: *summary_index,
            part: part.clone(),
        }),
        StreamingEvent::ReasoningSummaryPartDone {
            sequence_number,
            item_id,
            output_index,
            summary_index,
            part,
        } => Some(KnownStreamingEvent::ReasoningSummaryPartDone {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            summary_index: *summary_index,
            part: part.clone(),
        }),
        StreamingEvent::OutputTextAnnotationAdded {
            sequence_number,
            item_id,
            output_index,
            content_index,
            annotation_index,
            annotation,
        } => Some(KnownStreamingEvent::OutputTextAnnotationAdded {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            content_index: *content_index,
            annotation_index: *annotation_index,
            annotation: annotation.clone(),
        }),
        StreamingEvent::FunctionCallArgumentsDelta {
            sequence_number,
            item_id,
            output_index,
            delta,
            obfuscation,
        } => Some(KnownStreamingEvent::FunctionCallArgumentsDelta {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            delta: delta.clone(),
            obfuscation: obfuscation.clone(),
        }),
        StreamingEvent::FunctionCallArgumentsDone {
            sequence_number,
            item_id,
            output_index,
            arguments,
        } => Some(KnownStreamingEvent::FunctionCallArgumentsDone {
            sequence_number: *sequence_number,
            item_id: item_id.clone(),
            output_index: *output_index,
            arguments: arguments.clone(),
        }),
        StreamingEvent::Error {
            sequence_number,
            error,
        } => Some(KnownStreamingEvent::Error {
            sequence_number: *sequence_number,
            error: error.clone(),
        }),
        StreamingEvent::Unknown { .. } | StreamingEvent::Done => None,
    }
}

fn streaming_event_from_value(value: Value) -> Result<StreamingEvent, String> {
    let event_type = value
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| "Streaming event is missing a string `type` field".to_string())?
        .to_string();

    match event_type.as_str() {
        "response.created"
        | "response.queued"
        | "response.in_progress"
        | "response.completed"
        | "response.failed"
        | "response.incomplete"
        | "response.output_item.added"
        | "response.output_item.done"
        | "response.content_part.added"
        | "response.content_part.done"
        | "response.output_text.delta"
        | "response.output_text.done"
        | "response.refusal.delta"
        | "response.refusal.done"
        | "response.reasoning.delta"
        | "response.reasoning.done"
        | "response.reasoning_summary_text.delta"
        | "response.reasoning_summary_text.done"
        | "response.reasoning_summary_part.added"
        | "response.reasoning_summary_part.done"
        | "response.output_text.annotation.added"
        | "response.function_call_arguments.delta"
        | "response.function_call_arguments.done"
        | "error" => {
            let known: KnownStreamingEvent =
                serde_json::from_value(value).map_err(|e| e.to_string())?;
            Ok(known.into())
        }
        _ => Ok(StreamingEvent::Unknown {
            sequence_number: value
                .get("sequence_number")
                .and_then(Value::as_i64)
                .and_then(|n| i32::try_from(n).ok()),
            event_type,
            raw: value,
        }),
    }
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
