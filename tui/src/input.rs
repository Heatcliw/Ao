use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;

use crate::app::App;

impl App {
    pub async fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.exit = true;
                }
                KeyCode::Enter if key.modifiers.contains(KeyModifiers::ALT) => {
                    self.input.push('\n');
                }
                KeyCode::Enter => {
                    let input = std::mem::take(&mut self.input);
                    self.output.push_str(&format!("{}: {}\n", self.username, input));

                    match self.core.execute(&input).await {
                        Ok(Some(result)) if result == "exit" => {
                            self.exit = true;
                        }
                        Ok(Some(result)) => {
                            self.output.push_str(&format!("Ao: {result}\n"));
                        }
                        Ok(None) => {}
                        Err(error) => {
                            self.output.push_str(&format!("Error: {error}\n"));
                        }
                    }
                }
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Esc => {
                    self.input.clear();
                }
                KeyCode::Up => {
                    self.scroll = self.scroll.saturating_sub(5);
                }
                KeyCode::Down => {
                    self.scroll = self.scroll.saturating_add(5);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
