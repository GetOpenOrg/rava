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
    #[binary_name       = "java/util/stream/FindOps$FindSink"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/stream/TerminalSink"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<T:Ljava/lang/Object;O:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/stream/TerminalSink<TT;TO;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FindOps.java"]
    #[inner_classes     = "java/util/stream/FindOps$FindSink:java/util/stream/FindOps:FindSink:1034;java/util/stream/FindOps$FindSink$OfDouble:java/util/stream/FindOps$FindSink:OfDouble:24;java/util/stream/FindOps$FindSink$OfLong:java/util/stream/FindOps$FindSink:OfLong:24;java/util/stream/FindOps$FindSink$OfInt:java/util/stream/FindOps$FindSink:OfInt:24;java/util/stream/FindOps$FindSink$OfRef:java/util/stream/FindOps$FindSink:OfRef:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/FindOps$FindSink;java/util/stream/TerminalSink"]

    pub struct FindOps_FindSink<T: Clone + Default + 'static, O: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "hasValue", descriptor = "Z", is_static = false))]
        pub hasValue: bool,
        #[cfg_attr(any(), java_field(name = "value", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TT;"))]
        pub value: T,
    }

    impl<T, O> FindOps_FindSink<T, O> {
        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/FindOps$FindSink.<init>:()V")
        }

        #[java_method(name = "accept", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V")]
        pub fn accept(&self, value: T) -> Result<()> {
            panic!("stub: java/util/stream/FindOps$FindSink.accept:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "cancellationRequested", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cancellationRequested(&self) -> Result<bool> {
            panic!("stub: java/util/stream/FindOps$FindSink.cancellationRequested:()Z")
        }
    }
}
