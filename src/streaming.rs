use eventsource_stream::Eventsource;
use futures::{Stream, StreamExt};
use reqwest::{Client as ReqwestClient, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue}};
use thiserror::Error;

use crate::types::{CreateResponseBody, StreamingEvent};

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

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

#[derive(Clone)]
pub struct StreamingClient {
    inner: ReqwestClient,
    base_url: String,
    api_key: String,
}

impl StreamingClient {
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
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("text/event-stream"),
        );
        
        let inner = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");
        
        Self { inner, base_url, api_key }
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
        assert_eq!(client.base_url, DEFAULT_BASE_URL);
    }
    
    #[test]
    fn test_streaming_client_with_base_url() {
        let client = StreamingClient::with_base_url("test-key", "https://custom.api.com");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, "https://custom.api.com");
    }
}
