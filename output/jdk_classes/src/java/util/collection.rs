#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collection",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/Iterable",
    access      = "public abstract",
    source      = "Collection.java",
))]
pub struct Collection<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collection<E> {
    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Collection.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Collection.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.contains")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collection.iterator")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collection.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, arg0: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collection.toArray")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, generator: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collection.toArray")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.add")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.remove")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.containsAll")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.addAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.removeAll")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.removeIf")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.retainAll")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Collection.clear")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Collection.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collection.hashCode")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collection.spliterator")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        todo!("abstract java/util/Collection.stream")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        todo!("abstract java/util/Collection.parallelStream")
    }
}
