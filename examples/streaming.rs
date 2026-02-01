use openresponses_rust::{StreamingClient, CreateResponseBody, Input, Item};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "your-api-key".to_string());
    let client = StreamingClient::new(api_key);

    let request = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Items(vec![
            Item::system_message("You are a helpful assistant."),
            Item::user_message("Count from 1 to 5 slowly."),
        ])),
        temperature: Some(0.7),
        ..Default::default()
    };

    println!("Streaming response:\n");

    let mut stream = client.stream_response(request).await?;

    while let Some(event_result) = stream.next().await {
        match event_result {
            Ok(event) => {
                match event {
                    openresponses_rust::StreamingEvent::OutputTextDelta { delta, .. } => {
                        print!("{}", delta);
                    }
                    openresponses_rust::StreamingEvent::ResponseCompleted { .. } => {
                        println!("\n\n[Response completed]");
                    }
                    openresponses_rust::StreamingEvent::ResponseFailed { .. } => {
                        println!("\n\n[Response failed]");
                    }
                    openresponses_rust::StreamingEvent::Done => {
                        println!("\n[Stream ended]");
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
            }
        }
    }

    Ok(())
}
