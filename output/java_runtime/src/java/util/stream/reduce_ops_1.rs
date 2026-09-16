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

impl From<ReduceOps_1> for ReduceOps_ReduceOp<Object, Object, Object> {
    fn from(v: ReduceOps_1) -> ReduceOps_ReduceOp<Object, Object, Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/ReduceOps$1"]
    #[super_class       = "java/util/stream/ReduceOps$ReduceOp"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/util/stream/ReduceOps$ReduceOp<TT;TU;Ljava/util/stream/ReduceOps$1ReducingSink;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReduceOps.java"]
    #[inner_classes     = "java/util/stream/ReduceOps$1:::0;java/util/stream/ReduceOps$ReduceOp:java/util/stream/ReduceOps:ReduceOp:1034;java/util/stream/ReduceOps$1ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$AccumulatingSink:java/util/stream/ReduceOps:AccumulatingSink:1546"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ReduceOps_ReduceOp<Object, Object, Object>"]
    #[superclass_fields(inputShape: StreamShape)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/ReduceOps$1;java/util/stream/ReduceOps$ReduceOp;java/util/stream/TerminalOp"]

    pub struct ReduceOps_1 {
        #[cfg_attr(any(), java_field(name = "val$combiner", descriptor = "Ljava/util/function/BinaryOperator;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_combiner: Object,
        #[cfg_attr(any(), java_field(name = "val$reducer", descriptor = "Ljava/util/function/BiFunction;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_reducer: Object,
        #[cfg_attr(any(), java_field(name = "val$seed", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_seed: Object,
    }

    impl ReduceOps_1 {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/StreamShape;Ljava/util/function/BinaryOperator;Ljava/util/function/BiFunction;Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut shape: StreamShape, mut arg_1: Object, mut arg_2: Object, mut arg_3: Object) -> Result<Self> {
            let mut this = Self::default();
            this.__set_val_combiner(Clone::clone(&arg_1));
            this.__set_val_reducer(Clone::clone(&arg_2));
            this.__set_val_seed(Clone::clone(&arg_3));
            this = Self::__new_with_super(ReduceOps_ReduceOp::new(Clone::clone(&shape))?);
            Ok(this)
        }

        #[java_method(name = "makeSink", descriptor = "()Ljava/util/stream/ReduceOps$1ReducingSink;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeSink(&self) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps$1.makeSink:()Ljava/util/stream/ReduceOps$1ReducingSink;")
        }
    }
}
