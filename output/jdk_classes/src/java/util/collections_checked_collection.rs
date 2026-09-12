#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedCollection",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedCollection<E> {
    #[cfg_attr(any(), java_field(name = "c", descriptor = "Ljava/util/Collection;", access = "final"))]
    pub c: Field<Object>,
    #[cfg_attr(any(), java_field(name = "type", descriptor = "Ljava/lang/Class;", access = "final"))]
    pub type_: Field<Object>,
    #[cfg_attr(any(), java_field(name = "zeroLengthElementArray", descriptor = "[Ljava/lang/Object;", access = "private"))]
    pub zeroLengthElementArray: Field<Vec<Object>>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CheckedCollection<E> {
    // java: typeCheck(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn typeCheck(&self, o: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedCollection.typeCheck")
    }

    // java: badElementMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badElementMsg(&self, o: Object) -> Result<String> {
        todo!("abstract java/util/Collections$CheckedCollection.badElementMsg")
    }

    // java: <init>(Ljava/util/Collection;Ljava/lang/Class;)V
    pub fn new(&self, c: Object, type_: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedCollection.<init>")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$CheckedCollection.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.contains")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$CheckedCollection.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$CheckedCollection.toArray")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$CheckedCollection.toArray")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/Collections$CheckedCollection.toString")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.remove")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedCollection.clear")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, coll: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.containsAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, coll: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.removeAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, coll: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.retainAll")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedCollection.iterator")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.add")
    }

    // java: zeroLengthElementArray()[Ljava/lang/Object;
    pub fn zeroLengthElementArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$CheckedCollection.zeroLengthElementArray")
    }

    // java: checkedCopyOf(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn checkedCopyOf(&self, coll: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedCollection.checkedCopyOf")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, coll: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.addAll")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedCollection.forEach")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedCollection.removeIf")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedCollection.spliterator")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedCollection.stream")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedCollection.parallelStream")
    }
}
