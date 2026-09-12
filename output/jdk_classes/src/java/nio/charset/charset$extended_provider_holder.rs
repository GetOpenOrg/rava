#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/Charset$ExtendedProviderHolder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "Charset.java",
))]
pub struct Charset_ExtendedProviderHolder;

impl Charset_ExtendedProviderHolder {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "extendedProviders", descriptor = "()[Ljava/nio/charset/spi/CharsetProvider;", access = "private static"))]
    pub fn extendedProviders() -> Result<Vec<Object>> {
        let _t0: Object = AccessController::doPrivileged(Charset_ExtendedProviderHolder_1::new()?)?;
        Ok(_t0)
    }
}
