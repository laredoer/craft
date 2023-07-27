use craft::explainer::extension;
use gosyn::ast::Declaration;
use gosyn::parse_file;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let path = std::env::args()
        .nth(1)
        .or(Some(String::from("./")))
        .unwrap();

    let directory = Path::new(&path);
    let mut file_paths = Vec::new();
    read_files_in_directory(&directory, &mut file_paths);
    for path in file_paths {
        let content = parse_file(path).unwrap();

        for decl in content.decl {
            match decl {
                Declaration::Function(func_decl) => {}
                Declaration::Type(type_decl) => {
                    for spec in type_decl.specs {
                        let c = spec
                            .docs
                            .into_iter()
                            .map(|s| s.text.clone())
                            .collect::<Vec<String>>()
                            .join(" ");

                        let exs = extension::parse_extension(c);
                        println!("{:#?}", exs);
                    }
                }
                _ => {}
            }
        }
    }
}

fn read_files_in_directory(folder_path: &Path, file_paths: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    read_files_in_directory(&path, file_paths);
                } else if let Some(extension) = path.extension() {
                    if extension == "go" {
                        file_paths.push(path.clone());
                    }
                }
            }
        }
    }
}
