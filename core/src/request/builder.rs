use crate::features::Request;

impl Request {
    pub fn new() -> Self {
        Self { content: None }
    }
    pub fn push(&mut self, data: &str) {
        self.content.get_or_insert_with(String::new).push_str(data);
    }
    pub fn clear(&mut self) {
        self.content = None;
    }
}
