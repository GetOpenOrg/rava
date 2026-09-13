#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SingletonSet",
    super_class = "java/util/AbstractSet",
    interfaces  = "java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SingletonSet<E> {
    #[cfg_attr(any(), java_field(name = "element", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub element: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SingletonSet<E> {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SingletonSet.<init>:(Ljava/lang/Object;)V")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SingletonSet.iterator:()Ljava/util/Iterator;")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SingletonSet.size:()I")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SingletonSet.contains:(Ljava/lang/Object;)Z")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SingletonSet.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SingletonSet.spliterator:()Ljava/util/Spliterator;")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SingletonSet.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SingletonSet.hashCode:()I")
    }
}
