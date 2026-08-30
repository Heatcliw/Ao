use Ao::Command as Ao_Command;
use Ao::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Ao local CLI interface");

    let request = &mut Request::new();
    let filter = &mut FileFilter::new();
    let llama = &mut Llama::new();

    let client = &mut Client::new();
    let data = &mut String::new();
    let profile = &mut Profile::new();
    let system_prompt = &mut SystemPrompt::new(profile);
    loop {
        let line = get_data()?;

        match check_command(&line.0) {
            Ao_Command::App(Exit) => break,
            
            Ao_Command::App(Push) => ao_push(request, data),
            Ao_Command::App(Send) => publish(request, &client, &system_prompt).await?,
            
            Ao_Command::File(Add(path)) => ao_read_file(path, request, filter)?,
            Ao_Command::File(Ignore(path)) => ao_add_ignored_filter(filter, path),
            
            Ao_Command::Profile(Identity(value)) => ao_set_identity(profile, system_prompt, value),
            Ao_Command::Profile(Personality(value)) => ao_set_personality(profile, system_prompt, value),
            Ao_Command::Profile(Rule(value)) => ao_add_rule(system_prompt, value),
            Ao_Command::Profile(Style(value)) => ao_add_style(system_prompt, value),

            Ao_Command::Request(MaxTokens(tokens)) => ao_change_max_tokens(client, tokens),
            
            Ao_Command::Server(Context(context)) => ao_set_context(llama, context),
            Ao_Command::Server(Start) => ao_start_server(llama).await?,
            Ao_Command::Server(Stop) => ao_stop_server(llama)?,
            Ao_Command::Server(GPULayers(layers)) => ao_set_gpu_layers(llama, layers),

            Ao_Command::Unknown => {
                data.push_str(&line.0);
                data.push('\n');
            }
        }
    }

    Ok(())
}
