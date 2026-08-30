use crate::Request;

pub fn ao_push(request: &mut Request, data: &mut String) {
    request.push(&data);
    data.clear();

    if let Some(content) = &request.content {
        println!("[Request updated: {} bytes]", content.len());
    }

}