use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

use crate::{Profile, system::SystemPrompt};

pub struct Client {
    pub http: HttpClient,
    pub address: String,
    pub temperature: f32,
    pub max_tokens: usize,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
    max_tokens: usize,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
            address: "http://127.0.0.1:8080".to_string(),
            temperature: 0.7,
            max_tokens: 512,
        }
    }
    pub async fn send(&self, prompt: &str, profile: &Profile, system_prompt: &SystemPrompt) -> Result<String, Box<dyn std::error::Error>> {
        let system_person = system_prompt.build();
        let request = ChatRequest {
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: &system_person,
                },
                ChatMessage {
                    role: "user",
                    content: prompt,
                },
            ],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        let response = self
            .http
            .post(format!("{}/v1/chat/completions", self.address))
            .json(&request)
            .send()
            .await?;

        let response: ChatResponse = response.json().await?;
        let content = response
            .choices
            .first()
            .ok_or("LLM returned no choices")?
            .message
            .content
            .clone();

        Ok(content)
    }
    // pub async fn send(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    //     let system_prompt = SystemPrompt::new();

    //     let request = ChatRequest {
    //         messages: vec![
    //             ChatMessage {
    //                 role: "system",
    //                 content: &system_prompt.0,
    //             },
    //             ChatMessage {
    //                 role: "user",
    //                 content: prompt,
    //             },
    //         ],
    //         temperature: self.temperature,
    //         max_tokens: self.max_tokens,
    //     };

    //     let response = self
    //         .http
    //         .post(format!("{}/v1/chat/completions", self.address))
    //         .json(&request)
    //         .send()
    //         .await?;

    //     println!("[HTTP status: {}]", response.status());

    //     let body = response.text().await?;

    //     println!("[LLM raw response]");
    //     println!("{}", body);

    //     Ok(body)
    // }
}
