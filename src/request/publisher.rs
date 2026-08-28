use std::println;

use crate::{Client, Profile, features::Request, system::SystemPrompt};

pub async fn publish(request: &Request, client: &Client, profile: &Profile, system_prompt: &SystemPrompt) -> Result<(), Box<dyn std::error::Error>> {
    match &request.content {
        Some(content) => {
            println!("Publishing request");
            println!("{}", content);

            println!("[Sending to LLM...]");

            let response = client.send(content, profile, &system_prompt).await?;

            println!("Ao: {}", response);
        }

        None => {
            println!("Request is empty");
        }
    }

    Ok(())
}
