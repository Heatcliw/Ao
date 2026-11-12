use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use futures_util::StreamExt;
use std::io;

use crate::app::App;

impl App {
    pub async fn handle_events(&mut self, events: &mut EventStream) -> io::Result<()> {
        let Some(event) = events.next().await else {
            return Ok(());
        };

        match event.map_err(io::Error::other)? {
            Event::Key(key) => {
                match key.code {
                    // Alt+C — копирование output
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::ALT) => {
                        self.hotkey_c();
                    }

                    // Ctrl+Q — выход
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.hotkey_q();
                    }

                    // Alt+Enter — перенос строки
                    KeyCode::Enter if key.modifiers.contains(KeyModifiers::ALT) => {
                        self.hotkey_alt_enter();
                    }

                    // Enter — выполнение команды
                    KeyCode::Enter => {
                        self.hotkey_enter().await;
                    }

                    // Ввод символа
                    KeyCode::Char(c) => {
                        self.hotkey_char(c);
                    }

                    // Удаление
                    KeyCode::Backspace => {
                        self.hotkey_backspace();
                    }

                    KeyCode::Delete => {
                        self.hotkey_delete();
                    }

                    // Перемещение каретки
                    KeyCode::Left => {
                        self.hotkey_left();
                    }

                    KeyCode::Right => {
                        self.hotkey_right();
                    }

                    // Очистка ввода
                    KeyCode::Esc => {
                        self.hotkey_esc();
                    }

                    // Прокрутка
                    KeyCode::Up => {
                        self.hotkey_up();
                    }

                    KeyCode::Down => {
                        self.hotkey_down();
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        Ok(())
    }
}
