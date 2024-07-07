use std::fs;
use std::io;
use std::path::Path;

pub fn read_dir_recursive(path: &Path) -> io::Result<Vec<String>> {
    let mut entries = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                entries.extend(read_dir_recursive(&path)?);
            } else if let Some(name) = path.file_name() {
                if let Some(_name_str) = name.to_str() {
                    entries.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(entries)
}
