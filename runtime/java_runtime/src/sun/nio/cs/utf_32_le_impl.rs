//! `sun/nio/cs/UTF_32LE` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//! JDK25 `HexFormat` / `StandardCharsets` 链以 `new` 构造（UTF_32BE 对称件）。

use crate::prelude::*;
use super::utf_32_le::UTF_32LE;

impl UTF_32LE {
    /// `<init>()V`：`super("UTF-32LE", StandardCharsets.aliases_UTF_32LE())`。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("UTF-32LE"));
        this.__set_aliases(JArray::from(
            ["UTF_32LE", "X-UTF-32LE"].iter()
                .map(|s| String::from(*s)).collect::<Vec<String>>()));
        Ok(this)
    }
}
