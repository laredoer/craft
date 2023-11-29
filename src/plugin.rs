use crate::explainer::extension::Field;
use gosyn::ast::TypeSpec;

pub trait Plugin {
    /// 获取扩展名称
    fn name(&self) -> &'static str;

    fn header(&self, package_name: &str) -> String;

    //
    fn build(&self, ts: TypeSpec, args: Vec<Field>) -> String;

    // 当拓展被加载时触发该事件
    fn on_extend_load(&self) {}
}
