use crate::{Client, Profile, features::Request, system::SystemPrompt};

pub async fn publish(
    request: &mut Request,
    client: &Client,
    system_prompt: &SystemPrompt,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = match &request.content {
        Some(content) => {
            let response = client.send(content, system_prompt).await?;
            response
        }
        None => return Ok("Request is empty".to_string()),
    };

    request.clear();

    Ok(response)
}
