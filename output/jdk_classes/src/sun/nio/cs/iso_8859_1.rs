#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "sun/nio/cs/ISO_8859_1",
    super_class = "java/nio/charset/Charset",
    interfaces  = "sun/nio/cs/HistoricallyNamedCharset",
    access      = "public",
    source      = "ISO_8859_1.java",
))]
pub struct ISO_8859_1;

impl ISO_8859_1 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        let _t0: Vec<String> = StandardCharsets::aliases_ISO_8859_1()?;
        /* invokespecial Method java/nio/charset/Charset.<init>:(Ljava/lang/String;[Ljava/lang/String;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "historicalName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn historicalName(&self) -> Result<String> {
        let this = self;
        Ok(String::from("ISO8859_1"))
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/nio/charset/Charset;)Z", access = "public"))]
    pub fn contains(&self, cs: Object) -> Result<bool> {
        let this = self;
        Ok(true!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "newDecoder", descriptor = "()Ljava/nio/charset/CharsetDecoder;", access = "public"))]
    pub fn newDecoder(&self) -> Result<Object> {
        let this = self;
        Ok(ISO_8859_1_Decoder::new(this)?)
    }

    #[cfg_attr(any(), java_method(name = "newEncoder", descriptor = "()Ljava/nio/charset/CharsetEncoder;", access = "public"))]
    pub fn newEncoder(&self) -> Result<Object> {
        let this = self;
        Ok(ISO_8859_1_Encoder::new(this)?)
    }
}
