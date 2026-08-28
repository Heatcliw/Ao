use std::{fs, path::Path};
use anyhow::{anyhow, Result};

pub struct FileData {
    pub name: String,
    pub path: String,
    pub extension: Option<String>,
    pub data: Vec<u8>,
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<FileData>> {
    let path = path.as_ref();

    if path.is_file() {
        return Ok(vec![read_file(path)?]);
    }

    if path.is_dir() {
        let mut files = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_file() {
                files.push(read_file(&file_path)?);
            }
        }

        return Ok(files);
    }

    Err(anyhow!("Path does not exist: {}", path.display()))
}

fn read_file(path: &Path) -> Result<FileData> {
    let data = fs::read(path)?;

    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_string);

    Ok(FileData {
        name,
        path: path.to_string_lossy().to_string(),
        extension,
        data,
    })
}