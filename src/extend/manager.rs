use crate::explainer::extension::Field;
use gosyn::ast::TypeSpec;
use libloading::Library;
use std::{any::Any, collections::HashMap, sync::Arc};

pub trait Extend: Any + Send + Sync {
    /// 获取扩展名称
    fn name(&self) -> &'static str {
        return "i18n";
    }

    //
    fn build(&self, ts: TypeSpec, args: Vec<Field>) -> String;

    // 当拓展被加载时触发该事件
    fn on_extend_load(&self) {}
}

pub struct AppExtendManager {
    pub path: String,
    pub extends: HashMap<String, Arc<Box<dyn Extend>>>,
    pub loaded_libraries: Vec<Library>,
}
