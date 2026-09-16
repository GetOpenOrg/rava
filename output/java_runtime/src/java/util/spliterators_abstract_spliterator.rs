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
    #[binary_name       = "java/util/Spliterators$AbstractSpliterator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Spliterator"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Spliterator<TT;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Spliterators.java"]
    #[inner_classes     = "java/util/Spliterators$AbstractSpliterator:java/util/Spliterators:AbstractSpliterator:1033;java/util/Spliterators$AbstractSpliterator$HoldingConsumer:java/util/Spliterators$AbstractSpliterator:HoldingConsumer:24;java/util/Spliterators$ArraySpliterator:java/util/Spliterators:ArraySpliterator:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterator;java/util/Spliterators$AbstractSpliterator"]

    pub struct Spliterators_AbstractSpliterator<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub characteristics: i32,
        #[cfg_attr(any(), java_field(name = "est", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub est: i64,
        #[cfg_attr(any(), java_field(name = "batch", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub batch: i32,
    }

    impl<T> Spliterators_AbstractSpliterator<T> {
        #[cfg_attr(any(), java_field(name = "BATCH_UNIT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1024"))]
        // static field: BATCH_UNIT:I
        pub fn BATCH_UNIT() -> i32 {
            1024
        }

        #[cfg_attr(any(), java_field(name = "MAX_BATCH", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "33554432"))]
        // static field: MAX_BATCH:I
        pub fn MAX_BATCH() -> i32 {
            33554432
        }

        #[java_method(name = "<init>", descriptor = "(JI)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut est: i64, mut additionalCharacteristics: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_est(est);
            this.__set_characteristics((if ((additionalCharacteristics&64i32)!=0) { (additionalCharacteristics|16384i32) } else { additionalCharacteristics }));
            Ok(this)
        }

        #[java_method(name = "trySplit", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TT;>;")]
        pub fn trySplit(&self) -> Result<Object> {
            panic!("stub: java/util/Spliterators$AbstractSpliterator.trySplit:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "estimateSize", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn estimateSize(&self) -> Result<i64> {
            panic!("stub: java/util/Spliterators$AbstractSpliterator.estimateSize:()J")
        }

        #[java_method(name = "characteristics", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn characteristics(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_characteristics())
        }

        #[java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V")]
        pub fn forEachRemaining(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/Spliterators$AbstractSpliterator.forEachRemaining:(Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "getExactSizeIfKnown", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExactSizeIfKnown(&self) -> Result<i64> {
            panic!("stub: java/util/Spliterators$AbstractSpliterator.getExactSizeIfKnown:()J")
        }

        #[java_method(name = "hasCharacteristics", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasCharacteristics(&self, characteristics: i32) -> Result<bool> {
            panic!("stub: java/util/Spliterators$AbstractSpliterator.hasCharacteristics:(I)Z")
        }

        #[java_method(name = "getComparator", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Comparator<-TT;>;")]
        pub fn getComparator(&self) -> Result<Object> {
            let this = self;
            return Err(JvmError::Custom("athrow".to_owned()));
        }
    }
}
