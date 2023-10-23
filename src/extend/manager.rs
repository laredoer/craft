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
    fn build(ts: TypeSpec, args: Vec<Field>) -> String
    where
        Self: Sized,
    {
        "".into()
    }

    // 当拓展被加载时触发该事件
    fn on_extend_load(&self) {}
}

pub struct AppExtendManager {
    path: String,
    extends: HashMap<String, Arc<Box<dyn Extend>>>,
    loaded_libraries: Vec<Library>,
}
