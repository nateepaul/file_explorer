use std::fs;

pub fn list_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                files.push(entry.path().display().to_string());
            }
        }
    }

    files
}