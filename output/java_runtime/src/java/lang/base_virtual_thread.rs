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
    binary_name       = "java/lang/BaseVirtualThread",
    super_class       = "java/lang/Thread",
    interfaces        = "",
    access            = "package",
    modifiers         = "abstract",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "BaseVirtualThread.java",
    inner_classes     = "java/lang/ThreadBuilders$BoundVirtualThread:java/lang/ThreadBuilders:BoundVirtualThread:24",
)]
#[derive(Clone, Default, PartialEq)]
pub struct BaseVirtualThread {
    pub _super: Thread,
}

impl BaseVirtualThread {
    pub fn as_thread(&self) -> &Thread { &self._super }
    pub fn into_thread(self) -> Thread { self._super }
}

impl From<BaseVirtualThread> for Thread {
    fn from(v: BaseVirtualThread) -> Thread { v._super }
}

impl BaseVirtualThread {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(name: String, characteristics: i32, bound: bool) -> Result<Self> {
        panic!("stub: java/lang/BaseVirtualThread.<init>:(Ljava/lang/String;IZ)V")
    }

    #[cfg_attr(any(), java_method(name = "park", descriptor = "()V", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn park(&self) -> Result<()> {
        panic!("stub: java/lang/BaseVirtualThread.park:()V")
    }

    #[cfg_attr(any(), java_method(name = "parkNanos", descriptor = "(J)V", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn parkNanos(&self, arg0: i64) -> Result<()> {
        panic!("stub: java/lang/BaseVirtualThread.parkNanos:(J)V")
    }

    #[cfg_attr(any(), java_method(name = "unpark", descriptor = "()V", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn unpark(&self) -> Result<()> {
        panic!("stub: java/lang/BaseVirtualThread.unpark:()V")
    }
}
