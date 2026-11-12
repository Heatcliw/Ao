
use crate::app::{App, MessageRole};
use std::io::Write;
use std::process::{Command, Stdio};

impl App {
    pub fn hotkey_c(&mut self) {
        let output = self
            .messages
            .iter()
            .map(|message| {
                let name = match message.role {
                    MessageRole::User => self.username.as_str(),
                    MessageRole::Assistant => "Ao",
                    MessageRole::System => "System",
                    MessageRole::Error => "Error",
                };

                format!("{name}: {}", message.content)
            })
            .collect::<Vec<_>>()
            .join("\n");

        match Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
            Ok(mut child) => {
                if let Some(mut stdin) = child.stdin.take() {
                    if let Err(error) = stdin.write_all(output.as_bytes()) {
                        log::error!("Clipboard write error: {error}");
                    }
                }

                match child.wait() {
                    Ok(status) if status.success() => {
                        log::info!("Output copied");
                    }
                    Ok(status) => log::error!("wl-copy failed: {status}"),
                    Err(error) => log::error!("Clipboard error: {error}"),
                }
            }
            Err(error) => log::error!("Failed to start wl-copy: {error}"),
        }
    }
}

