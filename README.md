# Open Responses

[![Crates.io](https://img.shields.io/crates/v/openresponses-rust)](https://crates.io/crates/openresponses-rust)
[![Documentation](https://docs.rs/openresponses-rust/badge.svg)](https://docs.rs/openresponses-rust)
[![License](https://img.shields.io/crates/l/openresponses-rust)](LICENSE)

A Rust client library for the Open Responses API specification.

Open Responses is an open-source specification for building multi-provider, interoperable LLM interfaces based on the OpenAI Responses API. This library provides a unified experience for calling language models, streaming results, and composing agentic workflows across different providers.

## Key Features

- **Schema Compliance**: Strictly follows the latest OpenAI Responses API spec.
- **Auto URL Normalization**: Just provide the base domain; we'll handle the `/v1/responses` path for you.
- **MCP Tool Support**: Compatible with Model Context Protocol (MCP) tools (e.g., in LM Studio).
- **Stateful & Stateless**: Support for both standard chat and stateful follow-ups using `previous_response_id`.
- **Rich Streaming**: Comprehensive SSE event handling for real-time applications.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openresponses-rust = "0.2.0"
tokio = { version = "1", features = ["full"] }
```

## How to Configure Your API

The library provides a flexible `ClientBuilder` to suit your development or production environment.

### 1. Direct Input (Simple & Quick)
Best for local testing or when using local LLM servers like **LM Studio**. You can pass string literals directly.

```rust
use openresponses_rust::Client;

let client = Client::builder("any-key")
    .base_url("http://localhost:1234") // No need to add /v1/responses
    .build();
```

### 2. Environment Variables (Recommended for Production)
Best for keeping secrets and configurations out of your source code.

```rust
use openresponses_rust::Client;
use std::env;

let api_key = env::var("API_KEY").expect("API_KEY is required");
let api_url = env::var("API_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());

let client = Client::builder(api_key)
    .base_url(api_url)
    .build();
```

---

## Quick Start

### Basic Chat

```rust
use openresponses_rust::{Client, CreateResponseBody, Input, Item};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key");
    
    let request = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Items(vec![
            Item::user_message("Hello, how are you?")
        ])),
        ..Default::default()
    };
    
    let response = client.create_response(request).await?;
    
    for item in response.output {
        if let Item::Message { content, .. } = item {
            println!("Assistant: {:?}", content);
        }
    }
    
    Ok(())
}
```

### Streaming Responses

```rust
use openresponses_rust::{StreamingClient, StreamingEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = StreamingClient::new("your-api-key");
    let mut stream = client.stream_response(request).await?;

    while let Some(event) = stream.next().await {
        match event {
            Ok(StreamingEvent::OutputTextDelta { delta, .. }) => {
                print!("{}", delta);
            }
            Ok(StreamingEvent::Done) => break,
            _ => {}
        }
    }
    Ok(())
}
```

### Stateful Follow-up

Continue a conversation by referencing a previous response ID (if supported by your provider).

```rust
let request = CreateResponseBody {
    model: Some("gpt-4o".to_string()),
    input: Some(Input::Single("What was my last question?".to_string())),
    previous_response_id: Some("resp_123...".to_string()),
    ..Default::default()
};
```

## Examples

Check the `examples/` directory for ready-to-run code:

- `direct_input.rs`: Simplified connection to local servers.
- `env_config.rs`: Using environment variables.
- `stateful_follow_up.rs`: Chaining conversations.
- `streaming.rs`: SSE streaming implementation.
- `function_calling.rs`: Using tools and functions.

Run any example:
```bash
cargo run --example direct_input
```

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.