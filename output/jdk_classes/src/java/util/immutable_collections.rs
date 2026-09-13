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
        panic!("stub: java/util/ImmutableCollections.<init>:()V")
    }

    // java: uoe()Ljava/lang/UnsupportedOperationException;
    pub fn uoe() -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections.uoe:()Ljava/lang/UnsupportedOperationException;")
    }

    // java: listCopy(Ljava/util/Collection;)Ljava/util/List;
    pub fn listCopy(coll: Object) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections.listCopy:(Ljava/util/Collection;)Ljava/util/List;")
    }

    // java: listFromArray([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromArray(input: Vec<Object>) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections.listFromArray:([Ljava/lang/Object;)Ljava/util/List;")
    }

    // java: listFromTrustedArray([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromTrustedArray(input: Vec<Object>) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections.listFromTrustedArray:([Ljava/lang/Object;)Ljava/util/List;")
    }

    // java: listFromTrustedArrayNullsAllowed([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromTrustedArrayNullsAllowed(input: Vec<Object>) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections.listFromTrustedArrayNullsAllowed:([Ljava/lang/Object;)Ljava/util/List;")
    }
}
