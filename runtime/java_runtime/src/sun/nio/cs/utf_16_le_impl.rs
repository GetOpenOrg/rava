//! `sun/nio/cs/UTF_16LE` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//! `StandardCharsets.<clinit>` 以 `new` 构造。

use crate::prelude::*;
use super::utf_16_le::UTF_16LE;

impl UTF_16LE {
    /// `<init>()V`：`super("UTF-16LE", StandardCharsets.aliases_UTF_16LE())`。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("UTF-16LE"));
        this.__set_aliases(JArray::from(
            ["UTF_16LE", "X-UTF-16LE", "UnicodeLittleUnmarked"].iter()
                .map(|s| String::from(*s)).collect::<Vec<String>>()));
        Ok(this)
    }
}
