use crate::llm::client::{GenerationResult, MessageStats};
use crate::{Client, Profile, features::Request, system::SystemPrompt};
pub async fn publish(
    request: &mut Request,
    client: &Client,
    system_prompt: &SystemPrompt,
) -> Result<GenerationResult, Box<dyn std::error::Error>> {
    let response = match &request.content {
        Some(content) => client.send(content, system_prompt).await?,
        None => {
            return Ok(GenerationResult {
                content: "Request is empty".to_string(),
                stats: MessageStats {
                    bytes: 0,
                    tokens: None,
                    tps: None,
                    generation_time: None,
                },
            });
        }
    };

    request.clear();

    Ok(response)
}
