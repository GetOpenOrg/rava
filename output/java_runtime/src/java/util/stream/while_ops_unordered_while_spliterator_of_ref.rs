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

impl<T: Clone + Default + 'static> From<WhileOps_UnorderedWhileSpliterator_OfRef<T>> for WhileOps_UnorderedWhileSpliterator<T, Object> {
    fn from(v: WhileOps_UnorderedWhileSpliterator_OfRef<T>) -> WhileOps_UnorderedWhileSpliterator<T, Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef"]
    #[super_class       = "java/util/stream/WhileOps$UnorderedWhileSpliterator"]
    #[interfaces        = "java/util/function/Consumer"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/util/stream/WhileOps$UnorderedWhileSpliterator<TT;Ljava/util/Spliterator<TT;>;>;Ljava/util/function/Consumer<TT;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "WhileOps.java"]
    #[inner_classes     = "java/util/stream/WhileOps$UnorderedWhileSpliterator:java/util/stream/WhileOps:UnorderedWhileSpliterator:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfRef:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Dropping:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:Dropping:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:Taking:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "WhileOps_UnorderedWhileSpliterator<T, Object>"]
    #[superclass_fields(s: Object, noSplitting: bool, cancel: AtomicBoolean, takeOrDrop: bool, count: i32)]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterator;java/util/function/Consumer;java/util/stream/WhileOps$UnorderedWhileSpliterator;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef"]

    pub struct WhileOps_UnorderedWhileSpliterator_OfRef<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "p", descriptor = "Ljava/util/function/Predicate;", access = "package", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/Predicate<-TT;>;"))]
        pub p: Object,
        #[cfg_attr(any(), java_field(name = "t", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TT;"))]
        pub t: T,
    }

    impl<T> WhileOps_UnorderedWhileSpliterator_OfRef<T> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;ZLjava/util/function/Predicate;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<TT;>;ZLjava/util/function/Predicate<-TT;>;)V")]
        // java: <init>(Ljava/util/Spliterator;ZLjava/util/function/Predicate;)V
        pub fn new_splite_z_predic(mut s: Object, mut noSplitting: bool, mut p: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(WhileOps_UnorderedWhileSpliterator::new_splite_z(Clone::clone(&s), noSplitting)?);
            this.__set_p(Clone::clone(&p));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<TT;>;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef<TT;>;)V")]
        pub fn new_splite_whileo(s: Object, parent: WhileOps_UnorderedWhileSpliterator_OfRef<T>) -> Result<Self> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef.<init>:(Ljava/util/Spliterator;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef;)V")
        }

        #[java_method(name = "accept", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V")]
        pub fn accept(&self, mut t: T) -> Result<()> {
            let this = self;
            this.__set_count(((this.__get_count()).wrapping_add(1i32)&63i32));
            this.__set_t(Clone::clone(&t));
            Ok(())
        }

        #[java_method(name = "andThen", descriptor = "(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)Ljava/util/function/Consumer<TT;>;")]
        pub fn andThen(&self, after: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef.andThen:(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;")
        }
    }
}
