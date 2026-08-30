use std::{fs, path::Path};
use anyhow::{anyhow, Result};

pub struct FileData {
    pub name: String,
    pub path: String,
    pub extension: Option<String>,
    pub data: Vec<u8>,
}

pub struct FileFilter {
    pub patterns: Vec<String>,
}

impl FileFilter {
    fn is_ignored(&self, path: &Path) -> bool {
        for pattern in &self.patterns  {
            if path == Path::new(pattern) {
                return true;
            }
        }
        false
    }
    pub fn add(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }
    pub fn list(&self) -> &Vec<String> {
        &self.patterns
    }
    pub fn new() -> Self {
        Self { patterns: Vec::new() }
    }
}

pub fn read<P: AsRef<Path>>(path: P, filter: &FileFilter,) -> Result<Vec<FileData>> {
    let path = path.as_ref();
    if filter.is_ignored(path) {
        return Ok(vec![]);
    }
    if path.is_file() {
        return Ok(vec![read_file(path)?]);
    }

    if path.is_dir() {
        let mut files = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if filter.is_ignored(&file_path) {
                continue;
            }

            if file_path.is_file() {
                files.push(read_file(&file_path)?);
            } else if file_path.is_dir() {
                files.extend(read(&file_path, filter)?);
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