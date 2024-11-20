use std::{
    fs::{self, DirEntry},
    io::Error,
};

pub struct FileReader {
    pub files: Vec<String>,
}

impl FileReader {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    fn add_file(&mut self, dir_entry: DirEntry) {
        let file_name = dir_entry.path().display().to_string();
        if !file_name.starts_with("./.") && !file_name.starts_with(".\\.") {
            self.files.push(file_name);
        }
    }

    fn handle_dir_entry(&mut self, dir_entry: DirEntry) {
        match dir_entry.metadata() {
            Ok(metadata) => {
                if metadata.is_file() {
                    self.add_file(dir_entry);
                }
            }
            Err(e) => println!("Failed to get metadata: {}", e),
        }
    }

    fn handle_entry(&mut self, entry: Result<DirEntry, Error>) {
        match entry {
            Err(e) => println!("Failed to read entry: {}", e),
            Ok(dir_entry) => self.handle_dir_entry(dir_entry),
        }
    }

    pub fn read_files_in_current_dir(&mut self) -> Vec<String> {
        let paths = fs::read_dir("./").unwrap();

        for entry in paths {
            self.handle_entry(entry);
        }

        self.files.clone()
    }
}
