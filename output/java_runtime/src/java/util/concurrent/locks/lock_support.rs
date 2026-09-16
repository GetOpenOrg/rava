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
use crate::jdk::internal::misc::Unsafe;
use crate::jdk::internal::misc::VirtualThreads;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/locks/LockSupport"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LockSupport.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/concurrent/locks/LockSupport"]

    pub struct LockSupport;

    impl LockSupport {
        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: java/util/concurrent/locks/LockSupport.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "PARKBLOCKER", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: PARKBLOCKER:J
        pub fn PARKBLOCKER() -> i64 {
            panic!("stub: java/util/concurrent/locks/LockSupport.PARKBLOCKER:J")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/concurrent/locks/LockSupport.<init>:()V")
        }

        #[java_method(name = "setBlocker", descriptor = "(Ljava/lang/Thread;Ljava/lang/Object;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setBlocker(mut t: Thread, mut arg: Object) -> Result<()> {
            LockSupport::U().putReferenceOpaque(Object::from_any(t.clone()), LockSupport::PARKBLOCKER(), Clone::clone(&arg))?;
            Ok(())
        }

        #[java_method(name = "setCurrentBlocker", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCurrentBlocker(blocker: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/locks/LockSupport.setCurrentBlocker:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "unpark", descriptor = "(Ljava/lang/Thread;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unpark(thread: Thread) -> Result<()> {
            panic!("stub: java/util/concurrent/locks/LockSupport.unpark:(Ljava/lang/Thread;)V")
        }

        #[java_method(name = "park", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: park(Ljava/lang/Object;)V
        pub fn park_obj(mut blocker: Object) -> Result<()> {
            let _t0: Thread = Thread::currentThread()?;
            let mut t: Thread = _t0;
            LockSupport::setBlocker(Clone::clone(&t), Clone::clone(&blocker))?;
            let _t1 = t.isVirtual()?;
            if _t1 {
                VirtualThreads::park()?;
            } else {
                LockSupport::U().park((0i32 != 0i32), 0i64)?;
            }
            LockSupport::setBlocker(Clone::clone(&t), Clone::clone(&Object::default()))?;
            Ok(())
        }

        #[java_method(name = "parkNanos", descriptor = "(Ljava/lang/Object;J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parkNanos_obj_l(blocker: Object, nanos: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/locks/LockSupport.parkNanos:(Ljava/lang/Object;J)V")
        }

        #[java_method(name = "parkUntil", descriptor = "(Ljava/lang/Object;J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parkUntil_obj_l(blocker: Object, deadline: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/locks/LockSupport.parkUntil:(Ljava/lang/Object;J)V")
        }

        #[java_method(name = "getBlocker", descriptor = "(Ljava/lang/Thread;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBlocker(t: Thread) -> Result<Object> {
            panic!("stub: java/util/concurrent/locks/LockSupport.getBlocker:(Ljava/lang/Thread;)Ljava/lang/Object;")
        }

        #[java_method(name = "park", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn park() -> Result<()> {
            panic!("stub: java/util/concurrent/locks/LockSupport.park:()V")
        }

        #[java_method(name = "parkNanos", descriptor = "(J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parkNanos_l(nanos: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/locks/LockSupport.parkNanos:(J)V")
        }

        #[java_method(name = "parkUntil", descriptor = "(J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parkUntil_l(deadline: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/locks/LockSupport.parkUntil:(J)V")
        }

        #[java_method(name = "getThreadId", descriptor = "(Ljava/lang/Thread;)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getThreadId(thread: Thread) -> Result<i64> {
            panic!("stub: java/util/concurrent/locks/LockSupport.getThreadId:(Ljava/lang/Thread;)J")
        }
    }
}
