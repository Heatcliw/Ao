use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Paragraph},
};
use ratatui::style::{Color, Style,};
use ratatui::text::{Span, Line};
use ao_core::AoCore;

use crate::theme::{Theme, THEMES};

use std::io;

pub fn fmt_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

pub struct App {
   pub counter: u8,
   pub exit: bool,
   pub scroll: u16,
   pub theme_index: usize,
   pub core: AoCore,
   pub input: String,
   pub output: String,
   pub username: String,
}

impl App {
    pub fn new() -> Self {
        let username = std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "user".to_string());

        Self {
            counter: 0,
            core: AoCore::new(),
            exit: false,
            scroll: 0,
            theme_index:2,
            input: String::new(),
            output: String::new(),
            username,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }
}
