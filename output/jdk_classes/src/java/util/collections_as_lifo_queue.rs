#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$AsLIFOQueue",
    super_class = "java/util/AbstractQueue",
    interfaces  = "java/util/Queue,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_AsLIFOQueue<E> {
    #[cfg_attr(any(), java_field(name = "q", descriptor = "Ljava/util/Deque;", access = "private final"))]
    pub q: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_AsLIFOQueue<E> {
    // java: <init>(Ljava/util/Deque;)V
    pub fn new(&self, q: Object) -> Result<()> {
        todo!("abstract java/util/Collections$AsLIFOQueue.<init>")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.add")
    }

    // java: offer(Ljava/lang/Object;)Z
    pub fn offer(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.offer")
    }

    // java: poll()Ljava/lang/Object;
    pub fn poll(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$AsLIFOQueue.poll")
    }

    // java: remove()Ljava/lang/Object;
    pub fn remove(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$AsLIFOQueue.remove")
    }

    // java: peek()Ljava/lang/Object;
    pub fn peek(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$AsLIFOQueue.peek")
    }

    // java: element()Ljava/lang/Object;
    pub fn element(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$AsLIFOQueue.element")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Collections$AsLIFOQueue.clear")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$AsLIFOQueue.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.contains")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.remove")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$AsLIFOQueue.iterator")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$AsLIFOQueue.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$AsLIFOQueue.toArray")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections$AsLIFOQueue.toArray")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/Collections$AsLIFOQueue.toString")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.containsAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.removeAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.retainAll")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Collections$AsLIFOQueue.forEach")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$AsLIFOQueue.removeIf")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$AsLIFOQueue.spliterator")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$AsLIFOQueue.stream")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$AsLIFOQueue.parallelStream")
    }
}
