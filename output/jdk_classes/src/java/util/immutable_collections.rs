#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections;

impl ImmutableCollections {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections.<init>")
    }

    // java: uoe()Ljava/lang/UnsupportedOperationException;
    pub fn uoe() -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections.uoe")
    }

    // java: listCopy(Ljava/util/Collection;)Ljava/util/List;
    pub fn listCopy(coll: Object) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections.listCopy")
    }

    // java: listFromArray([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromArray(input: Vec<Object>) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections.listFromArray")
    }

    // java: listFromTrustedArray([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromTrustedArray(input: Vec<Object>) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections.listFromTrustedArray")
    }

    // java: listFromTrustedArrayNullsAllowed([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromTrustedArrayNullsAllowed(input: Vec<Object>) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections.listFromTrustedArrayNullsAllowed")
    }
}
