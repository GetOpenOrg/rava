#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "sun/nio/cs/UTF_8",
    super_class = "sun/nio/cs/Unicode",
    interfaces  = "",
    access      = "public final",
    source      = "UTF_8.java",
))]
pub struct UTF_8;

impl UTF_8 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        let _t0: Vec<String> = StandardCharsets::aliases_UTF_8()?;
        /* invokespecial Method sun/nio/cs/Unicode.<init>:(Ljava/lang/String;[Ljava/lang/String;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "historicalName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn historicalName(&self) -> Result<String> {
        let this = self;
        Ok(String::from("UTF8"))
    }

    #[cfg_attr(any(), java_method(name = "newDecoder", descriptor = "()Ljava/nio/charset/CharsetDecoder;", access = "public"))]
    pub fn newDecoder(&self) -> Result<Object> {
        let this = self;
        Ok(UTF_8_Decoder::new(this)?)
    }

    #[cfg_attr(any(), java_method(name = "newEncoder", descriptor = "()Ljava/nio/charset/CharsetEncoder;", access = "public"))]
    pub fn newEncoder(&self) -> Result<Object> {
        let this = self;
        Ok(UTF_8_Encoder::new(this)?)
    }

    #[cfg_attr(any(), java_method(name = "updatePositions", descriptor = "(Ljava/nio/Buffer;ILjava/nio/Buffer;I)V", access = "static final"))]
    pub fn updatePositions(src: Object, sp: i32, dst: Object, dp: i32) -> Result<()> {
        let _t0 = src.arrayOffset()?;
        let _t1 = src.position((sp).wrapping_sub(_t0))?;
        let _t2 = dst.arrayOffset()?;
        let _t3 = dst.position((dp).wrapping_sub(_t2))?;
        Ok(())
    }
}
