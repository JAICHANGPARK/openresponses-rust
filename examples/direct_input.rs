use openresponses_rust::{Client, CreateResponseBody, Input, Item};

/// 이 예제는 환경변수를 사용하지 않고 코드에 직접 API URL과 키를 입력하는 방법을 보여줍니다.
/// 로컬에서 실행 중인 LM Studio나 특정 프록시 서버에 연결할 때 유용합니다.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. URL과 API 키를 문자열 리터럴로 직접 정의합니다.
    // /v1을 붙이지 않아도 라이브러리가 자동으로 처리합니다.
    let api_url = "http://localhost:1234"; 
    let api_key = "lm-studio"; // 로컬 서버는 보통 아무 키나 허용합니다.
    
    println!("직접 입력된 URL로 연결 시도: {}", api_url);

    // 2. ClientBuilder를 사용하여 클라이언트를 생성합니다.
    let client = Client::builder(api_key)
        .base_url(api_url)
        .build();

    let request = CreateResponseBody {
        model: Some("openai/gpt-oss-20b".to_string()),
        input: Some(Input::Single("안녕하세요! 직접 입력 방식으로 연결되었습니다.".to_string())),
        ..Default::default()
    };

    match client.create_response(request).await {
        Ok(response) => {
            println!("성공! 응답 ID: {}", response.id);
        }
        Err(e) => {
            eprintln!("연결 실패: {}. 서버가 실행 중인지 확인하세요.", e);
        }
    }

    Ok(())
}