use crate::{FileFilter, Request, read};

pub fn ao_read_file(
    path: String,
    request: &mut Request,
    filter: &FileFilter,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let files = read(path, &filter)?;

    for file in &files {
        let content = String::from_utf8_lossy(&file.data);

        request.push(&format!(
            "\n--- FILE: {} ---\n{}\n--- END FILE ---\n",
            file.path, content
        ));
    }

    Ok(request
        .content
        .as_ref()
        .map(|content| format!("[Request updated: {} bytes]", content.len())))
}

pub fn ao_add_ignored_filter(filter: &mut FileFilter, pattern: String) {
    filter.add(pattern);
    let _list = filter.list();
}
