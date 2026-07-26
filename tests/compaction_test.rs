use mockito::Server;
use openresponses_rust::{
    Client, CompactResponseBody, Input, Item, MessagePhase,
};

#[tokio::test]
async fn test_compact_response_api() {
    let mut server = Server::new_async().await;

    let mock_response = serde_json::json!({
        "id": "resp_compact_001",
        "object": "response.compaction",
        "created_at": 1764967971,
        "output": [
            {
                "id": "msg_000",
                "type": "message",
                "status": "completed",
                "role": "user",
                "content": [
                    {
                        "type": "input_text",
                        "text": "Initial prompt"
                    }
                ]
            },
            {
                "id": "cmp_001",
                "type": "compaction",
                "encrypted_content": "gAAAAABpM0Yj-test"
            }
        ],
        "usage": {
            "input_tokens": 100,
            "input_tokens_details": {
                "cached_tokens": 0
            },
            "output_tokens": 50,
            "output_tokens_details": {
                "reasoning_tokens": 10
            },
            "total_tokens": 150
        }
    });

    let mock = server
        .mock("POST", "/v1/responses/compact")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let client = Client::with_base_url("test-api-key", server.url());

    let request = CompactResponseBody {
        model: "gpt-5".to_string(),
        input: Some(Input::Items(vec![
            Item::user_message("Initial prompt"),
            Item::assistant_message_with_phase("Drafting answer", MessagePhase::Commentary),
        ])),
        ..Default::default()
    };

    let result = client.compact_response(request).await.unwrap();

    assert_eq!(result.id, "resp_compact_001");
    assert_eq!(result.object, "response.compaction");
    assert_eq!(result.output.len(), 2);

    if let Item::Compaction { encrypted_content, .. } = &result.output[1] {
        assert_eq!(encrypted_content, "gAAAAABpM0Yj-test");
    } else {
        panic!("Expected Item::Compaction");
    }

    mock.assert_async().await;
}
