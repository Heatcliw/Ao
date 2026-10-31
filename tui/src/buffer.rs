use std::path::PathBuf;

pub struct Buffer {
    pub content: String,
    pub cursor: usize,

    pub path: Option<PathBuf>,
    pub suggestions: Vec<PathBuf>,
    pub selected: usize,
}

impl Buffer {
    // Создание нового буфера
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor: 0,
            path: None,
            suggestions: Vec::new(),
            selected: 0,
        }
    }

    // Изменение позиции каретки
    pub fn set_cursor(&mut self, position: usize) {
        self.cursor = position.min(self.content.len());

        while !self.content.is_char_boundary(self.cursor) {
            self.cursor -= 1;
        }
    }

    // Разбор пути из текущего содержимого
    pub fn parse_path(&mut self) {
        let start = self.content[..self.cursor]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);

        let input = &self.content[start..self.cursor];

        self.path = if input.is_empty() {
            None
        } else {
            Some(PathBuf::from(input))
        };
    }
}
