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
    #[binary_name       = "java/util/stream/FindOps"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FindOps.java"]
    #[inner_classes     = "java/util/stream/FindOps$FindSink:java/util/stream/FindOps:FindSink:1034;java/util/stream/FindOps$FindSink$OfRef:java/util/stream/FindOps$FindSink:OfRef:24;java/util/stream/FindOps$FindSink$OfInt:java/util/stream/FindOps$FindSink:OfInt:24;java/util/stream/FindOps$FindSink$OfLong:java/util/stream/FindOps$FindSink:OfLong:24;java/util/stream/FindOps$FindSink$OfDouble:java/util/stream/FindOps$FindSink:OfDouble:24;java/util/stream/FindOps$FindTask:java/util/stream/FindOps:FindTask:26;java/util/stream/FindOps$FindOp:java/util/stream/FindOps:FindOp:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/FindOps"]

    pub struct FindOps;

    impl FindOps {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/FindOps.<init>:()V")
        }

        #[java_method(name = "makeRef", descriptor = "(Z)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Z)Ljava/util/stream/TerminalOp<TT;Ljava/util/Optional<TT;>;>;")]
        pub fn makeRef(mut mustFindFirst: bool) -> Result<Object> {
            Ok((if mustFindFirst { FindOps_FindSink_OfRef::<Object>::OP_FIND_FIRST() } else { FindOps_FindSink_OfRef::<Object>::OP_FIND_ANY() }))
        }

        #[java_method(name = "makeInt", descriptor = "(Z)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)Ljava/util/stream/TerminalOp<Ljava/lang/Integer;Ljava/util/OptionalInt;>;")]
        pub fn makeInt(mustFindFirst: bool) -> Result<Object> {
            panic!("stub: java/util/stream/FindOps.makeInt:(Z)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeLong", descriptor = "(Z)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)Ljava/util/stream/TerminalOp<Ljava/lang/Long;Ljava/util/OptionalLong;>;")]
        pub fn makeLong(mustFindFirst: bool) -> Result<Object> {
            panic!("stub: java/util/stream/FindOps.makeLong:(Z)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeDouble", descriptor = "(Z)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)Ljava/util/stream/TerminalOp<Ljava/lang/Double;Ljava/util/OptionalDouble;>;")]
        pub fn makeDouble(mustFindFirst: bool) -> Result<Object> {
            panic!("stub: java/util/stream/FindOps.makeDouble:(Z)Ljava/util/stream/TerminalOp;")
        }
    }
}
