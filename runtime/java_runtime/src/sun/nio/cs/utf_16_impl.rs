//! `sun/nio/cs/UTF_16` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//! `StandardCharsets.<clinit>` 以 `new` 构造。

use crate::prelude::*;
use super::utf_16::UTF_16;

impl UTF_16 {
    /// `<init>()V`：`super("UTF-16", StandardCharsets.aliases_UTF_16())`。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("UTF-16"));
        this.__set_aliases(JArray::from(
            ["UTF_16", "utf16", "unicode", "UnicodeBig"].iter()
                .map(|s| String::from(*s)).collect::<Vec<String>>()));
        Ok(this)
    }
}
