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
use crate::jdk::internal::r#ref::CleanerImpl_PhantomCleanableRef;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/ref/Cleaner"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Cleaner.java"]
    #[inner_classes     = "jdk/internal/ref/CleanerImpl$PhantomCleanableRef:jdk/internal/ref/CleanerImpl:PhantomCleanableRef:25;java/lang/ref/Cleaner$1:::0;java/lang/ref/Cleaner$Cleanable:java/lang/ref/Cleaner:Cleanable:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/ref/Cleaner"]

    pub struct Cleaner {
        #[cfg_attr(any(), java_field(name = "impl", descriptor = "Ljdk/internal/ref/CleanerImpl;", access = "package", modifiers = "final", is_static = false))]
        pub impl_: Object,
    }

    impl Cleaner {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/ref/Cleaner.<init>:()V")
        }

        #[java_method(name = "create", descriptor = "()Ljava/lang/ref/Cleaner;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn create() -> Result<Cleaner> {
            panic!("stub: java/lang/ref/Cleaner.create:()Ljava/lang/ref/Cleaner;")
        }

        #[java_method(name = "create", descriptor = "(Ljava/util/concurrent/ThreadFactory;)Ljava/lang/ref/Cleaner;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn create_thread(threadFactory: Object) -> Result<Cleaner> {
            panic!("stub: java/lang/ref/Cleaner.create:(Ljava/util/concurrent/ThreadFactory;)Ljava/lang/ref/Cleaner;")
        }

        #[java_method(name = "register", descriptor = "(Ljava/lang/Object;Ljava/lang/Runnable;)Ljava/lang/ref/Cleaner$Cleanable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn register(&self, mut obj: Object, mut action: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj_str(Clone::clone(&obj), Clone::clone(&String::from("obj")))?;
            let _t1: Object = Objects::requireNonNull_obj_str(Clone::clone(&action), Clone::clone(&String::from("action")))?;
            Ok(Object::from_any(CleanerImpl_PhantomCleanableRef::new_obj_cleane_runnab(Clone::clone(&obj), Clone::clone(this), Clone::clone(&action))?.clone()))
        }
    }
}
