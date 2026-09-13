#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SingletonList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/RandomAccess,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SingletonList<E> {
    #[cfg_attr(any(), java_field(name = "element", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub element: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SingletonList<E> {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new(&self, obj: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SingletonList.<init>:(Ljava/lang/Object;)V")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SingletonList.iterator:()Ljava/util/Iterator;")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SingletonList.size:()I")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SingletonList.contains:(Ljava/lang/Object;)Z")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$SingletonList.get:(I)Ljava/lang/Object;")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SingletonList.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SingletonList.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SingletonList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SingletonList.sort:(Ljava/util/Comparator;)V")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SingletonList.spliterator:()Ljava/util/Spliterator;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SingletonList.hashCode:()I")
    }
}
