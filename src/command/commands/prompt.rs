use crate::{Profile, SystemPrompt};

pub fn ao_set_identity(profile: &mut Profile,system_prompt: &mut SystemPrompt, value: String) {
    profile.identity = value;
    system_prompt.update_from_profile(profile);
}

pub fn ao_set_personality(profile: &mut Profile, system_prompt: &mut SystemPrompt, value: String) {
    profile.personality = value;
    system_prompt.update_from_profile(profile);
}

pub fn ao_add_rule(system_prompt: &mut SystemPrompt, value: String) {
    system_prompt.rules.push(value);
}

pub fn ao_add_style(system_prompt: &mut SystemPrompt, value: String) {
    system_prompt.style_rules.push(value);
}