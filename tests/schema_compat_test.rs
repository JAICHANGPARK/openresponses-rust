use openresponses_rust::types::*;
use serde_json::{json, Value};

#[test]
fn test_extension_item_roundtrip() {
    let raw = json!({
        "type": "acme:search_result",
        "id": "sr_123",
        "status": "completed",
        "query": "climate change",
        "score": 0.98
    });

    let item: Item = serde_json::from_value(raw.clone()).unwrap();

    match &item {
        Item::Extension {
            id,
            item_type,
            status,
            extra,
        } => {
            assert_eq!(id.as_deref(), Some("sr_123"));
            assert_eq!(item_type, "acme:search_result");
            assert_eq!(status.as_deref(), Some("completed"));
            assert_eq!(extra.get("query"), Some(&Value::String("climate change".to_string())));
        }
        _ => panic!("expected extension item"),
    }

    let serialized = serde_json::to_value(item).unwrap();
    assert_eq!(serialized, raw);
}

#[test]
fn test_extension_tool_roundtrip() {
    let raw = json!({
        "type": "implementor_slug:custom_document_search",
        "documents": [{"type": "external_file", "url": "https://example.com/doc.pdf"}]
    });

    let tool: Tool = serde_json::from_value(raw.clone()).unwrap();

    match &tool {
        Tool::Extension { tool_type, extra } => {
            assert_eq!(tool_type, "implementor_slug:custom_document_search");
            assert!(extra.contains_key("documents"));
        }
        _ => panic!("expected extension tool"),
    }

    assert_eq!(serde_json::to_value(tool).unwrap(), raw);
}

#[test]
fn test_message_content_string_deserializes_for_request_roles() {
    let user_message: Item = serde_json::from_value(json!({
        "type": "message",
        "role": "user",
        "content": "hello"
    }))
    .unwrap();

    assert!(matches!(
        user_message,
        Item::Message { role: MessageRole::User, content, .. }
            if matches!(content.as_slice(), [MessageContent::InputText { text }] if text == "hello")
    ));

    let assistant_message: Item = serde_json::from_value(json!({
        "type": "message",
        "role": "assistant",
        "content": "done"
    }))
    .unwrap();

    assert!(matches!(
        assistant_message,
        Item::Message { role: MessageRole::Assistant, content, .. }
            if matches!(content.as_slice(), [MessageContent::OutputText { text, .. }] if text == "done")
    ));
}

#[test]
fn test_invalid_role_content_is_rejected_on_serialize() {
    let invalid = Item::Message {
        id: None,
        status: None,
        role: MessageRole::Assistant,
        content: vec![MessageContent::input_text("not allowed")],
    };

    let error = serde_json::to_string(&invalid).unwrap_err();
    assert!(error.to_string().contains("not valid"));
}

#[test]
fn test_function_call_status_is_optional() {
    let item: Item = serde_json::from_value(json!({
        "type": "function_call",
        "call_id": "call_123",
        "name": "lookup_weather",
        "arguments": "{\"location\":\"Seoul\"}"
    }))
    .unwrap();

    assert!(matches!(
        item,
        Item::FunctionCall { status: None, .. }
    ));

    let output_item: Item = serde_json::from_value(json!({
        "type": "function_call_output",
        "call_id": "call_123",
        "output": "{\"temperature\":20}"
    }))
    .unwrap();

    assert!(matches!(
        output_item,
        Item::FunctionCallOutput { status: None, .. }
    ));
}

#[test]
fn test_allowed_tools_mode_is_optional_for_requests() {
    let tool_choice: ToolChoiceParam = serde_json::from_value(json!({
        "type": "allowed_tools",
        "tools": [
            {"type": "function", "name": "lookup_weather"}
        ]
    }))
    .unwrap();

    assert!(matches!(
        tool_choice,
        ToolChoiceParam::Allowed { mode: None, .. }
    ));
}

#[test]
fn test_invalid_function_call_output_content_is_rejected() {
    let invalid = Item::FunctionCallOutput {
        id: None,
        call_id: "call_123".to_string(),
        output: FunctionOutput::Content(vec![MessageContent::output_text("not allowed")]),
        status: None,
    };

    let error = serde_json::to_string(&invalid).unwrap_err();
    assert!(error.to_string().contains("function_call_output"));
}
