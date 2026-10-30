pub mod command;
pub mod input;
pub mod llm;
pub mod logger;
pub mod profile;
pub mod prompt;
pub mod request;

pub mod file;

pub use command::{
    AppCommand, Command, FileCommand, MemoryCommand, ProfileCommand, RequestCommand, ServerCommand,
    check_command,
};

pub use command::commands::{
    ao_add_ignored_filter, ao_add_rule, ao_add_style, ao_change_max_tokens, ao_help, ao_push,
    ao_read_file, ao_set_context, ao_set_gpu_layers, ao_set_identity, ao_set_personality,
    ao_start_server, ao_stop_server,
};

pub use command::{
    AppCommand::{Exit, Help, Push, Send},
    FileCommand::{Add, Ignore},
    ProfileCommand::{Identity, Personality, Rule, Style},
    RequestCommand::MaxTokens,
    ServerCommand::{Context, GPULayers, Start, Stop},
};

pub use input::*;
pub use llm::*;
pub use logger::*;
pub use profile::*;
pub use prompt::*;
pub use request::*;

pub use file::*;

pub mod app;
pub use app::AoCore;
