#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

impl<T: Clone + Default + 'static> From<SoftReference<T>> for Reference<T> {
    fn from(v: SoftReference<T>) -> Reference<T> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/ref/SoftReference"]
    #[super_class       = "java/lang/ref/Reference"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/ref/Reference<TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "SoftReference.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Reference<T>"]
    #[superclass_fields(referent: T, queue: ReferenceQueue<T>, next: Reference<Object>, discovered: Reference<Object>)]
    #[all_supertypes    = "java/lang/Object;java/lang/ref/Reference;java/lang/ref/SoftReference"]

    pub struct SoftReference<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "timestamp", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub timestamp: i64,
    }

    impl<T> SoftReference<T> {
        #[cfg_attr(any(), java_field(name = "clock", descriptor = "J", access = "private", modifiers = "static", is_static = true))]
        // static field: clock:J
        pub fn clock() -> i64 {
            panic!("stub: java/lang/ref/SoftReference.clock:J")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V")]
        // java: <init>(Ljava/lang/Object;)V
        pub fn new_obj(mut referent: T) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Reference::new_obj(Clone::clone(&referent))?);
            this.__set_timestamp(SoftReference::<Object>::clock());
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;Ljava/lang/ref/ReferenceQueue<-TT;>;)V")]
        pub fn new_obj_refere(referent: T, q: ReferenceQueue<T>) -> Result<Self> {
            panic!("stub: java/lang/ref/SoftReference.<init>:(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V")
        }

        #[java_method(name = "get", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;")]
        pub fn get(&self) -> Result<T> {
            let this = self;
            let _t0 = this.__super().get()?;
            let mut o: T = _t0;
            if (((this.__get_timestamp()>(SoftReference::<Object>::clock())) as i32-((this.__get_timestamp())<(SoftReference::<Object>::clock())) as i32)!=0) {
                this.__set_timestamp(SoftReference::<Object>::clock());
            }
            Ok(o)
        }
    }
}
