#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/CodingErrorAction",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "CodingErrorAction.java",
))]
pub struct CodingErrorAction {
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private"))]
    pub name: Field<String>,
}

impl CodingErrorAction {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "private"))]
    pub fn new(name: String) -> Result<Self> {
        let this = Self { name: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.name.set(name);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }
}
