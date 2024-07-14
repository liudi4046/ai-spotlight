use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

pub async fn query_openai(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;
    let request = OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;
    println!("OpenAI API response status: {}", status);
    println!("OpenAI API response body: {}", body);

    let response: OpenAIResponse = serde_json::from_str(&body)?;
    Ok(response.choices[0].message.content.trim().to_string())
}
