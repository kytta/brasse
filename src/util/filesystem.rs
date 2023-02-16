use std::path::PathBuf;
use std::{fs, io};

pub fn get_directories(path: PathBuf) -> Result<Vec<String>, io::Error> {
    let mut result: Vec<String> = Vec::new();

    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        let entry_name = path.file_name().unwrap().to_str().unwrap();

        if entry_name.starts_with('.') {
            continue;
        }

        result.push(entry_name.to_string());
    }

    Ok(result)
}
