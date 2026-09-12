#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/IllegalCharsetNameException",
    super_class = "java/lang/IllegalArgumentException",
    interfaces  = "",
    access      = "public",
    source      = "IllegalCharsetNameException.java",
))]
pub struct IllegalCharsetNameException {
    #[cfg_attr(any(), java_field(name = "charsetName", descriptor = "Ljava/lang/String;", access = "private"))]
    pub charsetName: Field<String>,
}

impl IllegalCharsetNameException {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn new(charsetName: String) -> Result<Self> {
        let this = Self { charsetName: Field::new(String::new()) };
        /* invokespecial Method java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V */
        this.charsetName.set(charsetName);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getCharsetName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getCharsetName(&self) -> Result<String> {
        let this = self;
        Ok(this.charsetName.get())
    }
}
