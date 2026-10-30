use crate::Request;

pub fn ao_push(request: &mut Request, data: &mut String) -> Option<String> {
    request.push(&data);
    data.clear();

    request
        .content
        .as_ref()
        .map(|content| format!("[Request updated: {} bytes]", content.len()))
}
