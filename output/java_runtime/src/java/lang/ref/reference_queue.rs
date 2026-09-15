#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;
use crate::jdk::internal::misc::VM;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/ref/ReferenceQueue",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ReferenceQueue.java",
    inner_classes     = "java/lang/ref/ReferenceQueue$Null:java/lang/ref/ReferenceQueue:Null:10",
    all_supertypes    = "java/lang/Object;java/lang/ref/ReferenceQueue",
)]
#[derive(Clone, Default, PartialEq)]
pub struct ReferenceQueue<T: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "head", descriptor = "Ljava/lang/ref/Reference;", access = "private", modifiers = "volatile", is_static = false, generic_signature = "Ljava/lang/ref/Reference<+TT;>;"))]
    pub head: JField<Reference<T>>,
    #[cfg_attr(any(), java_field(name = "queueLength", descriptor = "J", access = "private", modifiers = "", is_static = false))]
    pub queueLength: JField<i64>,
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljava/util/concurrent/locks/ReentrantLock;", access = "private", modifiers = "final", is_static = false))]
    pub lock: JField<Object>,
    #[cfg_attr(any(), java_field(name = "notEmpty", descriptor = "Ljava/util/concurrent/locks/Condition;", access = "private", modifiers = "final", is_static = false))]
    pub notEmpty: JField<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Default + 'static> ReferenceQueue<T> {
    #[cfg_attr(any(), java_field(name = "NULL", descriptor = "Ljava/lang/ref/ReferenceQueue;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/ref/ReferenceQueue<Ljava/lang/Object;>;"))]
    // static field: NULL:Ljava/lang/ref/ReferenceQueue;
    pub fn NULL() -> ReferenceQueue<Object> {
        panic!("stub: java/lang/ref/ReferenceQueue.NULL:Ljava/lang/ref/ReferenceQueue;")
    }

    #[cfg_attr(any(), java_field(name = "ENQUEUED", descriptor = "Ljava/lang/ref/ReferenceQueue;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/ref/ReferenceQueue<Ljava/lang/Object;>;"))]
    // static field: ENQUEUED:Ljava/lang/ref/ReferenceQueue;
    pub fn ENQUEUED() -> ReferenceQueue<Object> {
        panic!("stub: java/lang/ref/ReferenceQueue.ENQUEUED:Ljava/lang/ref/ReferenceQueue;")
    }

    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_method(name = "signal", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn signal(&self) -> Result<()> {
        panic!("stub: java/lang/ref/ReferenceQueue.signal:()V")
    }

    #[cfg_attr(any(), java_method(name = "await", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn await_(&self) -> Result<()> {
        panic!("stub: java/lang/ref/ReferenceQueue.await:()V")
    }

    #[cfg_attr(any(), java_method(name = "await", descriptor = "(J)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn await_l(&self, timeoutMillis: i64) -> Result<()> {
        panic!("stub: java/lang/ref/ReferenceQueue.await:(J)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/ref/ReferenceQueue.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_i(dummy: i32) -> Result<Self> {
        panic!("stub: java/lang/ref/ReferenceQueue.<init>:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "enqueue0", descriptor = "(Ljava/lang/ref/Reference;)Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ref/Reference<+TT;>;)Z"))]
    pub fn enqueue0(&self, r: Reference<Object>) -> Result<bool> {
        panic!("stub: java/lang/ref/ReferenceQueue.enqueue0:(Ljava/lang/ref/Reference;)Z")
    }

    #[cfg_attr(any(), java_method(name = "headIsNull", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn headIsNull(&self) -> Result<bool> {
        panic!("stub: java/lang/ref/ReferenceQueue.headIsNull:()Z")
    }

    #[cfg_attr(any(), java_method(name = "poll0", descriptor = "()Ljava/lang/ref/Reference;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/ref/Reference<+TT;>;"))]
    pub fn poll0(&self) -> Result<Reference<Object>> {
        panic!("stub: java/lang/ref/ReferenceQueue.poll0:()Ljava/lang/ref/Reference;")
    }

    #[cfg_attr(any(), java_method(name = "remove0", descriptor = "(J)Ljava/lang/ref/Reference;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/InterruptedException", generic_signature = "(J)Ljava/lang/ref/Reference<+TT;>;"))]
    pub fn remove0_l(&self, timeout: i64) -> Result<Reference<Object>> {
        panic!("stub: java/lang/ref/ReferenceQueue.remove0:(J)Ljava/lang/ref/Reference;")
    }

    #[cfg_attr(any(), java_method(name = "remove0", descriptor = "()Ljava/lang/ref/Reference;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException", generic_signature = "()Ljava/lang/ref/Reference<+TT;>;"))]
    pub fn remove0(&self) -> Result<Reference<Object>> {
        panic!("stub: java/lang/ref/ReferenceQueue.remove0:()Ljava/lang/ref/Reference;")
    }

    #[cfg_attr(any(), java_method(name = "enqueue", descriptor = "(Ljava/lang/ref/Reference;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ref/Reference<+TT;>;)Z"))]
    pub fn enqueue(&self, r: Reference<Object>) -> Result<bool> {
        panic!("stub: java/lang/ref/ReferenceQueue.enqueue:(Ljava/lang/ref/Reference;)Z")
    }

    #[cfg_attr(any(), java_method(name = "poll", descriptor = "()Ljava/lang/ref/Reference;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/ref/Reference<+TT;>;"))]
    pub fn poll(&self) -> Result<Reference<Object>> {
        panic!("stub: java/lang/ref/ReferenceQueue.poll:()Ljava/lang/ref/Reference;")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(J)Ljava/lang/ref/Reference;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException", generic_signature = "(J)Ljava/lang/ref/Reference<+TT;>;"))]
    pub fn remove_l(&self, timeout: i64) -> Result<Reference<Object>> {
        panic!("stub: java/lang/ref/ReferenceQueue.remove:(J)Ljava/lang/ref/Reference;")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "()Ljava/lang/ref/Reference;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException", generic_signature = "()Ljava/lang/ref/Reference<+TT;>;"))]
    pub fn remove(&self) -> Result<Reference<Object>> {
        panic!("stub: java/lang/ref/ReferenceQueue.remove:()Ljava/lang/ref/Reference;")
    }

    #[cfg_attr(any(), java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-Ljava/lang/ref/Reference<+TT;>;>;)V"))]
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/lang/ref/ReferenceQueue.forEach:(Ljava/util/function/Consumer;)V")
    }
}
