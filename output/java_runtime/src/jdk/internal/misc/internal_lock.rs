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
    #[binary_name       = "jdk/internal/misc/InternalLock"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "InternalLock.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/misc/InternalLock"]

    pub struct InternalLock {
        #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljava/util/concurrent/locks/ReentrantLock;", access = "private", modifiers = "final", is_static = false))]
        pub lock: Object,
    }

    impl InternalLock {
        #[cfg_attr(any(), java_field(name = "CAN_USE_INTERNAL_LOCK", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
        // static field: CAN_USE_INTERNAL_LOCK:Z
        pub fn CAN_USE_INTERNAL_LOCK() -> bool {
            true
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/misc/InternalLock.<init>:()V")
        }

        #[java_method(name = "newLockOrNull", descriptor = "()Ljdk/internal/misc/InternalLock;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newLockOrNull() -> Result<InternalLock> {
            panic!("stub: jdk/internal/misc/InternalLock.newLockOrNull:()Ljdk/internal/misc/InternalLock;")
        }

        #[java_method(name = "newLockOr", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newLockOr(obj: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/InternalLock.newLockOr:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "tryLock", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryLock(&self) -> Result<bool> {
            panic!("stub: jdk/internal/misc/InternalLock.tryLock:()Z")
        }

        #[java_method(name = "isHeldByCurrentThread", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isHeldByCurrentThread(&self) -> Result<bool> {
            panic!("stub: jdk/internal/misc/InternalLock.isHeldByCurrentThread:()Z")
        }
    }
}
