#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Comparable",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Comparable.java",
))]
pub struct Comparable<T>(std::marker::PhantomData<T>);

impl<T: Clone + 'static> Comparable<T> {
    // java: compareTo(Ljava/lang/Object;)I
    pub fn compareTo(&self, arg0: Object) -> Result<i32> {
        todo!("abstract java/lang/Comparable.compareTo")
    }
}
