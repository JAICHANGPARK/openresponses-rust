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
