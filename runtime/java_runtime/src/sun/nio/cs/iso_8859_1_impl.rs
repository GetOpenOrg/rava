//! `sun/nio/cs/ISO_8859_1` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。

use crate::prelude::*;
use super::iso_8859_1::ISO_8859_1;

/// `StandardCharsets.aliases_ISO_8859_1()` 的 JDK 21 数据。
fn _aliases() -> JArray<String> {
    JArray::from(
        ["iso-ir-100", "ISO_8859-1", "latin1", "l1", "IBM819", "cp819", "csISOLatin1",
         "819", "IBM-819", "ISO8859_1", "ISO_8859-1:1987", "ISO_8859_1", "8859_1",
         "ISO8859-1"]
            .iter().map(|s| String::from(*s)).collect::<Vec<String>>())
}

impl ISO_8859_1 {
    /// `<init>()V`：`super("ISO-8859-1", StandardCharsets.aliases_ISO_8859_1())`。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("ISO-8859-1"));
        this.__set_aliases(_aliases());
        Ok(this)
    }

    /// static final INSTANCE：进程内唯一实例（JDK `<clinit>` 的 new 缓存）。
    #[jvm_boundary]
    pub fn INSTANCE() -> Result<ISO_8859_1> {
        crate::__process_static! {
            static INSTANCE: ISO_8859_1 = ISO_8859_1::new().unwrap();
        }
        Ok(INSTANCE.with(Clone::clone))
    }
}
