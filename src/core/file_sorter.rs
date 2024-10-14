use std::fs;

pub struct FileSorter {
    files: Vec<String>,
}

impl FileSorter {
    pub fn new() -> Self {
        let files: Vec<String> = Vec::new();

        Self { files }
    }

    pub fn add_file(&mut self, file_path: String) {
        self.files.push(file_path);
    }

    fn create_directories(&self) {
        for file in self.files.clone() {
            // Normalize the file path for different OS
            let path = std::path::Path::new(&file);
            let path_letters: Vec<char> = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .chars()
                .collect();

            let first_letter = path_letters[0].to_uppercase().to_string();

            if !fs::exists(first_letter.clone()).unwrap() {
                fs::create_dir(first_letter.clone()).unwrap();
            }

            let dir_path = std::path::Path::new(first_letter.as_str());
            match self.move_to_dir(path, dir_path) {
                Ok(_) => {
                    println!("Moved {}", file)
                },
                Err(why) => println!("Could not move {}: {}", file, why),
            }
        }
    }

    fn move_to_dir(&self, from: &std::path::Path, to: &std::path::Path) -> Result<(), std::io::Error> {
        let file_in_dir = to.join(from);
        // This is used to move directories
        fs::rename(from, file_in_dir)
    }

    pub fn sort(&self) {
        self.create_directories();
    }
}
