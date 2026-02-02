use eventsource_stream::Eventsource;
use futures::{Stream, StreamExt};
use reqwest::{Client as ReqwestClient, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue}};
use thiserror::Error;

use crate::types::{CreateResponseBody, StreamingEvent};

const DEFAULT_BASE_URL: &str = "https://api.openai.com";

#[derive(Error, Debug)]
pub enum StreamingError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Event stream error: {0}")]
    StreamError(String),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {message}")]
    ApiError { message: String },
}

pub struct StreamingClientBuilder {
    api_key: String,
    base_url: Option<String>,
}

impl StreamingClientBuilder {
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

    pub fn build(self) -> StreamingClient {
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
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("text/event-stream"),
        );
        
        let inner = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");
        
        StreamingClient {
            inner,
            base_url,
            api_key: self.api_key,
        }
    }
}

#[derive(Clone)]
pub struct StreamingClient {
    inner: ReqwestClient,
    base_url: String,
    api_key: String,
}

impl StreamingClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        StreamingClientBuilder::new(api_key).build()
    }

    pub fn builder(api_key: impl Into<String>) -> StreamingClientBuilder {
        StreamingClientBuilder::new(api_key)
    }
    
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        StreamingClientBuilder::new(api_key).base_url(base_url).build()
    }
    
    pub async fn stream_response(
        &self,
        mut request: CreateResponseBody,
    ) -> Result<impl Stream<Item = Result<StreamingEvent, StreamingError>>, StreamingError> {
        request.stream = Some(true);
        
        let url = format!("{}/responses", self.base_url);
        
        let response = self.inner
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(StreamingError::ApiError { message: error_text });
        }
        
        let stream = response.bytes_stream();
        let eventsource = stream.eventsource();
        
        let event_stream = eventsource.map(|event| {
            match event {
                Ok(event) => {
                    if event.data == "[DONE]" {
                        Ok(StreamingEvent::Done)
                    } else {
                        serde_json::from_str::<StreamingEvent>(&event.data)
                            .map_err(StreamingError::JsonError)
                    }
                }
                Err(e) => Err(StreamingError::StreamError(e.to_string())),
            }
        });
        
        Ok(event_stream)
    }
    
    pub async fn stream_response_lines(
        &self,
        mut request: CreateResponseBody,
    ) -> Result<impl Stream<Item = Result<String, StreamingError>>, StreamingError> {
        request.stream = Some(true);
        
        let url = format!("{}/responses", self.base_url);
        
        let response = self.inner
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(StreamingError::ApiError { message: error_text });
        }
        
        let stream = response.bytes_stream();
        let eventsource = stream.eventsource();
        
        let line_stream = eventsource.map(|event| {
            match event {
                Ok(event) => Ok(event.data),
                Err(e) => Err(StreamingError::StreamError(e.to_string())),
            }
        });
        
        Ok(line_stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_streaming_client_creation() {
        let client = StreamingClient::new("test-api-key");
        assert_eq!(client.api_key, "test-api-key");
        assert_eq!(client.base_url, "https://api.openai.com/v1");
    }
    
    #[test]
    fn test_streaming_client_with_base_url_normalization() {
        let client = StreamingClient::with_base_url("test-key", "https://openrouter.ai/api");
        assert_eq!(client.base_url, "https://openrouter.ai/api/v1");
    }
}
