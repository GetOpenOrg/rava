#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/stream/Collectors$CollectorImpl",
    super_class       = "java/lang/Record",
    interfaces        = "java/util/stream/Collector",
    access            = "package",
    modifiers         = "final",
    generic_signature = "<T:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>Ljava/lang/Record;Ljava/util/stream/Collector<TT;TA;TR;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Collectors.java",
    inner_classes     = "java/util/stream/Collectors$CollectorImpl:java/util/stream/Collectors:CollectorImpl:24;java/util/stream/Collector$Characteristics:java/util/stream/Collector:Characteristics:16409;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "java/lang/Object;java/lang/Record;java/util/stream/Collector;java/util/stream/Collectors$CollectorImpl",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Collectors_CollectorImpl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> {
    pub _super: Record,
    #[cfg_attr(any(), java_field(name = "supplier", descriptor = "Ljava/util/function/Supplier;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/Supplier<TA;>;"))]
    pub supplier: JField<Object>,
    #[cfg_attr(any(), java_field(name = "accumulator", descriptor = "Ljava/util/function/BiConsumer;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/BiConsumer<TA;TT;>;"))]
    pub accumulator: JField<Object>,
    #[cfg_attr(any(), java_field(name = "combiner", descriptor = "Ljava/util/function/BinaryOperator;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/BinaryOperator<TA;>;"))]
    pub combiner: JField<Object>,
    #[cfg_attr(any(), java_field(name = "finisher", descriptor = "Ljava/util/function/Function;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/Function<TA;TR;>;"))]
    pub finisher: JField<Object>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "Ljava/util/Set;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    pub characteristics: JField<Set<Object>>,
    pub _phantom: std::marker::PhantomData<(T, A, R,)>,
}

impl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> Collectors_CollectorImpl<T, A, R> {
    pub fn as_record(&self) -> &Record { &self._super }
    pub fn into_record(self) -> Record { self._super }
}

impl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> From<Collectors_CollectorImpl<T, A, R>> for Record {
    fn from(v: Collectors_CollectorImpl<T, A, R>) -> Record { v._super }
}

impl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> From<Collectors_CollectorImpl<T, A, R>> for Collector<T, A, R> {
    fn from(v: Collectors_CollectorImpl<T, A, R>) -> Collector<T, A, R> { Default::default() }
}

impl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> Collectors_CollectorImpl<T, A, R> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<TA;>;Ljava/util/function/BiConsumer<TA;TT;>;Ljava/util/function/BinaryOperator<TA;>;Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;)V"))]
    // java: <init>(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V
    pub fn new_suppli_bicons_binary_set(mut supplier: Object, mut accumulator: Object, mut combiner: Object, mut characteristics: Set<Object>) -> Result<Self> {
        let mut this = Self { _super: Default::default(), supplier: JField::new(Default::default()), accumulator: JField::new(Default::default()), combiner: JField::new(Default::default()), finisher: JField::new(Default::default()), characteristics: JField::new(Default::default()), _phantom: std::marker::PhantomData, ..Default::default() };
        let _t0: Object = Collectors::castingIdentity()?;
        this = Collectors_CollectorImpl::new_suppli_bicons_binary_functi_set(Clone::clone(&supplier), Clone::clone(&accumulator), Clone::clone(&combiner), Clone::clone(&_t0), Clone::clone(&characteristics))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<TA;>;Ljava/util/function/BiConsumer<TA;TT;>;Ljava/util/function/BinaryOperator<TA;>;Ljava/util/function/Function<TA;TR;>;Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;)V", method_parameters = "supplier:0;accumulator:0;combiner:0;finisher:0;characteristics:0"))]
    // java: <init>(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V
    pub fn new_suppli_bicons_binary_functi_set(mut supplier: Object, mut accumulator: Object, mut combiner: Object, mut finisher: Object, mut characteristics: Set<Object>) -> Result<Self> {
        let mut this = Self { _super: Default::default(), supplier: JField::new(Default::default()), accumulator: JField::new(Default::default()), combiner: JField::new(Default::default()), finisher: JField::new(Default::default()), characteristics: JField::new(Default::default()), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = Record::new()?;
        this.supplier.set(Clone::clone(&supplier));
        this.accumulator.set(Clone::clone(&accumulator));
        this.combiner.set(Clone::clone(&combiner));
        this.finisher.set(Clone::clone(&finisher));
        this.characteristics.set(Clone::clone(&characteristics));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/stream/Collectors$CollectorImpl.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/stream/Collectors$CollectorImpl.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/stream/Collectors$CollectorImpl.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "supplier", descriptor = "()Ljava/util/function/Supplier;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/Supplier<TA;>;"))]
    pub fn supplier(&self) -> Result<Object> {
        let this = self;
        Ok(this.supplier.get())
    }

    #[cfg_attr(any(), java_method(name = "accumulator", descriptor = "()Ljava/util/function/BiConsumer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/BiConsumer<TA;TT;>;"))]
    pub fn accumulator(&self) -> Result<Object> {
        let this = self;
        Ok(this.accumulator.get())
    }

    #[cfg_attr(any(), java_method(name = "combiner", descriptor = "()Ljava/util/function/BinaryOperator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/BinaryOperator<TA;>;"))]
    pub fn combiner(&self) -> Result<Object> {
        let this = self;
        Ok(this.combiner.get())
    }

    #[cfg_attr(any(), java_method(name = "finisher", descriptor = "()Ljava/util/function/Function;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/Function<TA;TR;>;"))]
    pub fn finisher(&self) -> Result<Object> {
        let this = self;
        Ok(this.finisher.get())
    }

    #[cfg_attr(any(), java_method(name = "characteristics", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    pub fn characteristics(&self) -> Result<Set<Object>> {
        let this = self;
        Ok(this.characteristics.get())
    }
}
