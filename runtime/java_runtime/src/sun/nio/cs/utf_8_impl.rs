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
        crate::__process_static! {
            static INSTANCE: UTF_8 = UTF_8::new().unwrap();
        }
        Ok(INSTANCE.with(Clone::clone))
    }

    /// `newDecoder()`：JDK 返回 UTF_8$Decoder（CharsetDecoder 协议机器）。
    /// POSIX 档 A 的解码在 StreamDecoder 伴生内直连承载（见 stream_decoder_impl），
    /// 此处返回携带 charset 引用的合成 decoder（消费面只读 dec.charset() 构造
    /// 快照）；UTF_8$Decoder 的 CoderResult 机器留档 B。
    #[jvm_boundary]
    pub fn __impl_newDecoder(&self) -> Result<crate::java::nio::charset::CharsetDecoder> {
        let mut dec = crate::java::nio::charset::CharsetDecoder::default();
        dec._init_not_null();
        dec.__set_charset(Clone::clone(self).into());
        Ok(dec)
    }
}
