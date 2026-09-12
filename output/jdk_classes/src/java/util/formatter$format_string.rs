#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Formatter$FormatString",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "abstract",
    source      = "Formatter.java",
))]
pub struct Formatter_FormatString;

impl Formatter_FormatString {
    #[cfg_attr(any(), java_native(name = "index", descriptor = "()I", access = "public abstract"))]
    pub fn index(&self) -> Result<i32> {
        todo!("abstract java/util/Formatter$FormatString.index")
    }

    #[cfg_attr(any(), java_native(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "public abstract"))]
    pub fn print(&self, arg0: Object, arg1: Object, arg2: Object) -> Result<()> {
        todo!("abstract java/util/Formatter$FormatString.print")
    }

    #[cfg_attr(any(), java_native(name = "toString", descriptor = "()Ljava/lang/String;", access = "public abstract"))]
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/Formatter$FormatString.toString")
    }
}
