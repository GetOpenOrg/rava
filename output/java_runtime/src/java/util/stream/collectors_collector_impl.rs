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

impl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> From<Collectors_CollectorImpl<T, A, R>> for Record {
    fn from(v: Collectors_CollectorImpl<T, A, R>) -> Record { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/Collectors$CollectorImpl"]
    #[super_class       = "java/lang/Record"]
    #[interfaces        = "java/util/stream/Collector"]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>Ljava/lang/Record;Ljava/util/stream/Collector<TT;TA;TR;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Collectors.java"]
    #[inner_classes     = "java/util/stream/Collectors$CollectorImpl:java/util/stream/Collectors:CollectorImpl:24;java/util/stream/Collector$Characteristics:java/util/stream/Collector:Characteristics:16409;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Record"]
    #[all_supertypes    = "java/lang/Object;java/lang/Record;java/util/stream/Collector;java/util/stream/Collectors$CollectorImpl"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Collectors_CollectorImpl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "supplier", descriptor = "Ljava/util/function/Supplier;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/Supplier<TA;>;"))]
        pub supplier: Object,
        #[cfg_attr(any(), java_field(name = "accumulator", descriptor = "Ljava/util/function/BiConsumer;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/BiConsumer<TA;TT;>;"))]
        pub accumulator: Object,
        #[cfg_attr(any(), java_field(name = "combiner", descriptor = "Ljava/util/function/BinaryOperator;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/BinaryOperator<TA;>;"))]
        pub combiner: Object,
        #[cfg_attr(any(), java_field(name = "finisher", descriptor = "Ljava/util/function/Function;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/Function<TA;TR;>;"))]
        pub finisher: Object,
        #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "Ljava/util/Set;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
        pub characteristics: Object,
    }

    impl<T, A, R> Collectors_CollectorImpl<T, A, R> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<TA;>;Ljava/util/function/BiConsumer<TA;TT;>;Ljava/util/function/BinaryOperator<TA;>;Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;)V")]
        // java: <init>(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V
        pub fn new_suppli_bicons_binary_set(mut supplier: Object, mut accumulator: Object, mut combiner: Object, mut characteristics: Object) -> Result<Self> {
            let mut this = Self::default();
            let _t0: Object = Collectors::castingIdentity()?;
            this = Collectors_CollectorImpl::new_suppli_bicons_binary_functi_set(Clone::clone(&supplier), Clone::clone(&accumulator), Clone::clone(&combiner), Clone::clone(&_t0), Clone::clone(&characteristics))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<TA;>;Ljava/util/function/BiConsumer<TA;TT;>;Ljava/util/function/BinaryOperator<TA;>;Ljava/util/function/Function<TA;TR;>;Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;)V", method_parameters = "supplier:0;accumulator:0;combiner:0;finisher:0;characteristics:0")]
        // java: <init>(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V
        pub fn new_suppli_bicons_binary_functi_set(mut supplier: Object, mut accumulator: Object, mut combiner: Object, mut finisher: Object, mut characteristics: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Record::new()?);
            this.__set_supplier(Clone::clone(&supplier));
            this.__set_accumulator(Clone::clone(&accumulator));
            this.__set_combiner(Clone::clone(&combiner));
            this.__set_finisher(Clone::clone(&finisher));
            this.__set_characteristics(Clone::clone(&characteristics));
            Ok(this)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/stream/Collectors$CollectorImpl.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "supplier", descriptor = "()Ljava/util/function/Supplier;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/Supplier<TA;>;")]
        pub fn supplier(&self) -> Result<Object> {
            let this = self;
            Ok(this.__get_supplier())
        }

        #[java_method(name = "accumulator", descriptor = "()Ljava/util/function/BiConsumer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/BiConsumer<TA;TT;>;")]
        pub fn accumulator(&self) -> Result<BiConsumer<A, T>> {
            let this = self;
            Ok(Default::default())
        }

        #[java_method(name = "combiner", descriptor = "()Ljava/util/function/BinaryOperator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/BinaryOperator<TA;>;")]
        pub fn combiner(&self) -> Result<Object> {
            let this = self;
            Ok(this.__get_combiner())
        }

        #[java_method(name = "finisher", descriptor = "()Ljava/util/function/Function;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/Function<TA;TR;>;")]
        pub fn finisher(&self) -> Result<Object> {
            let this = self;
            Ok(this.__get_finisher())
        }

        #[java_method(name = "characteristics", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;")]
        pub fn characteristics(&self) -> Result<Object> {
            let this = self;
            Ok(this.__get_characteristics())
        }
    }
}
