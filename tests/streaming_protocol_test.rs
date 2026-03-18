use futures::StreamExt;
use mockito::Server;
use openresponses_rust::{
    CreateResponseBody, RawSseEvent, StreamingClient, StreamingError, StreamingEvent,
};
use serde_json::json;

#[test]
fn test_nullable_streaming_payloads_deserialize() {
    let output_item_added: StreamingEvent = serde_json::from_value(json!({
        "type": "response.output_item.added",
        "sequence_number": 1,
        "output_index": 0,
        "item": null
    }))
    .unwrap();

    assert!(matches!(
        output_item_added,
        StreamingEvent::OutputItemAdded { item: None, .. }
    ));

    let annotation_added: StreamingEvent = serde_json::from_value(json!({
        "type": "response.output_text.annotation.added",
        "sequence_number": 2,
        "item_id": "msg_123",
        "output_index": 0,
        "content_index": 0,
        "annotation_index": 0,
        "annotation": null
    }))
    .unwrap();

    assert!(matches!(
        annotation_added,
        StreamingEvent::OutputTextAnnotationAdded { annotation: None, .. }
    ));
}

#[tokio::test]
async fn test_unknown_streaming_event_is_preserved() {
    let mut server = Server::new_async().await;
    let body = concat!(
        "event: acme:trace_event\n",
        "data: {\"type\":\"acme:trace_event\",\"sequence_number\":1,\"phase\":\"tool_resolution\"}\n\n",
        "data: [DONE]\n\n"
    );

    let _mock = server
        .mock("POST", "/v1/responses")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        .with_body(body)
        .create_async()
        .await;

    let client = StreamingClient::with_base_url("test-key", server.url());
    let mut stream = client
        .stream_response(CreateResponseBody::default())
        .await
        .unwrap();

    let first = stream.next().await.unwrap().unwrap();
    assert!(matches!(
        first,
        StreamingEvent::Unknown { ref event_type, .. } if event_type == "acme:trace_event"
    ));

    let done = stream.next().await.unwrap().unwrap();
    assert!(matches!(done, StreamingEvent::Done));
}

#[tokio::test]
async fn test_stream_event_name_mismatch_is_rejected() {
    let mut server = Server::new_async().await;
    let body = concat!(
        "event: response.output_text.delta\n",
        "data: {\"type\":\"acme:trace_event\",\"sequence_number\":1}\n\n"
    );

    let _mock = server
        .mock("POST", "/v1/responses")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        .with_body(body)
        .create_async()
        .await;

    let client = StreamingClient::with_base_url("test-key", server.url());
    let mut stream = client
        .stream_response(CreateResponseBody::default())
        .await
        .unwrap();

    let error = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(
        error,
        StreamingError::EventTypeMismatch { ref sse_event, ref body_type }
            if sse_event == "response.output_text.delta" && body_type == "acme:trace_event"
    ));
}

#[tokio::test]
async fn test_stream_protocol_violation_is_rejected() {
    let mut server = Server::new_async().await;
    let body = concat!(
        "event: response.output_text.delta\n",
        "data: {\"type\":\"response.output_text.delta\",\"sequence_number\":1,\"item_id\":\"msg_123\",\"output_index\":0,\"content_index\":0,\"delta\":\"Hello\"}\n\n"
    );

    let _mock = server
        .mock("POST", "/v1/responses")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        .with_body(body)
        .create_async()
        .await;

    let client = StreamingClient::with_base_url("test-key", server.url());
    let mut stream = client
        .stream_response(CreateResponseBody::default())
        .await
        .unwrap();

    let error = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(
        error,
        StreamingError::LifecycleError { ref message }
            if message.contains("response.output_item.added")
                || message.contains("response.content_part.added")
    ));
}

#[tokio::test]
async fn test_output_item_done_requires_closed_content_parts() {
    let mut server = Server::new_async().await;
    let body = concat!(
        "event: response.output_item.added\n",
        "data: {\"type\":\"response.output_item.added\",\"sequence_number\":1,\"output_index\":0,\"item\":{\"type\":\"message\",\"id\":\"msg_123\",\"status\":\"in_progress\",\"role\":\"assistant\",\"content\":[{\"type\":\"output_text\",\"text\":\"\",\"annotations\":[]}]}}\n\n",
        "event: response.content_part.added\n",
        "data: {\"type\":\"response.content_part.added\",\"sequence_number\":2,\"item_id\":\"msg_123\",\"output_index\":0,\"content_index\":0,\"part\":{\"type\":\"output_text\",\"text\":\"\",\"annotations\":[]}}\n\n",
        "event: response.output_item.done\n",
        "data: {\"type\":\"response.output_item.done\",\"sequence_number\":3,\"output_index\":0,\"item\":{\"type\":\"message\",\"id\":\"msg_123\",\"status\":\"completed\",\"role\":\"assistant\",\"content\":[{\"type\":\"output_text\",\"text\":\"\",\"annotations\":[]}]}}\n\n"
    );

    let _mock = server
        .mock("POST", "/v1/responses")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        .with_body(body)
        .create_async()
        .await;

    let client = StreamingClient::with_base_url("test-key", server.url());
    let mut stream = client
        .stream_response(CreateResponseBody::default())
        .await
        .unwrap();

    assert!(stream.next().await.unwrap().is_ok());
    assert!(stream.next().await.unwrap().is_ok());

    let error = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(
        error,
        StreamingError::LifecycleError { ref message }
            if message.contains("before all content parts were closed")
    ));
}

#[tokio::test]
async fn test_content_part_done_is_terminal() {
    let mut server = Server::new_async().await;
    let body = concat!(
        "event: response.output_item.added\n",
        "data: {\"type\":\"response.output_item.added\",\"sequence_number\":1,\"output_index\":0,\"item\":{\"type\":\"message\",\"id\":\"msg_123\",\"status\":\"in_progress\",\"role\":\"assistant\",\"content\":[{\"type\":\"output_text\",\"text\":\"\",\"annotations\":[]}]}}\n\n",
        "event: response.content_part.added\n",
        "data: {\"type\":\"response.content_part.added\",\"sequence_number\":2,\"item_id\":\"msg_123\",\"output_index\":0,\"content_index\":0,\"part\":{\"type\":\"output_text\",\"text\":\"\",\"annotations\":[]}}\n\n",
        "event: response.content_part.done\n",
        "data: {\"type\":\"response.content_part.done\",\"sequence_number\":3,\"item_id\":\"msg_123\",\"output_index\":0,\"content_index\":0,\"part\":{\"type\":\"output_text\",\"text\":\"\",\"annotations\":[]}}\n\n",
        "event: response.output_text.delta\n",
        "data: {\"type\":\"response.output_text.delta\",\"sequence_number\":4,\"item_id\":\"msg_123\",\"output_index\":0,\"content_index\":0,\"delta\":\"Hello\"}\n\n"
    );

    let _mock = server
        .mock("POST", "/v1/responses")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        .with_body(body)
        .create_async()
        .await;

    let client = StreamingClient::with_base_url("test-key", server.url());
    let mut stream = client
        .stream_response(CreateResponseBody::default())
        .await
        .unwrap();

    assert!(stream.next().await.unwrap().is_ok());
    assert!(stream.next().await.unwrap().is_ok());
    assert!(stream.next().await.unwrap().is_ok());

    let error = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(
        error,
        StreamingError::LifecycleError { ref message }
            if message.contains("after `response.content_part.done`")
    ));
}

#[test]
fn test_done_marker_cannot_be_json_serialized() {
    let error = serde_json::to_string(&StreamingEvent::Done).unwrap_err();
    assert!(error.to_string().contains("raw `[DONE]`"));
}

#[tokio::test]
async fn test_raw_sse_stream_preserves_event_name() {
    let mut server = Server::new_async().await;
    let body = concat!(
        "event: acme:trace_event\n",
        "data: {\"type\":\"acme:trace_event\",\"sequence_number\":1}\n\n"
    );

    let _mock = server
        .mock("POST", "/v1/responses")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        .with_body(body)
        .create_async()
        .await;

    let client = StreamingClient::with_base_url("test-key", server.url());
    let mut stream = client
        .stream_response_lines(CreateResponseBody::default())
        .await
        .unwrap();

    let event = stream.next().await.unwrap().unwrap();
    assert_eq!(
        event,
        RawSseEvent {
            event: Some("acme:trace_event".to_string()),
            data: "{\"type\":\"acme:trace_event\",\"sequence_number\":1}".to_string(),
        }
    );
}
