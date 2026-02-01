use open_responses::types::*;

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
    let text = InputContent::text("Hello world");
    assert!(matches!(text, InputContent::Text { text } if text == "Hello world"));

    let image = InputContent::image_url("https://example.com/image.png");
    assert!(
        matches!(image, InputContent::Image { image_url: Some(url), .. } if url == "https://example.com/image.png")
    );

    let file_url = InputContent::file_url("https://example.com/doc.pdf");
    assert!(
        matches!(file_url, InputContent::File { file_url: Some(url), .. } if url == "https://example.com/doc.pdf")
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
        status: FunctionCallStatus::Completed,
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
