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

impl<V: Clone + Default + 'static> From<RecursiveTask<V>> for ForkJoinTask<V> {
    fn from(v: RecursiveTask<V>) -> ForkJoinTask<V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/RecursiveTask"]
    #[super_class       = "java/util/concurrent/ForkJoinTask"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<V:Ljava/lang/Object;>Ljava/util/concurrent/ForkJoinTask<TV;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "RecursiveTask.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ForkJoinTask<V>"]
    #[superclass_fields(status: i32, aux: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/concurrent/ForkJoinTask;java/util/concurrent/Future;java/util/concurrent/RecursiveTask"]

    pub struct RecursiveTask<V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "result", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TV;"))]
        pub result: V,
    }

    impl<V> RecursiveTask<V> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "5232453952276485270"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            5232453952276485270i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ForkJoinTask::new()?);
            Ok(this)
        }

        #[java_method(name = "compute", descriptor = "()Ljava/lang/Object;", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()TV;")]
        pub fn compute(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/RecursiveTask.compute:()Ljava/lang/Object;")
        }

        #[java_method(name = "getRawResult", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TV;")]
        pub fn getRawResult(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/RecursiveTask.getRawResult:()Ljava/lang/Object;")
        }

        #[java_method(name = "setRawResult", descriptor = "(Ljava/lang/Object;)V", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TV;)V")]
        pub fn setRawResult(&self, value: V) -> Result<()> {
            panic!("stub: java/util/concurrent/RecursiveTask.setRawResult:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "exec", descriptor = "()Z", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exec(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/RecursiveTask.exec:()Z")
        }
    }
}
