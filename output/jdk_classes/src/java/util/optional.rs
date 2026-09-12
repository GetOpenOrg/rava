#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Optional",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Optional.java",
))]
pub struct Optional<T> {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub value: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Optional<T> {
    // java: empty()Ljava/util/Optional;
    pub fn empty() -> Result<Object> {
        todo!("abstract java/util/Optional.empty")
    }

    // java: <init>(Ljava/lang/Object;)V
    pub fn new(&self, value: Object) -> Result<()> {
        todo!("abstract java/util/Optional.<init>")
    }

    // java: of(Ljava/lang/Object;)Ljava/util/Optional;
    pub fn of(value: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.of")
    }

    // java: ofNullable(Ljava/lang/Object;)Ljava/util/Optional;
    pub fn ofNullable(value: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.ofNullable")
    }

    // java: get()Ljava/lang/Object;
    pub fn get(&self) -> Result<Object> {
        todo!("abstract java/util/Optional.get")
    }

    // java: isPresent()Z
    pub fn isPresent(&self) -> Result<bool> {
        todo!("abstract java/util/Optional.isPresent")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Optional.isEmpty")
    }

    // java: ifPresent(Ljava/util/function/Consumer;)V
    pub fn ifPresent(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Optional.ifPresent")
    }

    // java: ifPresentOrElse(Ljava/util/function/Consumer;Ljava/lang/Runnable;)V
    pub fn ifPresentOrElse(&self, action: Object, emptyAction: Object) -> Result<()> {
        todo!("abstract java/util/Optional.ifPresentOrElse")
    }

    // java: filter(Ljava/util/function/Predicate;)Ljava/util/Optional;
    pub fn filter(&self, predicate: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.filter")
    }

    // java: map(Ljava/util/function/Function;)Ljava/util/Optional;
    pub fn map(&self, mapper: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.map")
    }

    // java: flatMap(Ljava/util/function/Function;)Ljava/util/Optional;
    pub fn flatMap(&self, mapper: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.flatMap")
    }

    // java: or(Ljava/util/function/Supplier;)Ljava/util/Optional;
    pub fn or(&self, supplier: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.or")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        todo!("abstract java/util/Optional.stream")
    }

    // java: orElse(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn orElse(&self, other: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.orElse")
    }

    // java: orElseGet(Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn orElseGet(&self, supplier: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.orElseGet")
    }

    // java: orElseThrow()Ljava/lang/Object;
    pub fn orElseThrow(&self) -> Result<Object> {
        todo!("abstract java/util/Optional.orElseThrow")
    }

    // java: orElseThrow(Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn orElseThrow__suppli(&self, exceptionSupplier: Object) -> Result<Object> {
        todo!("abstract java/util/Optional.orElseThrow")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/util/Optional.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Optional.hashCode")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/Optional.toString")
    }
}
