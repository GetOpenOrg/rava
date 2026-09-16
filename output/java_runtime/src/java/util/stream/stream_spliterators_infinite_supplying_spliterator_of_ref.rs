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

impl<T: Clone + Default + 'static> From<StreamSpliterators_InfiniteSupplyingSpliterator_OfRef<T>> for StreamSpliterators_InfiniteSupplyingSpliterator<T> {
    fn from(v: StreamSpliterators_InfiniteSupplyingSpliterator_OfRef<T>) -> StreamSpliterators_InfiniteSupplyingSpliterator<T> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfRef"]
    #[super_class       = "java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator<TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StreamSpliterators.java"]
    #[inner_classes     = "java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator:java/util/stream/StreamSpliterators:InfiniteSupplyingSpliterator:1032;java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfRef:java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator:OfRef:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "StreamSpliterators_InfiniteSupplyingSpliterator<T>"]
    #[superclass_fields(estimate: i64)]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterator;java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator;java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfRef"]

    pub struct StreamSpliterators_InfiniteSupplyingSpliterator_OfRef<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/util/function/Supplier;", access = "package", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/Supplier<+TT;>;"))]
        pub s: Object,
    }

    impl<T> StreamSpliterators_InfiniteSupplyingSpliterator_OfRef<T> {
        #[java_method(name = "<init>", descriptor = "(JLjava/util/function/Supplier;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/Supplier<+TT;>;)V")]
        pub fn new(mut size: i64, mut s: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(StreamSpliterators_InfiniteSupplyingSpliterator::new(size)?);
            this.__set_s(Clone::clone(&s));
            Ok(this)
        }

        #[java_method(name = "tryAdvance", descriptor = "(Ljava/util/function/Consumer;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)Z")]
        pub fn tryAdvance(&self, action: Object) -> Result<bool> {
            panic!("stub: java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfRef.tryAdvance:(Ljava/util/function/Consumer;)Z")
        }

        #[java_method(name = "trySplit", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TT;>;")]
        pub fn trySplit(&self) -> Result<Object> {
            panic!("stub: java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfRef.trySplit:()Ljava/util/Spliterator;")
        }
    }
}
