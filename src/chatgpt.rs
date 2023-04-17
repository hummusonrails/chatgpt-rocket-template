use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
pub struct ChatGPTRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub max_tokens: i32,
    pub n: i32,
    pub stop: Option<String>,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatGPTResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

pub async fn chatgpt_api_call(prompt: &str, api_key: &str) -> Result<String, Box<dyn Error>> {
  let client = Client::new();
  let api_endpoint = "https://api.openai.com/v1/chat/completions";
  let chatgpt_request = ChatGPTRequest {
      messages: vec![Message {
          role: String::from("user"),
          content: prompt.to_string(),
      }],
      model: String::from("gpt-3.5-turbo"),
      max_tokens: 150,
      n: 1,
      stop: None,
      temperature: 0.8,
  };

    let response: Response = client
        .post(api_endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&chatgpt_request)
        .send()
        .await?;

        let response_text = response.text().await?;
        let chatgpt_response: ChatGPTResponse = serde_json::from_str(&response_text)?;
        let content = &chatgpt_response
            .choices
            .get(0)
            .ok_or("No choices found in response")?
            .message
            .content;
    
        Ok(content.to_string())
}
