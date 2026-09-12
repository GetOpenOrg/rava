#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Appendable",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Appendable.java",
))]
pub struct Appendable;

impl Appendable {
    #[cfg_attr(any(), java_native(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;", access = "public abstract"))]
    pub fn append__seq(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/lang/Appendable.append")
    }

    #[cfg_attr(any(), java_native(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;", access = "public abstract"))]
    pub fn append__seq_i_i(&self, arg0: Object, arg1: i32, arg2: i32) -> Result<Object> {
        todo!("abstract java/lang/Appendable.append")
    }

    #[cfg_attr(any(), java_native(name = "append", descriptor = "(C)Ljava/lang/Appendable;", access = "public abstract"))]
    pub fn append__c(&self, arg0: u16) -> Result<Object> {
        todo!("abstract java/lang/Appendable.append")
    }
}
