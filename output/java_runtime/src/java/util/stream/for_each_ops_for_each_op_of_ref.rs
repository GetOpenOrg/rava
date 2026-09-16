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

impl<T: Clone + Default + 'static> From<ForEachOps_ForEachOp_OfRef<T>> for ForEachOps_ForEachOp<T> {
    fn from(v: ForEachOps_ForEachOp_OfRef<T>) -> ForEachOps_ForEachOp<T> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/ForEachOps$ForEachOp$OfRef"]
    #[super_class       = "java/util/stream/ForEachOps$ForEachOp"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/util/stream/ForEachOps$ForEachOp<TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ForEachOps.java"]
    #[inner_classes     = "java/util/stream/ForEachOps$ForEachOp:java/util/stream/ForEachOps:ForEachOp:1032;java/util/stream/ForEachOps$ForEachOp$OfRef:java/util/stream/ForEachOps$ForEachOp:OfRef:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ForEachOps_ForEachOp<T>"]
    #[superclass_fields(ordered: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/ForEachOps$ForEachOp;java/util/stream/ForEachOps$ForEachOp$OfRef;java/util/stream/TerminalOp;java/util/stream/TerminalSink"]

    pub struct ForEachOps_ForEachOp_OfRef<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "consumer", descriptor = "Ljava/util/function/Consumer;", access = "package", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/Consumer<-TT;>;"))]
        pub consumer: Object,
    }

    impl<T> ForEachOps_ForEachOp_OfRef<T> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/function/Consumer;Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;Z)V")]
        pub fn new(mut consumer: Object, mut ordered: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ForEachOps_ForEachOp::new(ordered)?);
            this.__set_consumer(Clone::clone(&consumer));
            Ok(this)
        }

        #[java_method(name = "accept", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V")]
        pub fn accept(&self, t: T) -> Result<()> {
            panic!("stub: java/util/stream/ForEachOps$ForEachOp$OfRef.accept:(Ljava/lang/Object;)V")
        }
    }
}
