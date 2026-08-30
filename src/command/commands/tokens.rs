use crate::Client;

pub fn ao_change_max_tokens(client: &mut Client , tokens: usize) {
    client.max_tokens = tokens;
    println!("[Max tokens: {}]", client.max_tokens);
}