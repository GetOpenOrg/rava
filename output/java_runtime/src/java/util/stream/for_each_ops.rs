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
    #[binary_name       = "java/util/stream/ForEachOps"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ForEachOps.java"]
    #[inner_classes     = "java/util/stream/ForEachOps$ForEachOp:java/util/stream/ForEachOps:ForEachOp:1032;java/util/stream/ForEachOps$ForEachOp$OfRef:java/util/stream/ForEachOps$ForEachOp:OfRef:24;java/util/stream/ForEachOps$ForEachOp$OfInt:java/util/stream/ForEachOps$ForEachOp:OfInt:24;java/util/stream/ForEachOps$ForEachOp$OfLong:java/util/stream/ForEachOps$ForEachOp:OfLong:24;java/util/stream/ForEachOps$ForEachOp$OfDouble:java/util/stream/ForEachOps$ForEachOp:OfDouble:24;java/util/stream/ForEachOps$ForEachOrderedTask:java/util/stream/ForEachOps:ForEachOrderedTask:24;java/util/stream/ForEachOps$ForEachTask:java/util/stream/ForEachOps:ForEachTask:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/ForEachOps"]

    pub struct ForEachOps;

    impl ForEachOps {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/ForEachOps.<init>:()V")
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/util/function/Consumer;Z)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/Consumer<-TT;>;Z)Ljava/util/stream/TerminalOp<TT;Ljava/lang/Void;>;")]
        pub fn makeRef(mut action: Object, mut ordered: bool) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&action))?;
            Ok(Object::from_any(ForEachOps_ForEachOp_OfRef::<Object>::new(Clone::clone(&action), ordered)?.clone()))
        }

        #[java_method(name = "makeInt", descriptor = "(Ljava/util/function/IntConsumer;Z)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/IntConsumer;Z)Ljava/util/stream/TerminalOp<Ljava/lang/Integer;Ljava/lang/Void;>;")]
        pub fn makeInt(action: Object, ordered: bool) -> Result<Object> {
            panic!("stub: java/util/stream/ForEachOps.makeInt:(Ljava/util/function/IntConsumer;Z)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeLong", descriptor = "(Ljava/util/function/LongConsumer;Z)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/LongConsumer;Z)Ljava/util/stream/TerminalOp<Ljava/lang/Long;Ljava/lang/Void;>;")]
        pub fn makeLong(action: Object, ordered: bool) -> Result<Object> {
            panic!("stub: java/util/stream/ForEachOps.makeLong:(Ljava/util/function/LongConsumer;Z)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeDouble", descriptor = "(Ljava/util/function/DoubleConsumer;Z)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/DoubleConsumer;Z)Ljava/util/stream/TerminalOp<Ljava/lang/Double;Ljava/lang/Void;>;")]
        pub fn makeDouble(action: Object, ordered: bool) -> Result<Object> {
            panic!("stub: java/util/stream/ForEachOps.makeDouble:(Ljava/util/function/DoubleConsumer;Z)Ljava/util/stream/TerminalOp;")
        }
    }
}
