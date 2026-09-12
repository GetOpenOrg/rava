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
    pub fn new(q: Object) -> Result<Self> {
        let this = Self { q: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractQueue.<init>:()V */
        this.q.set(q);
        Ok(this)
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: E) -> Result<bool> {
        let this = self;
        this.q.get().addFirst(e)?;
        Ok(1i32)
    }

    // java: offer(Ljava/lang/Object;)Z
    pub fn offer(&self, e: E) -> Result<bool> {
        let this = self;
        let _t0 = this.q.get().offerFirst(e)?;
        Ok(_t0)
    }

    // java: poll()Ljava/lang/Object;
    pub fn poll(&self) -> Result<E> {
        let this = self;
        let _t0 = this.q.get().pollFirst()?;
        Ok(_t0)
    }

    // java: remove()Ljava/lang/Object;
    // java: remove()Ljava/lang/Object;
    pub fn remove(&self) -> Result<E> {
        let this = self;
        let _t0 = this.q.get().removeFirst()?;
        Ok(_t0)
    }

    // java: peek()Ljava/lang/Object;
    pub fn peek(&self) -> Result<E> {
        let this = self;
        let _t0 = this.q.get().peekFirst()?;
        Ok(_t0)
    }

    // java: element()Ljava/lang/Object;
    pub fn element(&self) -> Result<E> {
        let this = self;
        let _t0 = this.q.get().getFirst()?;
        Ok(_t0)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.q.get().clear()?;
        Ok(())
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.q.get().size()?;
        Ok(_t0)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.q.get().isEmpty()?;
        Ok(_t0)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.q.get().contains(o)?;
        Ok(_t0)
    }

    // java: remove(Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.q.get().remove(o)?;
        Ok(_t0)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.q.get().iterator()?;
        Ok(_t0)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.q.get().toArray()?;
        Ok(_t0)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.q.get().toArray(a)?;
        Ok(_t0)
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.q.get().toArray(f)?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.q.get().toString()?;
        Ok(_t0)
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.q.get().containsAll(c)?;
        Ok(_t0)
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.q.get().removeAll(c)?;
        Ok(_t0)
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.q.get().retainAll(c)?;
        Ok(_t0)
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        this.q.get().forEach(action)?;
        Ok(())
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.q.get().removeIf(filter)?;
        Ok(_t0)
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.q.get().spliterator()?;
        Ok(_t0)
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.q.get().stream()?;
        Ok(_t0)
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.q.get().parallelStream()?;
        Ok(_t0)
    }
}
