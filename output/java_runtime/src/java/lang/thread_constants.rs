#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Thread$Constants",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Thread.java",
    inner_classes     = "java/lang/Thread$Constants:java/lang/Thread:Constants:10;java/lang/Thread$Constants$1:::0",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Thread_Constants;

impl Thread_Constants {
    #[cfg_attr(any(), java_field(name = "VTHREAD_GROUP", descriptor = "Ljava/lang/ThreadGroup;", access = "package", modifiers = "static final", is_static = true))]
    // static field: VTHREAD_GROUP:Ljava/lang/ThreadGroup;
    pub fn VTHREAD_GROUP() -> Object {
        panic!("stub: java/lang/Thread$Constants.VTHREAD_GROUP:Ljava/lang/ThreadGroup;")
    }

    #[cfg_attr(any(), java_field(name = "NO_PERMISSIONS_ACC", descriptor = "Ljava/security/AccessControlContext;", access = "package", modifiers = "static final", is_static = true))]
    // static field: NO_PERMISSIONS_ACC:Ljava/security/AccessControlContext;
    pub fn NO_PERMISSIONS_ACC() -> Object {
        panic!("stub: java/lang/Thread$Constants.NO_PERMISSIONS_ACC:Ljava/security/AccessControlContext;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/Thread$Constants.<init>:()V")
    }
}
