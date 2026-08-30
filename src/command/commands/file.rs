use std::println;

use crate::{Request, FileFilter, read};

pub fn ao_read_file(path: String, request: &mut Request, filter: &FileFilter,) -> Result<(), Box<dyn std::error::Error>>{
    
    let files = read(path, &filter)?;

    for file in &files {
        let content = String::from_utf8_lossy(&file.data);

        request.push(&format!(
            "\n--- FILE: {} ---\n{}\n--- END FILE ---\n",
            file.path,
            content
        ));   
    }
    
    if let Some(content) = &request.content {
        println!("[Request updated: {} bytes]", content.len());
    }
    
    Ok(())
}

pub fn ao_add_ignored_filter(filter: &mut FileFilter, pattern: String) {
    filter.add(pattern);
    let list = filter.list();

    for pattern in list {
        println!("{}", pattern);
    }
}