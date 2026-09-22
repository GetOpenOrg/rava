//! `sun/nio/cs/UTF_16BE` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//! `StandardCharsets.<clinit>` 以 `new` 构造。

use crate::prelude::*;
use super::utf_16_be::UTF_16BE;

impl UTF_16BE {
    /// `<init>()V`：`super("UTF-16BE", StandardCharsets.aliases_UTF_16BE())`。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("UTF-16BE"));
        this.__set_aliases(JArray::from(
            ["UTF_16BE", "ISO-10646-UCS-2", "X-UTF-16BE", "UnicodeBigUnmarked"].iter()
                .map(|s| String::from(*s)).collect::<Vec<String>>()));
        Ok(this)
    }
}
