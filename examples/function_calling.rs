use openresponses_rust::{Client, CreateResponseBody, Input, Item, Tool, ToolChoiceParam};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "your-api-key".to_string());
    let client = Client::new(api_key);

    let get_weather_tool = Tool::function("get_weather")
        .with_description("Get the current weather for a location")
        .with_parameters(json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "The unit of temperature"
                }
            },
            "required": ["location"]
        }));

    let request = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Items(vec![
            Item::system_message("You are a helpful assistant. Use the available tools when needed."),
            Item::user_message("What's the weather like in San Francisco?"),
        ])),
        tools: Some(vec![get_weather_tool]),
        tool_choice: Some(ToolChoiceParam::Simple(openresponses_rust::ToolChoice::Auto)),
        ..Default::default()
    };

    match client.create_response(request).await {
        Ok(response) => {
            println!("Response ID: {}", response.id);
            
            for item in response.output {
                match item {
                    Item::Message { content, .. } => {
                        for part in content {
                            if let openresponses_rust::MessageContent::OutputText { text, .. } = part {
                                println!("Assistant: {}", text);
                            }
                        }
                    }
                    Item::FunctionCall { name, arguments, .. } => {
                        println!("\nFunction Call:");
                        println!("  Name: {}", name);
                        println!("  Arguments: {}", arguments);
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
