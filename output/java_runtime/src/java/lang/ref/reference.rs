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

#[java_rta_macros::java_class(
    binary_name       = "java/lang/ref/Reference",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Reference.java",
    inner_classes     = "java/lang/ref/Reference$ReferenceHandler:java/lang/ref/Reference:ReferenceHandler:10;java/lang/ref/Reference$1:::0",
    all_supertypes    = "java/lang/Object;java/lang/ref/Reference",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Reference<T: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "referent", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "", is_static = false, generic_signature = "TT;"))]
    pub referent: JField<T>,
    #[cfg_attr(any(), java_field(name = "queue", descriptor = "Ljava/lang/ref/ReferenceQueue;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "Ljava/lang/ref/ReferenceQueue<-TT;>;"))]
    pub queue: JField<ReferenceQueue<T>>,
    #[cfg_attr(any(), java_field(name = "next", descriptor = "Ljava/lang/ref/Reference;", access = "package", modifiers = "volatile", is_static = false))]
    pub next: JField<Reference<Object>>,
    #[cfg_attr(any(), java_field(name = "discovered", descriptor = "Ljava/lang/ref/Reference;", access = "private", modifiers = "transient", is_static = false, generic_signature = "Ljava/lang/ref/Reference<*>;"))]
    pub discovered: JField<Reference<Object>>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Default + 'static> Reference<T> {
    #[cfg_attr(any(), java_field(name = "processPendingLock", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
    // static field: processPendingLock:Ljava/lang/Object;
    pub fn processPendingLock() -> Object {
        panic!("stub: java/lang/ref/Reference.processPendingLock:Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_field(name = "processPendingActive", descriptor = "Z", access = "private", modifiers = "static", is_static = true))]
    // static field: processPendingActive:Z
    pub fn processPendingActive() -> bool {
        false
    }

    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_native(name = "getAndClearReferencePendingList", descriptor = "()Ljava/lang/ref/Reference;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/ref/Reference<*>;"))]
    pub fn getAndClearReferencePendingList() -> Result<Reference<Object>> {
        panic!("native: java/lang/ref/Reference.getAndClearReferencePendingList:()Ljava/lang/ref/Reference;")
    }

    #[cfg_attr(any(), java_native(name = "hasReferencePendingList", descriptor = "()Z", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn hasReferencePendingList() -> Result<bool> {
        panic!("native: java/lang/ref/Reference.hasReferencePendingList:()Z")
    }

    #[cfg_attr(any(), java_native(name = "waitForReferencePendingList", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn waitForReferencePendingList() -> Result<()> {
        panic!("native: java/lang/ref/Reference.waitForReferencePendingList:()V")
    }

    #[cfg_attr(any(), java_method(name = "enqueueFromPending", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn enqueueFromPending(&self) -> Result<()> {
        panic!("stub: java/lang/ref/Reference.enqueueFromPending:()V")
    }

    #[cfg_attr(any(), java_method(name = "processPendingReferences", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn processPendingReferences() -> Result<()> {
        panic!("stub: java/lang/ref/Reference.processPendingReferences:()V")
    }

    #[cfg_attr(any(), java_method(name = "waitForReferenceProcessing", descriptor = "()Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn waitForReferenceProcessing() -> Result<bool> {
        panic!("stub: java/lang/ref/Reference.waitForReferenceProcessing:()Z")
    }

    #[cfg_attr(any(), java_method(name = "startReferenceHandlerThread", descriptor = "(Ljava/lang/ThreadGroup;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn startReferenceHandlerThread(tg: Object) -> Result<()> {
        panic!("stub: java/lang/ref/Reference.startReferenceHandlerThread:(Ljava/lang/ThreadGroup;)V")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;"))]
    pub fn get(&self) -> Result<T> {
        let this = self;
        Ok(this.referent.get())
    }

    #[cfg_attr(any(), java_method(name = "refersTo", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)Z"))]
    pub fn refersTo(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/ref/Reference.refersTo:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "refersToImpl", descriptor = "(Ljava/lang/Object;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)Z"))]
    pub fn refersToImpl(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/ref/Reference.refersToImpl:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_native(name = "refersTo0", descriptor = "(Ljava/lang/Object;)Z", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn refersTo0(&self, arg0: Object) -> Result<bool> {
        panic!("native: java/lang/ref/Reference.refersTo0:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/lang/ref/Reference.clear:()V")
    }

    #[cfg_attr(any(), java_native(name = "clear0", descriptor = "()V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn clear0(&self) -> Result<()> {
        panic!("native: java/lang/ref/Reference.clear0:()V")
    }

    #[cfg_attr(any(), java_method(name = "getFromInactiveFinalReference", descriptor = "()Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;"))]
    pub fn getFromInactiveFinalReference(&self) -> Result<Object> {
        panic!("stub: java/lang/ref/Reference.getFromInactiveFinalReference:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "clearInactiveFinalReference", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clearInactiveFinalReference(&self) -> Result<()> {
        panic!("stub: java/lang/ref/Reference.clearInactiveFinalReference:()V")
    }

    #[cfg_attr(any(), java_method(name = "isEnqueued", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn isEnqueued(&self) -> Result<bool> {
        panic!("stub: java/lang/ref/Reference.isEnqueued:()Z")
    }

    #[cfg_attr(any(), java_method(name = "enqueue", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn enqueue(&self) -> Result<bool> {
        panic!("stub: java/lang/ref/Reference.enqueue:()Z")
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/CloneNotSupportedException"))]
    pub fn clone(&self) -> Result<Object> {
        panic!("stub: java/lang/ref/Reference.clone:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V"))]
    // java: <init>(Ljava/lang/Object;)V
    pub fn new_obj(mut referent: T) -> Result<Self> {
        let mut this = Self { referent: JField::new_uninit(), queue: JField::new(Default::default()), next: JField::new(Default::default()), discovered: JField::new(Default::default()), _phantom: std::marker::PhantomData, ..Default::default() };
        this = Reference::new_obj_refere(Clone::clone(&referent), Default::default())?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;Ljava/lang/ref/ReferenceQueue<-TT;>;)V"))]
    // java: <init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V
    pub fn new_obj_refere(mut referent: T, mut queue: ReferenceQueue<Object>) -> Result<Self> {
        let mut this = Self { referent: JField::new_uninit(), queue: JField::new(Default::default()), next: JField::new(Default::default()), discovered: JField::new(Default::default()), _phantom: std::marker::PhantomData, ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.referent.set(Clone::clone(&referent));
        this.queue.set(Clone::clone(&(if _is_jnull(&queue) { ReferenceQueue::<Object>::NULL() } else { queue })));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "reachabilityFence", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reachabilityFence(ref_: Object) -> Result<()> {
        panic!("stub: java/lang/ref/Reference.reachabilityFence:(Ljava/lang/Object;)V")
    }
}
