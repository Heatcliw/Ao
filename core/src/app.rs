use crate::{Client, FileFilter, Llama, PROJECT, Profile, ProjectInfo, Request, SystemPrompt};
use crate::{GenerationResult, MessageStats};

pub struct AoCore {
    pub request: Request,
    pub filter: FileFilter,
    pub llama: Llama,
    pub client: Client,
    pub profile: Profile,
    pub system_prompt: SystemPrompt,
    pub data: String,
    pub project_info: ProjectInfo,
}

fn text_result(content: String) -> GenerationResult {
    GenerationResult {
        stats: MessageStats {
            bytes: content.len(),
            tokens: None,
            tps: None,
            generation_time: None,
        },
        content,
    }
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
            project_info: PROJECT,
        }
    }
    pub async fn execute(
        &mut self,
        input: &str,
    ) -> Result<Option<GenerationResult>, Box<dyn std::error::Error>> {
        use crate::Command as Ao_Command;

        match crate::check_command(input) {
            Ao_Command::App(crate::AppCommand::Exit) => Ok(Some(text_result("exit".to_string()))),

            Ao_Command::App(crate::AppCommand::Help) => Ok(Some(text_result(crate::ao_help()))),

            Ao_Command::App(crate::AppCommand::Push) => {
                let msg = crate::ao_push(&mut self.request, &mut self.data);
                Ok(msg.map(text_result))
            }

            Ao_Command::App(crate::AppCommand::Send) => {
                let response =
                    crate::publish(&mut self.request, &self.client, &self.system_prompt).await?;

                Ok(Some(response))
            }

            Ao_Command::File(crate::FileCommand::Add(path)) => {
                let msg = crate::ao_read_file(path, &mut self.request, &mut self.filter)?;
                Ok(msg
                    .map(text_result)
                    .or_else(|| Some(text_result("File added to request".to_string()))))
            }

            Ao_Command::File(crate::FileCommand::Ignore(path)) => {
                crate::ao_add_ignored_filter(&mut self.filter, path);
                Ok(Some(text_result("File filter added".to_string())))
            }

            Ao_Command::Profile(crate::ProfileCommand::Identity(value)) => {
                crate::ao_set_identity(&mut self.profile, &mut self.system_prompt, value);
                Ok(Some(text_result("Identity updated".to_string())))
            }

            Ao_Command::Profile(crate::ProfileCommand::Personality(value)) => {
                crate::ao_set_personality(&mut self.profile, &mut self.system_prompt, value);
                Ok(Some(text_result("Personality updated".to_string())))
            }

            Ao_Command::Profile(crate::ProfileCommand::Rule(value)) => {
                crate::ao_add_rule(&mut self.system_prompt, value);
                Ok(Some(text_result("Rule added".to_string())))
            }

            Ao_Command::Profile(crate::ProfileCommand::Style(value)) => {
                crate::ao_add_style(&mut self.system_prompt, value);
                Ok(Some(text_result("Style added".to_string())))
            }

            Ao_Command::Request(crate::RequestCommand::MaxTokens(tokens)) => {
                crate::ao_change_max_tokens(&mut self.client, tokens);
                Ok(Some(text_result("Max tokens updated".to_string())))
            }

            Ao_Command::Server(crate::ServerCommand::Context(context)) => {
                crate::ao_set_context(&mut self.llama, context);
                Ok(Some(text_result("Context updated".to_string())))
            }

            Ao_Command::Server(crate::ServerCommand::Start) => {
                crate::ao_start_server(&mut self.llama).await?;
                Ok(Some(text_result("Server started".to_string())))
            }

            Ao_Command::Server(crate::ServerCommand::Stop) => {
                crate::ao_stop_server(&mut self.llama)?;
                Ok(Some(text_result("Server stopped".to_string())))
            }

            Ao_Command::Server(crate::ServerCommand::GPULayers(layers)) => {
                crate::ao_set_gpu_layers(&mut self.llama, layers);
                Ok(Some(text_result("GPU layers updated".to_string())))
            }

            Ao_Command::Unknown => {
                self.data.push_str(input);
                self.data.push('\n');
                Ok(None)
            }
        }
    }
}
