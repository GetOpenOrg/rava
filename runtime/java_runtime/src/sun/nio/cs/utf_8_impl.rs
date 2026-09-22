use crate::prelude::*;
use super::utf_8::UTF_8;

impl UTF_8 {
    /// `<init>()V`：`super("UTF-8", StandardCharsets.aliases_UTF_8())`——
    /// Charset 平铺字段（name/aliases）直接填（super 构造器语义）。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("UTF-8"));
        this.__set_aliases(JArray::from(
            ["UTF8", "unicode-1-1-utf-8"].iter()
                .map(|s| String::from(*s)).collect::<Vec<String>>()));
        Ok(this)
    }

    /// static final INSTANCE：进程内唯一的 UTF-8 charset 实例。
    #[jvm_boundary]
    pub fn INSTANCE() -> Result<UTF_8> {
        thread_local! {
            static INSTANCE: UTF_8 = UTF_8::new().unwrap();
        }
        Ok(INSTANCE.with(Clone::clone))
    }
}
