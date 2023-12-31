use std::collections::HashMap;
use std::path::PathBuf;

use crate::{explainer::extension::Field, plugin::Plugin};

use gosyn::ast::Expression::Ident;
use gosyn::ast::TypeSpec;

#[derive(Default, Debug)]
pub struct I18nExtend;

impl I18nExtend {
    pub fn new() -> I18nExtend {
        I18nExtend::default()
    }
}

impl Plugin for I18nExtend {
    fn name(&self) -> &'static str {
        "i18n"
    }

    fn header(&self, package_name: &str, _: &PathBuf) -> String {
        let mut header = String::new();
        header.push_str("// Code generated by craft; DO NOT EDIT.\n\n");
        header.push_str(&format!("package {}\n\n", package_name));
        header.push_str("import (\n");
        header.push_str("\t\"fmt\"\n");
        header.push_str("\tutils \"github.com/wule61/derive/utils\"\n");
        header.push_str(")\n\n");
        header
    }

    fn build(&self, ts: TypeSpec, args: Vec<Field>, _: &PathBuf) -> String {
        let mut args_map = HashMap::new();
        for arg in &args {
            args_map.insert(arg.name.to_owned(), &arg.value);
        }

        let mut res = String::new();
        res.push_str(&format!(
            "// {}_ {} [{}]\n",
            ts.name.name,
            args_map.get("zh-HK").unwrap(),
            args_map.get("code").unwrap(),
        ));
        res.push_str(&format!(
            "var {}_ {} = {}\n",
            ts.name.name,
            ts.name.name,
            args_map.get("code").unwrap(),
        ));

        res.push_str(&format!(
            "var {}Locales = map[string]string{{\n",
            ts.name.name.to_lowercase()
        ));
        for arg in &args {
            res.push_str(&format!("  \"{}\": \"{}\",\n", arg.name, arg.value));
        }
        res.push_str("}\n\n");

        res.push_str(&format!(
            "func ({}) Trans(langOrArgs ...any) string {{\n",
            ts.name.name
        ));
        res.push_str("\tlang, args := utils.ParseLangArgs(langOrArgs...)\n");
        res.push_str(&format!(
            "\tif msg, ok := {}Locales[lang]; ok {{\n",
            ts.name.name.to_lowercase()
        ));
        res.push_str("\t\tif len(args) > 0 {\n");
        res.push_str("\t\t\treturn fmt.Sprintf(msg, args...)\n");
        res.push_str("\t\t}\n");
        res.push_str("\t\treturn msg\n");
        res.push_str("\t}\n");
        res.push_str(&format!(
            "\treturn fmt.Sprintf({}Locales[\"zh-HK\"], args...)\n",
            ts.name.name.to_lowercase()
        ));
        res.push_str("}\n\n");

        res.push_str(&format!(
            "func ({}) Code() {} {{\n  return 400\n}}\n\n",
            ts.name.name,
            match ts.typ {
                Ident(ident) => ident.name,
                _ => panic!("not support type"),
            }
        ));

        res
    }
}
