//! `sun/nio/cs/UTF_32` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//! JDK25 `StandardCharsets.<clinit>` 以 `new` 构造（UTF_32 三件套之一）。

use crate::prelude::*;
use super::utf_32::UTF_32;

impl UTF_32 {
    /// `<init>()V`：`super("UTF-32", StandardCharsets.aliases_UTF_32())`。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("UTF-32"));
        this.__set_aliases(JArray::from(
            ["UTF_32", "UTF32"].iter()
                .map(|s| String::from(*s)).collect::<Vec<String>>()));
        Ok(this)
    }
}
