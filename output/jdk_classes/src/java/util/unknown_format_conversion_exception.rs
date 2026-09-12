#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/UnknownFormatConversionException",
    super_class = "java/util/IllegalFormatException",
    interfaces  = "",
    access      = "public",
    source      = "UnknownFormatConversionException.java",
))]
pub struct UnknownFormatConversionException {
    #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/lang/String;", access = "private"))]
    pub s: Field<String>,
}

impl UnknownFormatConversionException {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn new(s: String) -> Result<Self> {
        let this = Self { s: Field::new(String::new()) };
        /* invokespecial Method java/util/IllegalFormatException.<init>:()V */
        return Err(JvmError::Custom(String::from("athrow")));
        this.s.set(s);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getConversion", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getConversion(&self) -> Result<String> {
        let this = self;
        Ok(this.s.get())
    }

    #[cfg_attr(any(), java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getMessage(&self) -> Result<String> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = this.s.get();
        let _t1: String = String::format(String::from("Conversion = '%s'"), &_arr0)?;
        Ok(_t1)
    }
}
