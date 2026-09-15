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
    binary_name       = "java/lang/ref/SoftReference",
    super_class       = "java/lang/ref/Reference",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/ref/Reference<TT;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "SoftReference.java",
    all_supertypes    = "java/lang/Object;java/lang/ref/Reference;java/lang/ref/SoftReference",
)]
#[derive(Clone, Default, PartialEq)]
pub struct SoftReference<T: Clone + Default + 'static> {
    pub _super: Reference<T>,
    #[cfg_attr(any(), java_field(name = "timestamp", descriptor = "J", access = "private", modifiers = "", is_static = false))]
    pub timestamp: JField<i64>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Default + 'static> SoftReference<T> {
    pub fn as_reference(&self) -> &Reference<T> { &self._super }
    pub fn into_reference(self) -> Reference<T> { self._super }
}

impl<T: Clone + Default + 'static> From<SoftReference<T>> for Reference<T> {
    fn from(v: SoftReference<T>) -> Reference<T> { v._super }
}

impl<T: Clone + Default + 'static> SoftReference<T> {
    #[cfg_attr(any(), java_field(name = "clock", descriptor = "J", access = "private", modifiers = "static", is_static = true))]
    // static field: clock:J
    pub fn clock() -> i64 {
        panic!("stub: java/lang/ref/SoftReference.clock:J")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V"))]
    // java: <init>(Ljava/lang/Object;)V
    pub fn new_obj(mut referent: T) -> Result<Self> {
        let mut this = Self { _super: Default::default(), timestamp: JField::new(0), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = Reference::new_obj(Clone::clone(&referent))?;
        this.timestamp.set(SoftReference::<Object>::clock());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;Ljava/lang/ref/ReferenceQueue<-TT;>;)V"))]
    pub fn new_obj_refere(referent: Object, q: ReferenceQueue<Object>) -> Result<Self> {
        panic!("stub: java/lang/ref/SoftReference.<init>:(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;"))]
    pub fn get(&self) -> Result<T> {
        let this = self;
        let _t0 = this._super.get()?;
        let mut o: T = _t0;
        if (((this.timestamp.get()>(SoftReference::<Object>::clock())) as i32-((this.timestamp.get())<(SoftReference::<Object>::clock())) as i32)!=0) {
            this.timestamp.set(SoftReference::<Object>::clock());
        }
        Ok(o)
    }
}
