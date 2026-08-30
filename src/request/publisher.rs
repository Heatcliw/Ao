use std::println;

use crate::{Client, Profile, features::Request, system::SystemPrompt};

pub async fn publish(request: &mut Request, client: &Client, system_prompt: &SystemPrompt) -> Result<(), Box<dyn std::error::Error>> {
    match &request.content {
        Some(content) => {
            println!("Publishing request");
            println!("{}", content);

            
            println!("[Sending to LLM...]");

            let response = client.send(content, &system_prompt).await?;

            println!("Ao: {}", response);
        }

        None => {
            println!("Request is empty");
        }
    }

    request.clear();
    
    Ok(())
}
