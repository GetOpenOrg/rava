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

impl<T: Clone + Default + 'static> From<WhileOps_UnorderedWhileSpliterator_OfRef_Taking<T>> for WhileOps_UnorderedWhileSpliterator_OfRef<T> {
    fn from(v: WhileOps_UnorderedWhileSpliterator_OfRef_Taking<T>) -> WhileOps_UnorderedWhileSpliterator_OfRef<T> { v.__into_super() }
}

impl<T: Clone + Default + 'static> From<WhileOps_UnorderedWhileSpliterator_OfRef_Taking<T>> for WhileOps_UnorderedWhileSpliterator<T, Object> {
    fn from(v: WhileOps_UnorderedWhileSpliterator_OfRef_Taking<T>) -> WhileOps_UnorderedWhileSpliterator<T, Object> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking"]
    #[super_class       = "java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef<TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "WhileOps.java"]
    #[inner_classes     = "java/util/stream/WhileOps$UnorderedWhileSpliterator:java/util/stream/WhileOps:UnorderedWhileSpliterator:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfRef:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:Taking:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "WhileOps_UnorderedWhileSpliterator_OfRef<T>"]
    #[superclass_fields(s: Object, noSplitting: bool, cancel: AtomicBoolean, takeOrDrop: bool, count: i32, p: Object, t: T)]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterator;java/util/function/Consumer;java/util/stream/WhileOps$UnorderedWhileSpliterator;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking"]

    pub struct WhileOps_UnorderedWhileSpliterator_OfRef_Taking<T: Clone + Default + 'static>;

    impl<T> WhileOps_UnorderedWhileSpliterator_OfRef_Taking<T> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;ZLjava/util/function/Predicate;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<TT;>;ZLjava/util/function/Predicate<-TT;>;)V")]
        // java: <init>(Ljava/util/Spliterator;ZLjava/util/function/Predicate;)V
        pub fn new_splite_z_predic(mut s: Object, mut noSplitting: bool, mut p: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(WhileOps_UnorderedWhileSpliterator_OfRef::new_splite_z_predic(Clone::clone(&s), noSplitting, Clone::clone(&p))?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<TT;>;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking<TT;>;)V")]
        pub fn new_splite_whileo(s: Object, parent: WhileOps_UnorderedWhileSpliterator_OfRef_Taking<T>) -> Result<Self> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking.<init>:(Ljava/util/Spliterator;Ljava/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking;)V")
        }

        #[java_method(name = "tryAdvance", descriptor = "(Ljava/util/function/Consumer;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)Z")]
        pub fn tryAdvance(&self, action: Object) -> Result<bool> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking.tryAdvance:(Ljava/util/function/Consumer;)Z")
        }

        #[java_method(name = "trySplit", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TT;>;")]
        pub fn trySplit(&self) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking.trySplit:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "makeSpliterator", descriptor = "(Ljava/util/Spliterator;)Ljava/util/Spliterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<TT;>;)Ljava/util/Spliterator<TT;>;")]
        pub fn makeSpliterator(&self, s: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking.makeSpliterator:(Ljava/util/Spliterator;)Ljava/util/Spliterator;")
        }
    }
}
