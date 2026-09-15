#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::misc::*;

#[java_rta_macros::java_class(
    binary_name       = "jdk/internal/misc/InternalLock",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "InternalLock.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct InternalLock {
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljava/util/concurrent/locks/ReentrantLock;", access = "private", modifiers = "final", is_static = false))]
    pub lock: JField<Object>,
}

impl InternalLock {
    #[cfg_attr(any(), java_field(name = "CAN_USE_INTERNAL_LOCK", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
    // static field: CAN_USE_INTERNAL_LOCK:Z
    pub fn CAN_USE_INTERNAL_LOCK() -> bool {
        panic!("stub: jdk/internal/misc/InternalLock.CAN_USE_INTERNAL_LOCK:Z")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: jdk/internal/misc/InternalLock.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "newLockOrNull", descriptor = "()Ljdk/internal/misc/InternalLock;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn newLockOrNull() -> Result<InternalLock> {
        panic!("stub: jdk/internal/misc/InternalLock.newLockOrNull:()Ljdk/internal/misc/InternalLock;")
    }

    #[cfg_attr(any(), java_method(name = "newLockOr", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn newLockOr(obj: Object) -> Result<Object> {
        panic!("stub: jdk/internal/misc/InternalLock.newLockOr:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "tryLock", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn tryLock(&self) -> Result<bool> {
        panic!("stub: jdk/internal/misc/InternalLock.tryLock:()Z")
    }

    #[cfg_attr(any(), java_method(name = "isHeldByCurrentThread", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isHeldByCurrentThread(&self) -> Result<bool> {
        panic!("stub: jdk/internal/misc/InternalLock.isHeldByCurrentThread:()Z")
    }
}
