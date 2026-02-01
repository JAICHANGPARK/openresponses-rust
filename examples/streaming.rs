use open_responses::{StreamingClient, CreateResponseBody, Input, Item};
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
                    open_responses::StreamingEvent::OutputTextDelta { delta, .. } => {
                        print!("{}", delta);
                    }
                    open_responses::StreamingEvent::ResponseCompleted { .. } => {
                        println!("\n\n[Response completed]");
                    }
                    open_responses::StreamingEvent::ResponseFailed { .. } => {
                        println!("\n\n[Response failed]");
                    }
                    open_responses::StreamingEvent::Done => {
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
