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
        panic!("stub: java/util/Collections$AsLIFOQueue.<init>:(Ljava/util/Deque;)V")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.add:(Ljava/lang/Object;)Z")
    }

    // java: offer(Ljava/lang/Object;)Z
    pub fn offer(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.offer:(Ljava/lang/Object;)Z")
    }

    // java: poll()Ljava/lang/Object;
    pub fn poll(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$AsLIFOQueue.poll:()Ljava/lang/Object;")
    }

    // java: remove()Ljava/lang/Object;
    pub fn remove(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$AsLIFOQueue.remove:()Ljava/lang/Object;")
    }

    // java: peek()Ljava/lang/Object;
    pub fn peek(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$AsLIFOQueue.peek:()Ljava/lang/Object;")
    }

    // java: element()Ljava/lang/Object;
    pub fn element(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$AsLIFOQueue.element:()Ljava/lang/Object;")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collections$AsLIFOQueue.clear:()V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$AsLIFOQueue.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.contains:(Ljava/lang/Object;)Z")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.remove:(Ljava/lang/Object;)Z")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$AsLIFOQueue.iterator:()Ljava/util/Iterator;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$AsLIFOQueue.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$AsLIFOQueue.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$AsLIFOQueue.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/Collections$AsLIFOQueue.toString:()Ljava/lang/String;")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$AsLIFOQueue.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$AsLIFOQueue.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$AsLIFOQueue.spliterator:()Ljava/util/Spliterator;")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$AsLIFOQueue.stream:()Ljava/util/stream/Stream;")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$AsLIFOQueue.parallelStream:()Ljava/util/stream/Stream;")
    }
}
