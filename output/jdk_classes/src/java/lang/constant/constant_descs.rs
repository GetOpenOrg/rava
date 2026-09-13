#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/constant/ConstantDescs",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "ConstantDescs.java",
))]
pub struct ConstantDescs;

impl ConstantDescs {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/lang/constant/ConstantDescs.<init>:()V")
    }

    // java: ofCallsiteBootstrap(Ljava/lang/constant/ClassDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ClassDesc;)Ljava/lang/constant/DirectMethodHandleDesc;
    pub fn ofCallsiteBootstrap(owner: Object, name: String, returnType: Object, paramTypes: Vec<Object>) -> Result<Object> {
        panic!("stub: java/lang/constant/ConstantDescs.ofCallsiteBootstrap:(Ljava/lang/constant/ClassDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ClassDesc;)Ljava/lang/constant/DirectMethodHandleDesc;")
    }

    // java: ofConstantBootstrap(Ljava/lang/constant/ClassDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ClassDesc;)Ljava/lang/constant/DirectMethodHandleDesc;
    pub fn ofConstantBootstrap(owner: Object, name: String, returnType: Object, paramTypes: Vec<Object>) -> Result<Object> {
        panic!("stub: java/lang/constant/ConstantDescs.ofConstantBootstrap:(Ljava/lang/constant/ClassDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ClassDesc;)Ljava/lang/constant/DirectMethodHandleDesc;")
    }
}
