#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedCollection",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedCollection<E> {
    #[cfg_attr(any(), java_field(name = "c", descriptor = "Ljava/util/Collection;", access = "final"))]
    pub c: Field<Object>,
    #[cfg_attr(any(), java_field(name = "mutex", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub mutex: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SynchronizedCollection<E> {
    // java: <init>(Ljava/util/Collection;)V
    pub fn new__coll(&self, c: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedCollection.<init>")
    }

    // java: <init>(Ljava/util/Collection;Ljava/lang/Object;)V
    pub fn new__coll_obj(&self, c: Object, mutex: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedCollection.<init>")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$SynchronizedCollection.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.contains")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$SynchronizedCollection.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$SynchronizedCollection.toArray")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$SynchronizedCollection.toArray")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedCollection.iterator")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.add")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.remove")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, coll: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.containsAll")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, coll: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.addAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, coll: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.removeAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, coll: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.retainAll")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedCollection.clear")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/Collections$SynchronizedCollection.toString")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, consumer: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedCollection.forEach")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedCollection.removeIf")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedCollection.spliterator")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedCollection.stream")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedCollection.parallelStream")
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedCollection.writeObject")
    }
}
