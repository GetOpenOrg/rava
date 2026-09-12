#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/MissingFormatArgumentException",
    super_class = "java/util/IllegalFormatException",
    interfaces  = "",
    access      = "public",
    source      = "MissingFormatArgumentException.java",
))]
pub struct MissingFormatArgumentException {
    #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/lang/String;", access = "private"))]
    pub s: Field<String>,
}

impl MissingFormatArgumentException {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn new(s: String) -> Result<Self> {
        let this = Self { s: Field::new(String::new()) };
        /* invokespecial Method java/util/IllegalFormatException.<init>:()V */
        return Err(JvmError::Custom(String::from("athrow")));
        this.s.set(s);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getFormatSpecifier", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getFormatSpecifier(&self) -> Result<String> {
        let this = self;
        Ok(this.s.get())
    }

    #[cfg_attr(any(), java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getMessage(&self) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Format specifier '"))?;
        String::new().append(&this.s.get())?;
        String::new().append(&String::from("'"))?;
        Ok(String::new())
    }
}
