mod core;
use core::*;

fn main() {
    let files: Vec<String> = file_reader::FileReader::new().read_files_in_current_dir();

    let mut file_sorter = file_sorter::FileSorter::new();

    for file in files {
        let file_name = file.clone();
        file_sorter.add_file(file_name.clone());
        println!("Found: {}", file_name)
    }

    file_sorter.sort();
}
