use openresponses_rust::types::*;

#[test]
fn test_item_creation() {
    let user_msg = Item::user_message("Hello, how are you?");
    assert!(matches!(
        user_msg,
        Item::Message {
            role: MessageRole::User,
            ..
        }
    ));

    let assistant_msg = Item::assistant_message("I'm doing well!");
    assert!(matches!(
        assistant_msg,
        Item::Message {
            role: MessageRole::Assistant,
            ..
        }
    ));

    let system_msg = Item::system_message("You are a helpful assistant.");
    assert!(matches!(
        system_msg,
        Item::Message {
            role: MessageRole::System,
            ..
        }
    ));

    let dev_msg = Item::developer_message("Follow these instructions.");
    assert!(matches!(
        dev_msg,
        Item::Message {
            role: MessageRole::Developer,
            ..
        }
    ));

    let reference = Item::reference("msg_123");
    assert!(matches!(reference, Item::ItemReference { id } if id == "msg_123"));
}

#[test]
fn test_content_creation() {
    let text = MessageContent::input_text("Hello world");
    assert!(matches!(text, MessageContent::InputText { text } if text == "Hello world"));

    let image = MessageContent::image_url("https://example.com/image.png");
    assert!(
        matches!(image, MessageContent::InputImage { image_url: Some(url), .. } if url == "https://example.com/image.png")
    );

    let file_url = MessageContent::file_url("https://example.com/doc.pdf");
    assert!(
        matches!(file_url, MessageContent::InputFile { file_url: Some(url), .. } if url == "https://example.com/doc.pdf")
    );
}

#[test]
fn test_tool_creation() {
    let tool = Tool::function("get_weather")
        .with_description("Get the current weather")
        .with_parameters(serde_json::json!({
            "type": "object",
            "properties": {
                "location": { "type": "string" }
            }
        }))
        .strict(true);

    assert!(matches!(tool, Tool::Function { name, .. } if name == "get_weather"));
}

#[test]
fn test_request_body_creation() {
    let request = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Items(vec![
            Item::system_message("You are a helpful assistant."),
            Item::user_message("What is the weather?"),
        ])),
        temperature: Some(0.7),
        max_output_tokens: Some(150),
        ..Default::default()
    };

    assert_eq!(request.model, Some("gpt-4o".to_string()));
    assert!(matches!(request.input, Some(Input::Items(items)) if items.len() == 2));
}

#[test]
fn test_serialization_roundtrip() {
    let request = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Items(vec![Item::user_message("Test message")])),
        ..Default::default()
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: CreateResponseBody = serde_json::from_str(&json).unwrap();

    assert_eq!(request.model, deserialized.model);
}

#[test]
fn test_function_call_item() {
    let func_call = Item::FunctionCall {
        id: Some("fc_123".to_string()),
        call_id: "call_456".to_string(),
        name: "get_weather".to_string(),
        arguments: r#"{"location": "San Francisco"}"#.to_string(),
        status: Some(FunctionCallStatus::Completed),
    };

    assert!(matches!(func_call, Item::FunctionCall { name, .. } if name == "get_weather"));
}

#[test]
fn test_streaming_event_serialization() {
    let event = StreamingEvent::OutputTextDelta {
        sequence_number: 1,
        item_id: "msg_123".to_string(),
        output_index: 0,
        content_index: 0,
        delta: "Hello".to_string(),
        logprobs: None,
        obfuscation: None,
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("response.output_text.delta"));
    assert!(json.contains("Hello"));
}

#[test]
fn test_compaction_item_roundtrip() {
    let compaction = Item::compaction("encrypted_payload_123");
    let json = serde_json::to_string(&compaction).unwrap();
    assert!(json.contains("compaction"));
    assert!(json.contains("encrypted_payload_123"));

    let deserialized: Item = serde_json::from_str(&json).unwrap();
    assert_eq!(compaction, deserialized);
}

#[test]
fn test_message_phase_roundtrip() {
    let msg = Item::assistant_message_with_phase("Draft step", MessagePhase::Commentary);
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("commentary"));
    assert!(json.contains("phase"));

    let deserialized: Item = serde_json::from_str(&json).unwrap();
    assert_eq!(msg, deserialized);
}

#[test]
fn test_input_video_content() {
    let video = MessageContent::video_url("https://example.com/video.mp4");
    let json = serde_json::to_string(&video).unwrap();
    assert!(json.contains("input_video"));
    assert!(json.contains("https://example.com/video.mp4"));

    let deserialized: MessageContent = serde_json::from_str(&json).unwrap();
    assert_eq!(video, deserialized);
}

#[test]
fn test_compact_response_body_serialization() {
    let compact_req = CompactResponseBody {
        model: "gpt-5".to_string(),
        input: Some(Input::Items(vec![Item::user_message("Long conversation")])),
        previous_response_id: Some("resp_123".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&compact_req).unwrap();
    assert!(json.contains("gpt-5"));
    assert!(json.contains("resp_123"));
}

