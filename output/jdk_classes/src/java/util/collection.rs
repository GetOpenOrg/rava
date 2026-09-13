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
        panic!("stub: java/util/Collection.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collection.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.contains:(Ljava/lang/Object;)Z")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collection.iterator:()Ljava/util/Iterator;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collection.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, arg0: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collection.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, generator: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collection.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.add:(Ljava/lang/Object;)Z")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.remove:(Ljava/lang/Object;)Z")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.addAll:(Ljava/util/Collection;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collection.clear:()V")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collection.hashCode:()I")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collection.spliterator:()Ljava/util/Spliterator;")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Collection.stream:()Ljava/util/stream/Stream;")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/Collection.parallelStream:()Ljava/util/stream/Stream;")
    }
}
