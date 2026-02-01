//! # Open Responses
//!
//! A Rust client library for the Open Responses API specification.
//!
//! Open Responses is an open-source specification for building multi-provider,
//! interoperable LLM interfaces based on the OpenAI Responses API.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use open_responses::{Client, CreateResponseBody, Input, Item};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("your-api-key");
//!     
//!     let request = CreateResponseBody {
//!         model: Some("gpt-4o".to_string()),
//!         input: Some(Input::Items(vec![
//!             Item::user_message("Hello, how are you?")
//!         ])),
//!         ..Default::default()
//!     };
//!     
//!     let response = client.create_response(request).await?;
//!     println!("Response: {:?}", response);
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod streaming;
pub mod types;

pub use client::{Client, ClientError};
pub use streaming::{StreamingClient, StreamingError};
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::user_message("Hello");
        assert!(matches!(item, Item::Message { .. }));
    }
}
