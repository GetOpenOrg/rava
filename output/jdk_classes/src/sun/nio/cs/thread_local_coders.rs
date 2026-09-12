#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "sun/nio/cs/ThreadLocalCoders",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "ThreadLocalCoders.java",
))]
pub struct ThreadLocalCoders;

impl ThreadLocalCoders {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "decoderFor", descriptor = "(Ljava/lang/Object;)Ljava/nio/charset/CharsetDecoder;", access = "public static"))]
    pub fn decoderFor(name: Object) -> Result<Object> {
        let _t0 = ThreadLocalCoders::decoderCache().forName(name)?;
        let mut cd: Object = _t0;
        let _t1 = cd.reset()?;
        Ok(cd)
    }

    #[cfg_attr(any(), java_method(name = "encoderFor", descriptor = "(Ljava/lang/Object;)Ljava/nio/charset/CharsetEncoder;", access = "public static"))]
    pub fn encoderFor(name: Object) -> Result<Object> {
        let _t0 = ThreadLocalCoders::encoderCache().forName(name)?;
        let mut ce: Object = _t0;
        let _t1 = ce.reset()?;
        Ok(ce)
    }
}
