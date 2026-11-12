use std::collections::HashMap;

pub fn models() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("qwen2.5-coder-7b", "Qwen2.5-Coder-7B"),
        ("llama3.1-8b", "Llama 3.1 8B"),
    ])
}
