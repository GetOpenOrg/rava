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
        panic!("stub: java/util/Optional.empty:()Ljava/util/Optional;")
    }

    // java: <init>(Ljava/lang/Object;)V
    pub fn new(&self, value: Object) -> Result<()> {
        panic!("stub: java/util/Optional.<init>:(Ljava/lang/Object;)V")
    }

    // java: of(Ljava/lang/Object;)Ljava/util/Optional;
    pub fn of(value: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.of:(Ljava/lang/Object;)Ljava/util/Optional;")
    }

    // java: ofNullable(Ljava/lang/Object;)Ljava/util/Optional;
    pub fn ofNullable(value: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.ofNullable:(Ljava/lang/Object;)Ljava/util/Optional;")
    }

    // java: get()Ljava/lang/Object;
    pub fn get(&self) -> Result<Object> {
        panic!("stub: java/util/Optional.get:()Ljava/lang/Object;")
    }

    // java: isPresent()Z
    pub fn isPresent(&self) -> Result<bool> {
        panic!("stub: java/util/Optional.isPresent:()Z")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Optional.isEmpty:()Z")
    }

    // java: ifPresent(Ljava/util/function/Consumer;)V
    pub fn ifPresent(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Optional.ifPresent:(Ljava/util/function/Consumer;)V")
    }

    // java: ifPresentOrElse(Ljava/util/function/Consumer;Ljava/lang/Runnable;)V
    pub fn ifPresentOrElse(&self, action: Object, emptyAction: Object) -> Result<()> {
        panic!("stub: java/util/Optional.ifPresentOrElse:(Ljava/util/function/Consumer;Ljava/lang/Runnable;)V")
    }

    // java: filter(Ljava/util/function/Predicate;)Ljava/util/Optional;
    pub fn filter(&self, predicate: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.filter:(Ljava/util/function/Predicate;)Ljava/util/Optional;")
    }

    // java: map(Ljava/util/function/Function;)Ljava/util/Optional;
    pub fn map(&self, mapper: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.map:(Ljava/util/function/Function;)Ljava/util/Optional;")
    }

    // java: flatMap(Ljava/util/function/Function;)Ljava/util/Optional;
    pub fn flatMap(&self, mapper: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.flatMap:(Ljava/util/function/Function;)Ljava/util/Optional;")
    }

    // java: or(Ljava/util/function/Supplier;)Ljava/util/Optional;
    pub fn or(&self, supplier: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.or:(Ljava/util/function/Supplier;)Ljava/util/Optional;")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Optional.stream:()Ljava/util/stream/Stream;")
    }

    // java: orElse(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn orElse(&self, other: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.orElse:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: orElseGet(Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn orElseGet(&self, supplier: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.orElseGet:(Ljava/util/function/Supplier;)Ljava/lang/Object;")
    }

    // java: orElseThrow()Ljava/lang/Object;
    pub fn orElseThrow(&self) -> Result<Object> {
        panic!("stub: java/util/Optional.orElseThrow:()Ljava/lang/Object;")
    }

    // java: orElseThrow(Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn orElseThrow__suppli(&self, exceptionSupplier: Object) -> Result<Object> {
        panic!("stub: java/util/Optional.orElseThrow:(Ljava/util/function/Supplier;)Ljava/lang/Object;")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/util/Optional.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Optional.hashCode:()I")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/Optional.toString:()Ljava/lang/String;")
    }
}
