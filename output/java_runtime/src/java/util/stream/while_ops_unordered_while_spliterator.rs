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
    #[binary_name       = "java/util/stream/WhileOps$UnorderedWhileSpliterator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Spliterator"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<T:Ljava/lang/Object;T_SPLITR::Ljava/util/Spliterator<TT;>;>Ljava/lang/Object;Ljava/util/Spliterator<TT;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "WhileOps.java"]
    #[inner_classes     = "java/util/stream/WhileOps$UnorderedWhileSpliterator:java/util/stream/WhileOps:UnorderedWhileSpliterator:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfDouble:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfLong:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfInt:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfRef:1032"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterator;java/util/stream/WhileOps$UnorderedWhileSpliterator"]

    pub struct WhileOps_UnorderedWhileSpliterator<T: Clone + Default + 'static, T_SPLITR: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/util/Spliterator;", access = "package", modifiers = "final", is_static = false, generic_signature = "TT_SPLITR;"))]
        pub s: T_SPLITR,
        #[cfg_attr(any(), java_field(name = "noSplitting", descriptor = "Z", access = "package", modifiers = "final", is_static = false))]
        pub noSplitting: bool,
        #[cfg_attr(any(), java_field(name = "cancel", descriptor = "Ljava/util/concurrent/atomic/AtomicBoolean;", access = "package", modifiers = "final", is_static = false))]
        pub cancel: AtomicBoolean,
        #[cfg_attr(any(), java_field(name = "takeOrDrop", descriptor = "Z", is_static = false))]
        pub takeOrDrop: bool,
        #[cfg_attr(any(), java_field(name = "count", descriptor = "I", is_static = false))]
        pub count: i32,
    }

    impl<T, T_SPLITR> WhileOps_UnorderedWhileSpliterator<T, T_SPLITR> {
        #[cfg_attr(any(), java_field(name = "CANCEL_CHECK_COUNT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "63"))]
        // static field: CANCEL_CHECK_COUNT:I
        pub fn CANCEL_CHECK_COUNT() -> i32 {
            63
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT_SPLITR;Z)V")]
        // java: <init>(Ljava/util/Spliterator;Z)V
        pub fn new_splite_z(mut s: T_SPLITR, mut noSplitting: bool) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_takeOrDrop((1i32 != 0i32));
            this.__set_s(Object::from_any(s.clone()));
            this.__set_noSplitting(noSplitting);
            this.__set_cancel(Clone::clone(&AtomicBoolean::new()?));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT_SPLITR;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator<TT;TT_SPLITR;>;)V")]
        pub fn new_splite_whileo(s: T_SPLITR, parent: WhileOps_UnorderedWhileSpliterator<T, T_SPLITR>) -> Result<Self> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator.<init>:(Ljava/util/Spliterator;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator;)V")
        }

        #[java_method(name = "estimateSize", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn estimateSize(&self) -> Result<i64> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator.estimateSize:()J")
        }

        #[java_method(name = "characteristics", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn characteristics(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.__get_s().characteristics()?;
            Ok((_t0&-16449i32))
        }

        #[java_method(name = "getExactSizeIfKnown", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExactSizeIfKnown(&self) -> Result<i64> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator.getExactSizeIfKnown:()J")
        }

        #[java_method(name = "getComparator", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Comparator<-TT;>;")]
        pub fn getComparator(&self) -> Result<Object> {
            let this = self;
            let _t0 = this.__get_s().getComparator()?;
            Ok(_t0)
        }

        #[java_method(name = "trySplit", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT_SPLITR;")]
        pub fn trySplit(&self) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator.trySplit:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "checkCancelOnCount", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkCancelOnCount(&self) -> Result<bool> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator.checkCancelOnCount:()Z")
        }

        #[java_method(name = "makeSpliterator", descriptor = "(Ljava/util/Spliterator;)Ljava/util/Spliterator;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TT_SPLITR;)TT_SPLITR;")]
        pub fn makeSpliterator(&self, arg0: T_SPLITR) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator.makeSpliterator:(Ljava/util/Spliterator;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V")]
        pub fn forEachRemaining(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator.forEachRemaining:(Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "hasCharacteristics", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasCharacteristics(&self, characteristics: i32) -> Result<bool> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator.hasCharacteristics:(I)Z")
        }
    }
}
