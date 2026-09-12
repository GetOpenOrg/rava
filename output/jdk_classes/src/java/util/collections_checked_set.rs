#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedSet",
    super_class = "java/util/Collections$CheckedCollection",
    interfaces  = "java/util/Set,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedSet<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_CheckedSet<E> {
    // java: <init>(Ljava/util/Set;Ljava/lang/Class;)V
    pub fn new(&self, s: Object, elementType: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedSet.<init>")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedSet.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$CheckedSet.hashCode")
    }
}
