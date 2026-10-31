use crossterm::event::{
    Event, EventStream, KeyCode, KeyModifiers,
};
use futures_util::StreamExt;
use std::io;

use crate::app::App;

impl App {
    pub async fn handle_events(
        &mut self,
        events: &mut EventStream,
    ) -> io::Result<()> {
        let Some(event) = events.next().await else {
            return Ok(());
        };

        match event.map_err(io::Error::other)? {
            Event::Key(key) => {
                match key.code {
                    // Копирование output
                    KeyCode::Char('c')
                        if key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        use std::io::Write;
                        use std::process::{Command, Stdio};

                        match Command::new("wl-copy")
                            .stdin(Stdio::piped())
                            .spawn()
                        {
                            Ok(mut child) => {
                                if let Some(mut stdin) = child.stdin.take() {
                                    if let Err(error) =
                                        stdin.write_all(self.output.as_bytes())
                                    {
                                        log::error!("Clipboard write error: {error}");
                                    }
                                }

                                match child.wait() {
                                    Ok(status) if status.success() => {
                                        log::info!("Output copied");
                                    }
                                    Ok(status) => {
                                        log::error!("wl-copy failed: {status}");
                                    }
                                    Err(error) => {
                                        log::error!("Clipboard error: {error}");
                                    }
                                }
                            }
                            Err(error) => {
                                log::error!("Failed to start wl-copy: {error}");
                            }
                        }
                    }

                    // Выход
                    KeyCode::Char('q')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        self.exit = true;
                    }

                    // Перенос строки
                    KeyCode::Enter
                        if key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        let cursor = self.buffer.cursor;
                        self.buffer.content.insert(cursor, '\n');
                        self.buffer.cursor += 1;
                        self.buffer.parse_path();
                    }

                    // Выполнение команды
                    KeyCode::Enter => {
                        let input = std::mem::take(&mut self.buffer.content);
                        self.buffer.cursor = 0;
                        self.buffer.path = None;
                        self.buffer.suggestions.clear();
                        self.buffer.selected = 0;

                        self.output.push_str(&format!(
                            "{}: {}\n",
                            self.username,
                            input
                        ));

                        match self.core.execute(&input).await {
                            Ok(Some(result)) if result == "exit" => {
                                self.exit = true;
                            }
                            Ok(Some(result)) => {
                                self.output.push_str(&format!("Ao: {result}\n"));
                            }
                            Ok(None) => {}
                            Err(error) => {
                                self.output.push_str(&format!(
                                    "Error: {error}\n"
                                ));
                            }
                        }
                    }

                    // Ввод символа
                    KeyCode::Char(c) => {
                        let cursor = self.buffer.cursor;

                        self.buffer.content.insert(cursor, c);
                        self.buffer.cursor += c.len_utf8();
                        self.buffer.parse_path();
                    }

                    // Удаление слева
                    KeyCode::Backspace => {
                        let cursor = self.buffer.cursor;

                        if cursor > 0 {
                            let previous = self.buffer.content[..cursor]
                                .char_indices()
                                .last()
                                .map(|(index, _)| index)
                                .unwrap_or(0);

                            self.buffer.content.drain(previous..cursor);
                            self.buffer.cursor = previous;
                            self.buffer.parse_path();
                        }
                    }

                    // Удаление справа
                    KeyCode::Delete => {
                        let cursor = self.buffer.cursor;

                        if cursor < self.buffer.content.len() {
                            let next = self.buffer.content[cursor..]
                                .char_indices()
                                .nth(1)
                                .map(|(index, _)| cursor + index)
                                .unwrap_or(self.buffer.content.len());

                            self.buffer.content.drain(cursor..next);
                            self.buffer.parse_path();
                        }
                    }

                    // Каретка влево
                    KeyCode::Left => {
                        let cursor = self.buffer.cursor;

                        if cursor > 0 {
                            let previous = self.buffer.content[..cursor]
                                .char_indices()
                                .last()
                                .map(|(index, _)| index)
                                .unwrap_or(0);

                            self.buffer.set_cursor(previous);
                        }
                    }

                    // Каретка вправо
                    KeyCode::Right => {
                        let cursor = self.buffer.cursor;

                        if cursor < self.buffer.content.len() {
                            let next = self.buffer.content[cursor..]
                                .char_indices()
                                .nth(1)
                                .map(|(index, _)| cursor + index)
                                .unwrap_or(self.buffer.content.len());

                            self.buffer.set_cursor(next);
                        }
                    }

                    // Очистка ввода
                    KeyCode::Esc => {
                        self.buffer.content.clear();
                        self.buffer.cursor = 0;
                        self.buffer.path = None;
                        self.buffer.suggestions.clear();
                        self.buffer.selected = 0;
                    }

                    // Прокрутка
                    KeyCode::Up => {
                        self.scroll = self.scroll.saturating_sub(5);
                    }

                    KeyCode::Down => {
                        self.scroll = self.scroll.saturating_add(5);
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        Ok(())
    }
}
