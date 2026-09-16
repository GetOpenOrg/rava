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

impl From<Stream_1> for Spliterators_AbstractSpliterator<Object> {
    fn from(v: Stream_1) -> Spliterators_AbstractSpliterator<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/Stream$1"]
    #[super_class       = "java/util/Spliterators$AbstractSpliterator"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/util/Spliterators$AbstractSpliterator<TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Stream.java"]
    #[inner_classes     = "java/util/stream/Stream$1:::0;java/util/Spliterators$AbstractSpliterator:java/util/Spliterators:AbstractSpliterator:1033"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Spliterators_AbstractSpliterator<Object>"]
    #[superclass_fields(characteristics: i32, est: i64, batch: i32)]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterator;java/util/Spliterators$AbstractSpliterator;java/util/stream/Stream$1"]

    pub struct Stream_1 {
        #[cfg_attr(any(), java_field(name = "prev", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TT;"))]
        pub prev: Object,
        #[cfg_attr(any(), java_field(name = "started", descriptor = "Z", is_static = false))]
        pub started: bool,
        #[cfg_attr(any(), java_field(name = "val$f", descriptor = "Ljava/util/function/UnaryOperator;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_f: Object,
        #[cfg_attr(any(), java_field(name = "val$seed", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_seed: Object,
    }

    impl Stream_1 {
        #[java_method(name = "<init>", descriptor = "(JILjava/util/function/UnaryOperator;Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut est: i64, mut additionalCharacteristics: i32, mut arg_2: Object, mut arg_3: Object) -> Result<Self> {
            let mut this = Self::default();
            this.__set_val_f(Clone::clone(&arg_2));
            this.__set_val_seed(Clone::clone(&arg_3));
            this = Self::__new_with_super(Spliterators_AbstractSpliterator::new(est, additionalCharacteristics)?);
            Ok(this)
        }

        #[java_method(name = "tryAdvance", descriptor = "(Ljava/util/function/Consumer;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)Z")]
        pub fn tryAdvance(&self, action: Object) -> Result<bool> {
            panic!("stub: java/util/stream/Stream$1.tryAdvance:(Ljava/util/function/Consumer;)Z")
        }
    }
}
