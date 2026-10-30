use crate::{Client, FileFilter, Llama, Profile, Request, SystemPrompt};

pub struct AoCore {
    pub request: Request,
    pub filter: FileFilter,
    pub llama: Llama,
    pub client: Client,
    pub profile: Profile,
    pub system_prompt: SystemPrompt,
    pub data: String,
}

impl AoCore {
    pub fn new() -> Self {
        let profile = Profile::new();
        let system_prompt = SystemPrompt::new(&profile);

        Self {
            request: Request::new(),
            filter: FileFilter::new(),
            llama: Llama::new(),
            client: Client::new(),
            profile,
            system_prompt,
            data: String::new(),
        }
    }
    pub async fn execute(
        &mut self,
        input: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        use crate::Command as Ao_Command;

        match crate::check_command(input) {
            Ao_Command::App(crate::AppCommand::Exit) => Ok(Some("exit".to_string())),
            Ao_Command::App(crate::AppCommand::Help) => {
                Ok(Some(crate::ao_help()))
            }
            Ao_Command::App(crate::AppCommand::Push) => {
                let msg = crate::ao_push(&mut self.request, &mut self.data);
                Ok(msg.or_else(|| Some("Request updated".to_string())))
            }
            Ao_Command::App(crate::AppCommand::Send) => {
                let response =
                    crate::publish(&mut self.request, &self.client, &self.system_prompt).await?;

                Ok(Some(response))
            }
            Ao_Command::File(crate::FileCommand::Add(path)) => {
                let msg = crate::ao_read_file(path, &mut self.request, &mut self.filter)?;
                Ok(msg.or_else(|| Some("File added to request".to_string())))
            }
            Ao_Command::File(crate::FileCommand::Ignore(path)) => {
                crate::ao_add_ignored_filter(&mut self.filter, path);
                Ok(Some("File filter added".to_string()))
            }
            Ao_Command::Profile(crate::ProfileCommand::Identity(value)) => {
                crate::ao_set_identity(&mut self.profile, &mut self.system_prompt, value);
                Ok(Some("Identity updated".to_string()))
            }
            Ao_Command::Profile(crate::ProfileCommand::Personality(value)) => {
                crate::ao_set_personality(&mut self.profile, &mut self.system_prompt, value);
                Ok(Some("Personality updated".to_string()))
            }
            Ao_Command::Profile(crate::ProfileCommand::Rule(value)) => {
                crate::ao_add_rule(&mut self.system_prompt, value);
                Ok(Some("Rule added".to_string()))
            }
            Ao_Command::Profile(crate::ProfileCommand::Style(value)) => {
                crate::ao_add_style(&mut self.system_prompt, value);
                Ok(Some("Style added".to_string()))
            }
            Ao_Command::Request(crate::RequestCommand::MaxTokens(tokens)) => {
                crate::ao_change_max_tokens(&mut self.client, tokens);
                Ok(Some("Max tokens updated".to_string()))
            }
            Ao_Command::Server(crate::ServerCommand::Context(context)) => {
                crate::ao_set_context(&mut self.llama, context);
                Ok(Some("Context updated".to_string()))
            }
            Ao_Command::Server(crate::ServerCommand::Start) => {
                crate::ao_start_server(&mut self.llama).await?;
                Ok(Some("Server started".to_string()))
            }
            Ao_Command::Server(crate::ServerCommand::Stop) => {
                crate::ao_stop_server(&mut self.llama)?;
                Ok(Some("Server stopped".to_string()))
            }
            Ao_Command::Server(crate::ServerCommand::GPULayers(layers)) => {
                crate::ao_set_gpu_layers(&mut self.llama, layers);
                Ok(Some("GPU layers updated".to_string()))
            }
            Ao_Command::Unknown => {
                self.data.push_str(input);
                self.data.push('\n');
                Ok(None)
            }
        }
    }
}
