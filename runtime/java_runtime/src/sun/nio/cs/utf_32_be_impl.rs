//! `sun/nio/cs/UTF_32BE` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//! JDK25 `HexFormat` / `StandardCharsets` 链以 `new` 构造。

use crate::prelude::*;
use super::utf_32_be::UTF_32BE;

impl UTF_32BE {
    /// `<init>()V`：`super("UTF-32BE", StandardCharsets.aliases_UTF_32BE())`。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("UTF-32BE"));
        this.__set_aliases(JArray::from(
            ["UTF_32BE", "X-UTF-32BE"].iter()
                .map(|s| String::from(*s)).collect::<Vec<String>>()));
        Ok(this)
    }
}
