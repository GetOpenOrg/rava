#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/spi/CharsetProvider",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "CharsetProvider.java",
))]
pub struct CharsetProvider;

impl CharsetProvider {
    #[cfg_attr(any(), java_method(name = "checkPermission", descriptor = "()Ljava/lang/Void;", access = "private static"))]
    pub fn checkPermission() -> Result<Object> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("charsetProvider"))?)?;
        /* TODO: aconst_null  */
        Ok(sm)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Void;)V", access = "private"))]
    // java: <init>(Ljava/lang/Void;)V
    pub fn new__void(ignore: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        let _t0: Object = CharsetProvider::checkPermission()?;
        /* invokespecial Method java/nio/charset/spi/CharsetProvider.<init>:(Ljava/lang/Void;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "charsets", descriptor = "()Ljava/util/Iterator;", access = "public abstract"))]
    pub fn charsets(&self) -> Result<Object> {
        todo!("abstract java/nio/charset/spi/CharsetProvider.charsets")
    }

    #[cfg_attr(any(), java_native(name = "charsetForName", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "public abstract"))]
    pub fn charsetForName(&self, arg0: String) -> Result<Object> {
        todo!("abstract java/nio/charset/spi/CharsetProvider.charsetForName")
    }
}
