use openresponses_rust::{Client, ClientError, CreateResponseBody, Input};
use std::env;

/// 이 예제는 운영 환경에서 권장되는 환경변수(Environment Variables)를 통한 설정 방법을 보여줍니다.
/// 실행 전: export API_URL=https://openrouter.ai/api
///        export API_KEY=your_key_here
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 환경변수에서 값을 읽어옵니다. 값이 없으면 기본값을 사용합니다.
    let api_key = env::var("API_KEY").expect("API_KEY 환경변수가 설정되어 있어야 합니다.");
    let api_url = env::var("API_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());
    
    println!("환경변수 설정 사용:");
    println!("  URL: {}", api_url);

    // 2. 읽어온 값을 바탕으로 클라이언트를 생성합니다.
    let client = Client::builder(api_key)
        .base_url(api_url)
        .build();

    let request = CreateResponseBody {
        model: Some("gpt-4o".to_string()),
        input: Some(Input::Single("환경변수 방식으로 연결되었습니다.".to_string())),
        ..Default::default()
    };

    match client.create_response(request).await {
        Ok(response) => {
            println!("응답 성공: {}", response.id);
        }
        Err(ClientError::ApiError {
            status_code,
            error,
            raw_body,
        }) => {
            eprintln!("API 오류 (HTTP {}):", status_code);
            if let Some(error) = error {
                eprintln!("  message: {}", error.message);
                if let Some(error_type) = error.error_type {
                    eprintln!("  type: {}", error_type);
                }
                if let Some(code) = error.code {
                    eprintln!("  code: {}", code);
                }
            } else {
                eprintln!("  body: {}", raw_body);
            }
        }
        Err(e) => {
            eprintln!("오류 발생: {}", e);
        }
    }

    Ok(())
}
