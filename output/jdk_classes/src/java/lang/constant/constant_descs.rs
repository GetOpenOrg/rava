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
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: ofCallsiteBootstrap(Ljava/lang/constant/ClassDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ClassDesc;)Ljava/lang/constant/DirectMethodHandleDesc;
    pub fn ofCallsiteBootstrap(owner: Object, name: String, returnType: Object, paramTypes: &[Object]) -> Result<Object> {
        let _t0: Object = MethodTypeDesc::of(returnType, &paramTypes)?;
        let _t1 = _t0.insertParameterTypes(0i32, ConstantDescs::INDY_BOOTSTRAP_ARGS())?;
        let _t2: Object = MethodHandleDesc::ofMethod(DirectMethodHandleDesc_Kind::STATIC(), owner, name, _t1)?;
        Ok(_t2)
    }

    // java: ofConstantBootstrap(Ljava/lang/constant/ClassDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ClassDesc;)Ljava/lang/constant/DirectMethodHandleDesc;
    pub fn ofConstantBootstrap(owner: Object, name: String, returnType: Object, paramTypes: &[Object]) -> Result<Object> {
        let _t0: Object = MethodTypeDesc::of(returnType, &paramTypes)?;
        let _t1 = _t0.insertParameterTypes(0i32, ConstantDescs::CONDY_BOOTSTRAP_ARGS())?;
        let _t2: Object = MethodHandleDesc::ofMethod(DirectMethodHandleDesc_Kind::STATIC(), owner, name, _t1)?;
        Ok(_t2)
    }
}
