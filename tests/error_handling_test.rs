use mockito::Server;
use openresponses_rust::{
    ApiErrorResponse, Client, ClientError, CreateResponseBody, ResponseStatus, StreamingClient,
    StreamingError,
};

#[test]
fn test_api_error_response_parse() {
    let body = r#"{
        "error": {
            "message": "The requested model does not exist.",
            "type": "invalid_request_error",
            "param": "model",
            "code": "model_not_found"
        }
    }"#;

    let error = ApiErrorResponse::parse(body).unwrap();
    assert_eq!(error.message, "The requested model does not exist.");
    assert_eq!(error.error_type, "invalid_request_error");
    assert_eq!(error.param.as_deref(), Some("model"));
    assert_eq!(error.code.as_deref(), Some("model_not_found"));
}

#[test]
fn test_response_status_preserves_unknown_values() {
    let completed: ResponseStatus = serde_json::from_str("\"completed\"").unwrap();
    assert_eq!(completed, ResponseStatus::Completed);

    let custom: ResponseStatus = serde_json::from_str("\"provider_custom\"").unwrap();
    assert_eq!(custom, ResponseStatus::Other("provider_custom".to_string()));
    assert_eq!(serde_json::to_string(&custom).unwrap(), "\"provider_custom\"");
}

#[tokio::test]
async fn test_client_returns_structured_api_error() {
    let mut server = Server::new_async().await;
    let body = r#"{
        "error": {
            "message": "The requested model does not exist.",
            "type": "invalid_request_error",
            "param": "model",
            "code": "model_not_found"
        }
    }"#;

    let _mock = server
        .mock("POST", "/v1/responses")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create_async()
        .await;

    let client = Client::with_base_url("test-key", server.url());
    let error = client
        .create_response(CreateResponseBody::default())
        .await
        .unwrap_err();

    match error {
        ClientError::ApiError {
            status_code,
            error,
            raw_body,
        } => {
            assert_eq!(status_code, 400);
            let error = error.expect("expected parsed error body");
            assert_eq!(error.message, "The requested model does not exist.");
            assert_eq!(error.error_type, "invalid_request_error");
            assert_eq!(error.param.as_deref(), Some("model"));
            assert_eq!(error.code.as_deref(), Some("model_not_found"));
            assert!(raw_body.contains("model_not_found"));
        }
        other => panic!("expected API error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_streaming_client_returns_structured_api_error() {
    let mut server = Server::new_async().await;
    let body = r#"{
        "error": {
            "message": "The requested model does not exist.",
            "type": "invalid_request_error",
            "param": "model",
            "code": "model_not_found"
        }
    }"#;

    let _mock = server
        .mock("POST", "/v1/responses")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create_async()
        .await;

    let client = StreamingClient::with_base_url("test-key", server.url());
    let error = match client.stream_response(CreateResponseBody::default()).await {
        Ok(_) => panic!("expected API error"),
        Err(error) => error,
    };

    match error {
        StreamingError::ApiError {
            status_code,
            error,
            raw_body,
        } => {
            assert_eq!(status_code, 400);
            let error = error.expect("expected parsed error body");
            assert_eq!(error.message, "The requested model does not exist.");
            assert_eq!(error.error_type, "invalid_request_error");
            assert_eq!(error.param.as_deref(), Some("model"));
            assert_eq!(error.code.as_deref(), Some("model_not_found"));
            assert!(raw_body.contains("model_not_found"));
        }
        other => panic!("expected API error, got {other:?}"),
    }
}
