#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$ReverseComparator2",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Comparator,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_ReverseComparator2<T> {
    #[cfg_attr(any(), java_field(name = "cmp", descriptor = "Ljava/util/Comparator;", access = "final"))]
    pub cmp: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Collections_ReverseComparator2<T> {
    // java: <init>(Ljava/util/Comparator;)V
    pub fn new(&self, cmp: Object) -> Result<()> {
        todo!("abstract java/util/Collections$ReverseComparator2.<init>")
    }

    // java: compare(Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn compare(&self, t1: Object, t2: Object) -> Result<i32> {
        todo!("abstract java/util/Collections$ReverseComparator2.compare")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$ReverseComparator2.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$ReverseComparator2.hashCode")
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$ReverseComparator2.reversed")
    }
}
