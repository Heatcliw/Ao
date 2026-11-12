
use crate::app::{App, MessageRole, MessageStats, UiMessage};

impl App {
    pub fn hotkey_alt_enter(&mut self) {
        let cursor = self.buffer.cursor;
        self.buffer.content.insert(cursor, '\n');
        self.buffer.cursor += 1;
        self.buffer.parse_path();
    }

    pub async fn hotkey_enter(&mut self) {
        let input = std::mem::take(&mut self.buffer.content);

        self.buffer.cursor = 0;
        self.buffer.path = None;
        self.buffer.suggestions.clear();
        self.buffer.selected = 0;

        if input.trim().is_empty() {
            return;
        }

        // Сохраняем сообщение пользователя.
        self.messages.push(UiMessage {
            role: MessageRole::User,
            content: input.clone(),
            stats: MessageStats {
                bytes: input.len(),
                tokens: None,
                tps: None,
                generation_time: None,
            },
        });

        match self.core.execute(&input).await {
            // Команда выхода.
            Ok(Some(result)) if result.content == "exit" => {
                self.exit = true;
            }

            // Ответ Ao.
            Ok(Some(result)) => {
                self.messages.push(UiMessage {
                    role: MessageRole::Assistant,
                    content: result.content,
                    stats: MessageStats {
                        bytes: result.stats.bytes,
                        tokens: result.stats.tokens,
                        tps: result.stats.tps,
                        generation_time: result.stats.generation_time,
                    },
                });
            }

            // Команда не вернула сообщение.
            Ok(None) => {}

            // Ошибка выполнения.
            Err(error) => {
                let error = error.to_string();

                self.messages.push(UiMessage {
                    role: MessageRole::Error,
                    content: error.clone(),
                    stats: MessageStats {
                        bytes: error.len(),
                        tokens: None,
                        tps: None,
                        generation_time: None,
                    },
                });
            }
        }
    }
}

