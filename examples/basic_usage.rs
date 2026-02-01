use open_responses::{Client, CreateResponseBody, Input, Item};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "your-api-key".to_string());
    let client = Client::new(api_key);

    let request = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Items(vec![
            Item::system_message("You are a helpful assistant."),
            Item::user_message("What is the capital of France?"),
        ])),
        temperature: Some(0.7),
        max_output_tokens: Some(100),
        ..Default::default()
    };

    match client.create_response(request).await {
        Ok(response) => {
            println!("Response ID: {}", response.id);
            println!("Status: {}", response.status);
            
            if let Some(usage) = response.usage {
                println!("\nToken Usage:");
                println!("  Input: {}", usage.input_tokens);
                println!("  Output: {}", usage.output_tokens);
                println!("  Total: {}", usage.total_tokens);
            }

            println!("\nOutput:");
            for (i, item) in response.output.iter().enumerate() {
                println!("  Item {}: {:?}", i, item);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
