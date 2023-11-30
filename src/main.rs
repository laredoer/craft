use craft::explainer::extension::parse_extension;
use craft::libs::validator::Validate;
use gosyn::ast::Declaration;
use gosyn::parse_file;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use craft::libs::i18n::I18nExtend;
use craft::plugin::Plugin;

struct Extends {
    plugins: HashMap<&'static str, Box<dyn Plugin>>,
}

impl Extends {
    fn new() -> Extends {
        let mut e = Extends {
            plugins: HashMap::new(),
        };

        let i18n = I18nExtend::new();
        e.plugins.insert(i18n.name(), Box::new(i18n));

        let validator = Validate::new();
        e.plugins.insert(validator.name(), Box::new(validator));

        e
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .or(Some(String::from("./")))
        .unwrap();

    let directory = Path::new(&path);
    let mut go_file_paths: Vec<PathBuf> = Vec::new();
    read_gofiles_in_directory(&directory, &mut go_file_paths);
    let extends = Extends::new();

    go_file_paths.iter().for_each(|path| {
        let content = parse_file(path).unwrap();

        let mut file_path_to_content: HashMap<PathBuf, String> = HashMap::new();

        content.decl.into_iter().for_each(|decl| match decl {
            Declaration::Type(type_decl) => {
                type_decl.specs.into_iter().for_each(|spec| {
                    let comments = spec
                        .clone()
                        .docs
                        .into_iter()
                        .map(|s| s.text.clone())
                        .collect::<Vec<String>>()
                        .join(" ");

                    let exs = parse_extension(comments.into());
                    exs.into_iter().for_each(|ex| {
                        let plugin = extends.plugins.get(ex.name.as_str()).unwrap();

                        let output_file = get_output_file_path(path, plugin.name());
                        if !file_path_to_content.contains_key(&output_file) {
                            file_path_to_content.insert(
                                get_output_file_path(path, plugin.name()),
                                plugin.header(content.pkg_name.name.as_str()),
                            );
                        }

                        file_path_to_content
                            .get_mut(&output_file)
                            .unwrap()
                            .push_str(
                                &plugin.build(spec.clone(), ex.args.unwrap_or_else(|| vec![])),
                            );
                    });
                });
            }
            _ => {}
        });

        file_path_to_content
            .iter()
            .for_each(|(file_path, content)| {
                fs::write(file_path, content).unwrap();
            });
    });
}

fn read_gofiles_in_directory(folder_path: &Path, file_paths: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    read_gofiles_in_directory(&path, file_paths);
                } else if let Some(extension) = path.extension() {
                    if extension == "go" {
                        file_paths.push(path.clone());
                    }
                }
            }
        }
    }
}

// 活动要生成代码的文件路径
// 例如原始文件路径为：/Users/xxx/xxx/xxx.go 生成的文件路径为：/Users/xxx/xxx/xxx_i18n.go
fn get_output_file_path(file_path: &Path, plugin_name: &str) -> PathBuf {
    let mut output_file_path = file_path.to_path_buf();
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_name = file_name.replace(".go", "");
    output_file_path.set_file_name(format!("{}_{}.go", file_name, plugin_name));
    output_file_path
}
