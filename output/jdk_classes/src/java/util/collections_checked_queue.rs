#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedQueue",
    super_class = "java/util/Collections$CheckedCollection",
    interfaces  = "java/util/Queue,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedQueue<E> {
    #[cfg_attr(any(), java_field(name = "queue", descriptor = "Ljava/util/Queue;", access = "final"))]
    pub queue: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CheckedQueue<E> {
    // java: <init>(Ljava/util/Queue;Ljava/lang/Class;)V
    pub fn new(&self, queue: Object, elementType: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedQueue.<init>")
    }

    // java: element()Ljava/lang/Object;
    pub fn element(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedQueue.element")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedQueue.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$CheckedQueue.hashCode")
    }

    // java: peek()Ljava/lang/Object;
    pub fn peek(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedQueue.peek")
    }

    // java: poll()Ljava/lang/Object;
    pub fn poll(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedQueue.poll")
    }

    // java: remove()Ljava/lang/Object;
    pub fn remove(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedQueue.remove")
    }

    // java: offer(Ljava/lang/Object;)Z
    pub fn offer(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedQueue.offer")
    }
}
