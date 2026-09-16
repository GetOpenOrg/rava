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

impl<T: Clone + Default + 'static> From<FindOps_FindSink_OfRef<T>> for FindOps_FindSink<T, Object> {
    fn from(v: FindOps_FindSink_OfRef<T>) -> FindOps_FindSink<T, Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/FindOps$FindSink$OfRef"]
    #[super_class       = "java/util/stream/FindOps$FindSink"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/util/stream/FindOps$FindSink<TT;Ljava/util/Optional<TT;>;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FindOps.java"]
    #[inner_classes     = "java/util/stream/FindOps$FindSink:java/util/stream/FindOps:FindSink:1034;java/util/stream/FindOps$FindSink$OfRef:java/util/stream/FindOps$FindSink:OfRef:24;java/util/stream/FindOps$FindOp:java/util/stream/FindOps:FindOp:26;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "FindOps_FindSink<T, Object>"]
    #[superclass_fields(hasValue: bool, value: T)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/FindOps$FindSink;java/util/stream/FindOps$FindSink$OfRef;java/util/stream/TerminalSink"]

    pub struct FindOps_FindSink_OfRef<T: Clone + Default + 'static>;

    impl<T> FindOps_FindSink_OfRef<T> {
        #[cfg_attr(any(), java_field(name = "OP_FIND_FIRST", descriptor = "Ljava/util/stream/TerminalOp;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/stream/TerminalOp<**>;"))]
        // static field: OP_FIND_FIRST:Ljava/util/stream/TerminalOp;
        pub fn OP_FIND_FIRST() -> Object {
            panic!("stub: java/util/stream/FindOps$FindSink$OfRef.OP_FIND_FIRST:Ljava/util/stream/TerminalOp;")
        }

        #[cfg_attr(any(), java_field(name = "OP_FIND_ANY", descriptor = "Ljava/util/stream/TerminalOp;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/stream/TerminalOp<**>;"))]
        // static field: OP_FIND_ANY:Ljava/util/stream/TerminalOp;
        pub fn OP_FIND_ANY() -> Object {
            panic!("stub: java/util/stream/FindOps$FindSink$OfRef.OP_FIND_ANY:Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/FindOps$FindSink$OfRef.<init>:()V")
        }

        #[java_method(name = "get", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<TT;>;")]
        pub fn get(&self) -> Result<Optional<Object>> {
            panic!("stub: java/util/stream/FindOps$FindSink$OfRef.get:()Ljava/util/Optional;")
        }
    }
}
