use reqwest::{Client as ReqwestClient, header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue}};
use serde_json;
use thiserror::Error;

use crate::types::{CreateResponseBody, ResponseResource};

const DEFAULT_BASE_URL: &str = "https://api.openai.com";

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

pub struct ClientBuilder {
    api_key: String,
    base_url: Option<String>,
}

impl ClientBuilder {
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

    pub fn build(self) -> Client {
        let mut base_url = self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        
        // Remove trailing slash if present
        if base_url.ends_with('/') {
            base_url.pop();
        }
        
        // Automatically append /v1 if it's not present in the path
        if !base_url.ends_with("/v1") {
            base_url.push_str("/v1");
        }
        
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        
        let inner = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");
        
        Client {
            inner,
            base_url,
            api_key: self.api_key,
        }
    }
}

#[derive(Clone)]
pub struct Client {
    inner: ReqwestClient,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        ClientBuilder::new(api_key).build()
    }
    
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        ClientBuilder::new(api_key).base_url(base_url).build()
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
        assert_eq!(client.base_url, "https://api.openai.com/v1");
    }
    
    #[test]
    fn test_client_with_base_url_normalization() {
        // Domain only
        let client = Client::with_base_url("test-key", "https://openrouter.ai/api");
        assert_eq!(client.base_url, "https://openrouter.ai/api/v1");
        
        // Already includes v1
        let client = Client::with_base_url("test-key", "https://openrouter.ai/api/v1");
        assert_eq!(client.base_url, "https://openrouter.ai/api/v1");

        // Localhost
        let client = Client::with_base_url("test-key", "http://localhost:1234");
        assert_eq!(client.base_url, "http://localhost:1234/v1");
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
