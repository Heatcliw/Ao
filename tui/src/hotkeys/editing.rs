use crate::app::App;

impl App {
    pub fn hotkey_char(&mut self, c: char) {
        let cursor = self.buffer.cursor;
        self.buffer.content.insert(cursor, c);
        self.buffer.cursor += c.len_utf8();
        self.buffer.parse_path();
    }

    pub fn hotkey_backspace(&mut self) {
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

    pub fn hotkey_delete(&mut self) {
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
}
