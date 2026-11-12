use log::{debug, error, info};

use crate::models;
use std::time::Instant;

use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

use crate::{Profile, system::SystemPrompt};

pub struct Client {
    pub http: HttpClient,
    pub address: String,
    pub model: String,
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
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: Option<usize>,
    completion_tokens: Option<usize>,
    total_tokens: Option<usize>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

pub struct MessageStats {
    pub bytes: usize,
    pub tokens: Option<usize>,
    pub tps: Option<f32>,
    pub generation_time: Option<f32>,
}

pub struct GenerationResult {
    pub content: String,
    pub stats: MessageStats,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
            address: "http://127.0.0.1:8080".to_string(),
            model: "Qwen2.5-Coder-7B-Instruct".to_string(),
            temperature: 0.7,
            max_tokens: 512,
        }
    }
    pub async fn send(
        &self,
        prompt: &str,
        system_prompt: &SystemPrompt,
    ) -> Result<GenerationResult, Box<dyn std::error::Error>> {
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

        let start = Instant::now();

        let http_response = self
            .http
            .post(format!("{}/v1/chat/completions", self.address))
            .json(&request)
            .send()
            .await?;

        let status = http_response.status();
        let body = http_response.text().await?;

        debug!("LLM HTTP status: {}", status);
        debug!("LLM raw response: {}", body);

        if !status.is_success() {
            error!("LLM request failed: HTTP {}: {}", status, body);
            return Err(format!("LLM returned HTTP {}", status).into());
        }

        let chat_response: ChatResponse = serde_json::from_str(&body)?;

        let content = chat_response
            .choices
            .first()
            .ok_or("LLM returned no choices")?
            .message
            .content
            .clone();

        if content.is_empty() {
            error!("LLM returned empty response");
        }

        let generation_time = start.elapsed().as_secs_f32();
        let bytes = content.len();

        let tokens = chat_response
            .usage
            .as_ref()
            .and_then(|usage| usage.completion_tokens);

        let tps = tokens.and_then(|tokens| {
            if generation_time > 0.0 {
                Some(tokens as f32 / generation_time)
            } else {
                None
            }
        });

        Ok(GenerationResult {
            content,
            stats: MessageStats {
                bytes,
                tokens,
                tps,
                generation_time: Some(generation_time),
            },
        })
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

    //     info!("[HTTP status: {}]", response.status());

    //     let body = response.text().await?;

    //     info!("[LLM raw response]");
    //     info!("{}", body);

    //     Ok(body)
    // }
    //
    pub fn model_name(&self) -> &str {
        models()
            .get(self.model.as_str())
            .copied()
            .unwrap_or("Unknown model")
    }
}
