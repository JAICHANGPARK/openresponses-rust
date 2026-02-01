use reqwest::{Client as ReqwestClient, header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue}};
use serde_json;
use thiserror::Error;

use crate::types::{CreateResponseBody, ResponseResource};

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("API error: {code} - {message}")]
    ApiError { code: String, message: String },
    
    #[error("Invalid header value: {0}")]
    InvalidHeader(String),
}

#[derive(Clone)]
pub struct Client {
    inner: ReqwestClient,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, DEFAULT_BASE_URL)
    }
    
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        let api_key = api_key.into();
        let base_url = base_url.into();
        
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        
        let inner = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");
        
        Self { inner, base_url, api_key }
    }
    
    pub async fn create_response(&self, request: CreateResponseBody) -> Result<ResponseResource, ClientError> {
        let url = format!("{}/responses", self.base_url);
        
        let response = self.inner
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        
        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(ClientError::ApiError {
                code: status.to_string(),
                message: error_text,
            });
        }
        
        let response_body = response.json::<ResponseResource>().await?;
        Ok(response_body)
    }
    
    pub async fn create_response_raw(&self, request: CreateResponseBody) -> Result<String, ClientError> {
        let url = format!("{}/responses", self.base_url);
        
        let response = self.inner
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        if !status.is_success() {
            return Err(ClientError::ApiError {
                code: status.to_string(),
                message: body,
            });
        }
        
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Input, Item};
    
    #[test]
    fn test_client_creation() {
        let client = Client::new("test-api-key");
        assert_eq!(client.api_key, "test-api-key");
        assert_eq!(client.base_url, DEFAULT_BASE_URL);
    }
    
    #[test]
    fn test_client_with_base_url() {
        let client = Client::with_base_url("test-key", "https://custom.api.com");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, "https://custom.api.com");
    }
    
    #[tokio::test]
    async fn test_request_serialization() {
        let request = CreateResponseBody {
            model: Some("gpt-4o".to_string()),
            input: Some(Input::Items(vec![
                Item::user_message("Hello, world!")
            ])),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("gpt-4o"));
        assert!(json.contains("Hello, world!"));
    }
}
