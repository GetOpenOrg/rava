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
    pub fn new(queue: Object, elementType: Object) -> Result<Self> {
        let this = Self { queue: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$CheckedCollection.<init>:(Ljava/util/Collection;Ljava/lang/Class;)V */
        this.queue.set(queue);
        Ok(this)
    }

    // java: element()Ljava/lang/Object;
    pub fn element(&self) -> Result<E> {
        let this = self;
        let _t0 = this.queue.get().element()?;
        Ok(_t0)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.c.get().equals(o)?;
        Ok(_t0!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.c.get().hashCode()?;
        Ok(_t0)
    }

    // java: peek()Ljava/lang/Object;
    pub fn peek(&self) -> Result<E> {
        let this = self;
        let _t0 = this.queue.get().peek()?;
        Ok(_t0)
    }

    // java: poll()Ljava/lang/Object;
    pub fn poll(&self) -> Result<E> {
        let this = self;
        let _t0 = this.queue.get().poll()?;
        Ok(_t0)
    }

    // java: remove()Ljava/lang/Object;
    pub fn remove(&self) -> Result<E> {
        let this = self;
        let _t0 = this.queue.get().remove()?;
        Ok(_t0)
    }

    // java: offer(Ljava/lang/Object;)Z
    pub fn offer(&self, e: E) -> Result<bool> {
        let this = self;
        let _t0 = this.typeCheck(e)?;
        let _t1 = this.queue.get().offer(_t0)?;
        Ok(_t1)
    }
}
