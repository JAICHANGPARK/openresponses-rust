use openresponses_rust::{Client, CreateResponseBody, Input, Item};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "your-api-key".to_string());
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());
    
    let client = Client::builder(api_key).base_url(api_url).build();

    // Turn 1
    println!("Turn 1: Asking for a prime number...");
    let request1 = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Single("Provide a prime number less than 50".to_string())),
        ..Default::default()
    };

    let response1 = client.create_response(request1).await?;
    println!("Response 1 ID: {}", response1.id);
    
    // Turn 2: Follow up
    println!("\nTurn 2: Asking to multiply by 2...");
    let request2 = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Single("Multiply it by 2".to_string())),
        previous_response_id: Some(response1.id),
        ..Default::default()
    };

    let response2 = client.create_response(request2).await?;
    println!("Response 2 ID: {}", response2.id);
    
    for item in response2.output {
        if let Item::Message { content, .. } = item {
            for part in content {
                if let openresponses_rust::MessageContent::OutputText { text, .. } = part {
                    println!("Assistant: {}", text);
                }
            }
        }
    }

    Ok(())
}
