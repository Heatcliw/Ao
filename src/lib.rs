pub mod command;
pub mod input;
pub mod llm;
pub mod profile;
pub mod prompt;
pub mod request;

pub mod file;

pub use command::{
    check_command,
    Command,
    AppCommand,
    FileCommand,
    ProfileCommand,
    ServerCommand,
    RequestCommand,
    MemoryCommand,
    
};

pub use command::commands::{
    ao_push, ao_read_file, ao_change_max_tokens, 
    ao_add_rule, ao_add_style, ao_set_identity, 
    ao_set_personality, ao_start_server, ao_stop_server,
    ao_add_ignored_filter, ao_set_context, ao_set_gpu_layers,
};

pub use command::{
    AppCommand::{Exit, Push, Send},
    FileCommand::{Add, Ignore},
    ProfileCommand::{Identity, Personality, Rule, Style},
    ServerCommand::{Start, Stop, Context, GPULayers},
    RequestCommand::MaxTokens,
};

pub use input::*;
pub use llm::*;
pub use profile::*;
pub use prompt::*;
pub use request::*;

pub use file::*;