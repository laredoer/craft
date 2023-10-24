use craft::explainer::extension::Field;
use craft::extend::manager::Extend;
use gosyn::ast::TypeSpec;

#[derive(Default, Debug)]
pub struct I18nExtend;

impl Extend for I18nExtend {
    fn build(&self, ts: TypeSpec, args: Vec<Field>) -> String {
        "i18n build".into()
    }
}

#[no_mangle]
pub extern "C" fn create_extend() -> *mut dyn Extend {
    Box::into_raw(Box::new(I18nExtend))
}
