//! `sun/nio/cs/US_ASCII` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。

use crate::prelude::*;
use super::us_ascii::US_ASCII;

/// `StandardCharsets.aliases_US_ASCII()` 的 JDK 21 数据（sun/nio/cs/
/// StandardCharsets.java，javac 前源码核对）。
fn _aliases() -> JArray<String> {
    JArray::from(
        ["iso-ir-6", "ANSI_X3.4-1986", "ISO_646.irv:1991", "ASCII", "ISO646-US",
         "us", "IBM367", "cp367", "csASCII", "646", "iso_646.irv:1983",
         "ANSI_X3.4-1968", "ascii7"]
            .iter().map(|s| String::from(*s)).collect::<Vec<String>>())
}

impl US_ASCII {
    /// `<init>()V`：`super("US-ASCII", StandardCharsets.aliases_US_ASCII())`——
    /// Charset 平铺字段（name/aliases）直接填（super 构造器语义）。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_name(String::from("US-ASCII"));
        this.__set_aliases(_aliases());
        Ok(this)
    }

    /// static final INSTANCE：进程内唯一实例（JDK `<clinit>` 的 new 缓存）。
    #[jvm_boundary]
    pub fn INSTANCE() -> Result<US_ASCII> {
        crate::__process_static! {
            static INSTANCE: US_ASCII = US_ASCII::new().unwrap();
        }
        Ok(INSTANCE.with(Clone::clone))
    }
}
