#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/constant/ConstantDesc",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "ConstantDesc.java",
))]
pub struct ConstantDesc;

impl ConstantDesc {
    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Object;
    pub fn resolveConstantDesc(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/ConstantDesc.resolveConstantDesc")
    }
}
