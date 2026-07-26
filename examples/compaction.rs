use openresponses_rust::{
    Client, CompactResponseBody, Input, Item, MessagePhase,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "your-api-key".to_string());
    let _client = Client::new(api_key);

    let request = CompactResponseBody {
        model: "gpt-5".to_string(),
        input: Some(Input::Items(vec![
            Item::user_message("Please explain quantum computing in detail."),
            Item::assistant_message_with_phase(
                "Thinking through quantum mechanical principles...",
                MessagePhase::Commentary,
            ),
            Item::assistant_message_with_phase(
                "Quantum computing utilizes qubits...",
                MessagePhase::FinalAnswer,
            ),
        ])),
        ..Default::default()
    };

    println!("Sending compaction request...");
    // In real usage with valid API key:
    // let compacted = client.compact_response(request).await?;
    // println!("Compacted output items: {:?}", compacted.output);

    println!("Constructed compaction request: {:?}", request);
    Ok(())
}
