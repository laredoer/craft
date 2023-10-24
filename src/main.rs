use craft::explainer::extension::parse_extension;
use craft::extend::manager::Extend;
use gosyn::ast::Declaration;
use gosyn::ast::Expression::{Ident, TypeStruct};
use gosyn::parse_file;
use std::fs;
use std::ops::Index;
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
                Declaration::Type(type_decl) => {
                    // type StripePaymentIDError int32

                    // println!("{:?}", type_decl.specs);
                    let go_type_name;
                    let go_type_t;
                    let go_stmt = type_decl.specs.first().unwrap();
                    go_type_name = go_stmt.name.name.clone(); // StripePaymentIDError

                    match &go_stmt.typ {
                        Ident(ide) => {
                            go_type_t = ide.name.clone(); // int32
                        }
                        TypeStruct(ts) => {}
                        _ => {}
                    }

                    for spec in type_decl.specs {
                        // 获取注释
                        let comments = spec
                            .clone()
                            .docs
                            .into_iter()
                            .map(|s| s.text.clone())
                            .collect::<Vec<String>>()
                            .join(" ");

                        println!("{}", comments);
                        let exs = parse_extension(comments.into());

                        let i18n_extend = call_dynamic().unwrap();

                        let i18n_extend = unsafe { Box::from_raw(i18n_extend) };

                        for ex in exs {
                            if ex.name == "i18n" {
                                println!("{:?}", i18n_extend.build(spec.clone(), ex.args.unwrap()))
                            }
                        }
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

fn call_dynamic() -> Result<*mut dyn Extend, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("./bin/libi18n.dylib")?;
        let func: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn Extend> =
            lib.get(b"create_extend")?;

        let ret = func();
        Ok(ret.to_owned())
    }
}
