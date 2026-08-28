use Ao::Command as Ao_Command;
use Ao::system::SystemPrompt;
use Ao::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Ao input test");

    let mut request = Request::new();
    let client = Client::new();
    let mut data = String::new();
    let profile = &mut Profile::new();
    let mut system_prompt = SystemPrompt::new(profile);
    loop {
        let line = get_data()?;

        match check_command(&line.0) {
            Ao_Command::Exit => break,

            Ao_Command::Push => {
                request.push(&data);
                data.clear();

                if let Some(content) = &request.content {
                    println!("[Request updated: {} bytes]", content.len());
                }
            }

            Ao_Command::Send => {
                publish(&request, &client, profile, &system_prompt).await?;
            }

            Ao_Command::File(path) => {
                let files = read(path)?;

                for file in &files {
                    let content = String::from_utf8_lossy(&file.data);

                    request.push(&format!(
                        "\n--- FILE: {} ---\n{}\n--- END FILE ---\n",
                        file.path,
                        content
                    ));   
                }
                if let Some(content) = &request.content {
                    println!("[Request updated: {} bytes]", content.len());
                }
            }
            Ao_Command::Identity(value) => {
                profile.identity = value;
            }

            Ao_Command::Personality(value) => {
                profile.personality = value;
            }

            Ao_Command::Rule(value) => {
                system_prompt.rules.push(value);
            }

            Ao_Command::Style(value) => {
                system_prompt.style_rules.push(value);
            }
            Ao_Command::Unknown => {
                data.push_str(&line.0);
                data.push('\n');
            }
        }
    }

    Ok(())
}
