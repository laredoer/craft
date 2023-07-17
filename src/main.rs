use std::fs;
use std::path::Path;

fn main() {
    let path = std::env::args()
        .nth(1)
        .or(Some(String::from("./")))
        .unwrap();

    let directory = Path::new(&path);
    read_files_in_directory(&directory);
}

fn read_files_in_directory(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    println!("Found file: {:?}", path);
                } else if path.is_dir() {
                    println!("Entering directory: {:?}", path);
                    read_files_in_directory(&path);
                }
            }
        }
    } else {
        println!("Failed to read directory: {:?}", dir);
    }
}
