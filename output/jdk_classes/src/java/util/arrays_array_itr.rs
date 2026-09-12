#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays$ArrayItr",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "Arrays.java",
))]
pub struct Arrays_ArrayItr<E> {
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I", access = "private"))]
    pub cursor: Field<i32>,
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Vec<Object>>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Arrays_ArrayItr<E> {
    // java: <init>([Ljava/lang/Object;)V
    pub fn new(&self, a: Vec<Object>) -> Result<()> {
        todo!("abstract java/util/Arrays$ArrayItr.<init>")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/Arrays$ArrayItr.hasNext")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        todo!("abstract java/util/Arrays$ArrayItr.next")
    }
}
