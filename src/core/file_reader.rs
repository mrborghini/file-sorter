use std::fs;

pub fn read_files_in_current_dir() -> Vec<String>{
    let paths = fs::read_dir("./").unwrap();

    let mut files: Vec<String> = Vec::new();

    for entry in paths {
        match entry {
            Ok(dir_entry) => match dir_entry.metadata() {
                Ok(metadata) => {
                    if metadata.is_file() {
                        let file_name = dir_entry.path().display().to_string();
                        if !file_name.starts_with("./.") && !file_name.starts_with(".\\.") {
                            files.push(file_name);
                        }
                    }
                }
                Err(e) => println!("Failed to get metadata: {}", e),
            },
            Err(e) => println!("Failed to read entry: {}", e),
        }
    }

    return files;
}