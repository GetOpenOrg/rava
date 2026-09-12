#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SetFromMap",
    super_class = "java/util/AbstractSet",
    interfaces  = "java/util/Set,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SetFromMap<E> {
    #[cfg_attr(any(), java_field(name = "m", descriptor = "Ljava/util/Map;", access = "final"))]
    pub m: Field<Object>,
    #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/util/Set;", access = "private"))]
    pub s: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SetFromMap<E> {
    // java: <init>(Ljava/util/Map;)V
    pub fn new(&self, map: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SetFromMap.<init>")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Collections$SetFromMap.clear")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$SetFromMap.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.contains")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.remove")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.add")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SetFromMap.iterator")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$SetFromMap.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$SetFromMap.toArray")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/Collections$SetFromMap.toString")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$SetFromMap.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.equals")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.containsAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.removeAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.retainAll")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SetFromMap.forEach")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SetFromMap.removeIf")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SetFromMap.spliterator")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SetFromMap.stream")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SetFromMap.parallelStream")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, stream: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SetFromMap.readObject")
    }

    // java: readObjectNoData()V
    pub fn readObjectNoData(&self) -> Result<()> {
        todo!("abstract java/util/Collections$SetFromMap.readObjectNoData")
    }
}
