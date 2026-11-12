use crate::app::App;

impl App {
    pub fn hotkey_q(&mut self) {
        self.exit = true;
    }
}
