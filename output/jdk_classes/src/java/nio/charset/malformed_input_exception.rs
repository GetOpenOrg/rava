#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/MalformedInputException",
    super_class = "java/nio/charset/CharacterCodingException",
    interfaces  = "",
    access      = "public",
    source      = "MalformedInputException.java",
))]
pub struct MalformedInputException {
    #[cfg_attr(any(), java_field(name = "inputLength", descriptor = "I", access = "private"))]
    pub inputLength: Field<i32>,
}

impl MalformedInputException {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    pub fn new(inputLength: i32) -> Result<Self> {
        let this = Self { inputLength: Field::new(0) };
        /* invokespecial Method java/nio/charset/CharacterCodingException.<init>:()V */
        this.inputLength.set(inputLength);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getInputLength", descriptor = "()I", access = "public"))]
    pub fn getInputLength(&self) -> Result<i32> {
        let this = self;
        Ok(this.inputLength.get())
    }

    #[cfg_attr(any(), java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getMessage(&self) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Input length ="))?;
        String::new().append(&this.inputLength.get())?;
        Ok(String::new())
    }
}
