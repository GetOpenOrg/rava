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
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableCollection.<init>")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableCollection.add")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableCollection.addAll")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableCollection.clear")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableCollection.remove")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableCollection.removeAll")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableCollection.removeIf")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableCollection.retainAll")
    }
}
