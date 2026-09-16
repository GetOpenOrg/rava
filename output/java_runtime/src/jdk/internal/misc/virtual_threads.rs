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
use crate::jdk::internal::misc::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/misc/VirtualThreads"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "VirtualThreads.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/misc/VirtualThreads"]

    pub struct VirtualThreads;

    impl VirtualThreads {
        #[cfg_attr(any(), java_field(name = "JLA", descriptor = "Ljdk/internal/access/JavaLangAccess;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JLA:Ljdk/internal/access/JavaLangAccess;
        pub fn JLA() -> Object {
            panic!("stub: jdk/internal/misc/VirtualThreads.JLA:Ljdk/internal/access/JavaLangAccess;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/misc/VirtualThreads.<init>:()V")
        }

        #[java_method(name = "park", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn park() -> Result<()> {
            panic!("stub: jdk/internal/misc/VirtualThreads.park:()V")
        }

        #[java_method(name = "park", descriptor = "(J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn park_l(nanos: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/VirtualThreads.park:(J)V")
        }

        #[java_method(name = "parkUntil", descriptor = "(J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parkUntil(deadline: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/VirtualThreads.parkUntil:(J)V")
        }

        #[java_method(name = "unpark", descriptor = "(Ljava/lang/Thread;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unpark(thread: Thread) -> Result<()> {
            panic!("stub: jdk/internal/misc/VirtualThreads.unpark:(Ljava/lang/Thread;)V")
        }
    }
}
