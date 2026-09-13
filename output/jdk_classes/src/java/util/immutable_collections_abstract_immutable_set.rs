#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$AbstractImmutableSet",
    super_class = "java/util/ImmutableCollections$AbstractImmutableCollection",
    interfaces  = "java/util/Set",
    access      = "abstract",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_AbstractImmutableSet<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> ImmutableCollections_AbstractImmutableSet<E> {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableSet.<init>:()V")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableSet.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableSet.hashCode:()I")
    }
}
