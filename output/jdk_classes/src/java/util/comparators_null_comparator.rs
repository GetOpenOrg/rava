#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Comparators$NullComparator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Comparator,java/io/Serializable",
    access      = "final",
    source      = "Comparators.java",
))]
pub struct Comparators_NullComparator<T> {
    #[cfg_attr(any(), java_field(name = "nullFirst", descriptor = "Z", access = "private final"))]
    pub nullFirst: Field<bool>,
    #[cfg_attr(any(), java_field(name = "real", descriptor = "Ljava/util/Comparator;", access = "private final"))]
    pub real: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Comparators_NullComparator<T> {
    // java: <init>(ZLjava/util/Comparator;)V
    pub fn new(&self, nullFirst: bool, real: Object) -> Result<()> {
        todo!("abstract java/util/Comparators$NullComparator.<init>")
    }

    // java: compare(Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn compare(&self, a: Object, b: Object) -> Result<i32> {
        todo!("abstract java/util/Comparators$NullComparator.compare")
    }

    // java: thenComparing(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn thenComparing(&self, other: Object) -> Result<Object> {
        todo!("abstract java/util/Comparators$NullComparator.thenComparing")
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        todo!("abstract java/util/Comparators$NullComparator.reversed")
    }
}
