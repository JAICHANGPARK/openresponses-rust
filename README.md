# Open Responses

[![Crates.io](https://img.shields.io/crates/v/openresponses-rust)](https://crates.io/crates/openresponses-rust)
[![Documentation](https://docs.rs/openresponses-rust/badge.svg)](https://docs.rs/openresponses-rust)
[![License](https://img.shields.io/crates/l/openresponses-rust)](LICENSE)

A Rust client library for the Open Responses API specification.

Open Responses is an open-source specification for building multi-provider, interoperable LLM interfaces based on the OpenAI Responses API. It defines a shared schema and tooling layer that enable a unified experience for calling language models, streaming results, and composing agentic workflows—independent of provider.

## Features

- **Complete Type Coverage**: All request/response types from the Open Responses specification
- **Streaming Support**: Built-in SSE (Server-Sent Events) streaming for real-time responses
- **Type-Safe**: Strongly typed API with comprehensive error handling
- **Async/Await**: Built on Tokio for high-performance async operations
- **Easy to Use**: Simple, intuitive API with builder patterns
- **Multi-Provider Ready**: Works with any Open Responses compatible API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openresponses-rust = "0.1.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Basic Usage

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
    println!("Response: {:?}", response);
    
    Ok(())
}
```

### Streaming Responses

```rust
use openresponses_rust::{StreamingClient, CreateResponseBody, Input, Item};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = StreamingClient::new("your-api-key");
    
    let request = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Items(vec![
            Item::user_message("Count from 1 to 5")
        ])),
        ..Default::default()
    };
    
    let mut stream = client.stream_response(request).await?;
    
    while let Some(event) = stream.next().await {
        match event {
            Ok(openresponses_rust::StreamingEvent::OutputTextDelta { delta, .. }) => {
                print!("{}", delta);
            }
            Ok(openresponses_rust::StreamingEvent::Done) => break,
            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
    }
    
    Ok(())
}
```

### Function Calling

```rust
use openresponses_rust::{Client, CreateResponseBody, Input, Item, Tool, ToolChoiceParam};
use serde_json::json;

let get_weather = Tool::function("get_weather")
    .with_description("Get weather for a location")
    .with_parameters(json!({
        "type": "object",
        "properties": {
            "location": { "type": "string" }
        },
        "required": ["location"]
    }));

let request = CreateResponseBody {
    model: Some("gpt-4o".to_string()),
    input: Some(Input::Items(vec![
        Item::user_message("What's the weather in Paris?")
    ])),
    tools: Some(vec![get_weather]),
    tool_choice: Some(ToolChoiceParam::Simple(openresponses_rust::ToolChoice::Auto)),
    ..Default::default()
};
```

## Core Concepts

### Items

Items are the fundamental unit of context in Open Responses. They represent messages, tool calls, tool outputs, and reasoning.

```rust
use openresponses_rust::Item;

// Create different types of items
let user_msg = Item::user_message("Hello!");
let assistant_msg = Item::assistant_message("Hi there!");
let system_msg = Item::system_message("You are helpful.");
let dev_msg = Item::developer_message("Follow these rules.");
let reference = Item::reference("msg_123");
```

### Content Types

```rust
use openresponses_rust::InputContent;

// Text
let text = InputContent::text("Hello");

// Image
let image = InputContent::image_url("https://example.com/image.png");
let image_high_res = InputContent::image_url_with_detail(
    "https://example.com/image.png",
    openresponses_rust::ImageDetail::High
);

// File
let file = InputContent::file_url("https://example.com/doc.pdf");

// Video
let video = InputContent::video_url("https://example.com/video.mp4");
```

### Tools

```rust
use openresponses_rust::Tool;
use serde_json::json;

let tool = Tool::function("search")
    .with_description("Search the web")
    .with_parameters(json!({
        "type": "object",
        "properties": {
            "query": { "type": "string" }
        }
    }))
    .strict(true);
```

## API Reference

### Client

The `Client` type provides synchronous (blocking) API calls:

- `Client::new(api_key)` - Create a client with default base URL
- `Client::with_base_url(api_key, base_url)` - Create with custom base URL
- `client.create_response(request)` - Send a request and get a response

### StreamingClient

The `StreamingClient` provides SSE streaming:

- `StreamingClient::new(api_key)` - Create a streaming client
- `client.stream_response(request)` - Returns a stream of events

### Types

All types from the Open Responses specification are available:

- `CreateResponseBody` - Request body for creating responses
- `ResponseResource` - Response from the API
- `Item` - Core item types (messages, function calls, etc.)
- `InputContent` / `OutputContent` - Content types
- `StreamingEvent` - Streaming event types
- `Tool` / `ToolChoiceParam` - Tool definitions
- Enums: `MessageRole`, `MessageStatus`, `ToolChoice`, etc.

## Examples

See the `examples/` directory for complete working examples:

- `basic_usage.rs` - Simple API usage
- `streaming.rs` - Streaming responses
- `function_calling.rs` - Tool/Function calling

Run examples with:

```bash
export OPENAI_API_KEY="your-api-key"
cargo run --example basic_usage
cargo run --example streaming
cargo run --example function_calling
```

## Testing

Run the test suite:

```bash
cargo test
```

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Resources

- [Open Responses Specification](https://github.com/openresponses/spec)
- [Crates.io](https://crates.io/crates/openresponses-rust)
- [Documentation](https://docs.rs/openresponses-rust)
