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
    binary_name       = "java/lang/Thread$FieldHolder",
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
    inner_classes     = "java/lang/Thread$FieldHolder:java/lang/Thread:FieldHolder:10",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Thread_FieldHolder {
    #[cfg_attr(any(), java_field(name = "group", descriptor = "Ljava/lang/ThreadGroup;", access = "package", modifiers = "final", is_static = false))]
    pub group: JField<Object>,
    #[cfg_attr(any(), java_field(name = "task", descriptor = "Ljava/lang/Runnable;", access = "package", modifiers = "final", is_static = false))]
    pub task: JField<Object>,
    #[cfg_attr(any(), java_field(name = "stackSize", descriptor = "J", access = "package", modifiers = "final", is_static = false))]
    pub stackSize: JField<i64>,
    #[cfg_attr(any(), java_field(name = "priority", descriptor = "I", access = "package", modifiers = "volatile", is_static = false))]
    pub priority: JField<i32>,
    #[cfg_attr(any(), java_field(name = "daemon", descriptor = "Z", access = "package", modifiers = "volatile", is_static = false))]
    pub daemon: JField<bool>,
    #[cfg_attr(any(), java_field(name = "threadStatus", descriptor = "I", access = "package", modifiers = "volatile", is_static = false))]
    pub threadStatus: JField<i32>,
}

impl Thread_FieldHolder {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;JIZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(group: Object, task: Object, stackSize: i64, arg3: i32, priority: bool) -> Result<Self> {
        panic!("stub: java/lang/Thread$FieldHolder.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;JIZ)V")
    }
}
