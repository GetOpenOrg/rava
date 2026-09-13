#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$AbstractImmutableCollection",
    super_class = "java/util/AbstractCollection",
    interfaces  = "",
    access      = "abstract",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_AbstractImmutableCollection<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> ImmutableCollections_AbstractImmutableCollection<E> {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableCollection.<init>:()V")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableCollection.add:(Ljava/lang/Object;)Z")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableCollection.addAll:(Ljava/util/Collection;)Z")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableCollection.clear:()V")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableCollection.remove:(Ljava/lang/Object;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableCollection.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableCollection.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableCollection.retainAll:(Ljava/util/Collection;)Z")
    }
}
