use crate::Profile;

pub struct SystemPrompt {
    pub identity: String,
    pub personality: String,

    pub rules: Vec<String>,
    pub style_rules: Vec<String>,
}

impl SystemPrompt {
    pub fn new(profile: &Profile) -> Self {
        Self {
            identity: profile.identity.clone(),
            personality: profile.personality.clone(),

            rules: Vec::new(),
            style_rules: Vec::new(),
        }
    }
    pub fn build(&self) -> String {
        let mut prompt = String::new();

        if !self.identity.is_empty() {
            prompt.push_str("IDENTITY:\n");
            prompt.push_str(&self.identity);
            prompt.push_str("\n\n");
        }

        if !self.personality.is_empty() {
            prompt.push_str("PERSONALITY:\n");
            prompt.push_str(&self.personality);
            prompt.push_str("\n\n");
        }

        for rule in &self.rules {
            prompt.push_str("- ");
            prompt.push_str(rule);
            prompt.push('\n');
        }

        for style_rule in &self.style_rules {
            prompt.push_str("- ");
            prompt.push_str(style_rule);
            prompt.push('\n');
        }

        // println!("\n[System Prompt]");
        // println!("{}", prompt);
        // println!("[End System Prompt]\n");
        
        prompt
    }
    
    pub fn update_from_profile(&mut self, profile: &Profile) {
        self.identity = profile.identity.clone();
        self.personality = profile.personality.clone();
    }
}
