use crate::app::App;

impl App {
    pub fn hotkey_left(&mut self) {
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

    pub fn hotkey_right(&mut self) {
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

    pub fn hotkey_esc(&mut self) {
        self.buffer.content.clear();
        self.buffer.cursor = 0;
        self.buffer.path = None;
        self.buffer.suggestions.clear();
        self.buffer.selected = 0;
    }

    pub fn hotkey_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(5);
    }

    pub fn hotkey_down(&mut self) {
        self.scroll = self.scroll.saturating_add(5);
    }
}
