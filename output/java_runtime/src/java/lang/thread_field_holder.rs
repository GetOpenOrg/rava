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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Thread$FieldHolder"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Thread.java"]
    #[inner_classes     = "java/lang/Thread$FieldHolder:java/lang/Thread:FieldHolder:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/Thread$FieldHolder"]

    pub struct Thread_FieldHolder {
        #[cfg_attr(any(), java_field(name = "group", descriptor = "Ljava/lang/ThreadGroup;", access = "package", modifiers = "final", is_static = false))]
        pub group: Object,
        #[cfg_attr(any(), java_field(name = "task", descriptor = "Ljava/lang/Runnable;", access = "package", modifiers = "final", is_static = false))]
        pub task: Object,
        #[cfg_attr(any(), java_field(name = "stackSize", descriptor = "J", access = "package", modifiers = "final", is_static = false))]
        pub stackSize: i64,
        #[cfg_attr(any(), java_field(name = "priority", descriptor = "I", access = "package", modifiers = "volatile", is_static = false))]
        pub priority: i32,
        #[cfg_attr(any(), java_field(name = "daemon", descriptor = "Z", access = "package", modifiers = "volatile", is_static = false))]
        pub daemon: bool,
        #[cfg_attr(any(), java_field(name = "threadStatus", descriptor = "I", access = "package", modifiers = "volatile", is_static = false))]
        pub threadStatus: i32,
    }

    impl Thread_FieldHolder {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;JIZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(group: Object, task: Object, stackSize: i64, arg3: i32, priority: bool) -> Result<Self> {
            panic!("stub: java/lang/Thread$FieldHolder.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;JIZ)V")
        }
    }
}
