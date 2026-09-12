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
    // java: <init>(Ljava/util/Collection;)V
    pub fn new__coll(c: Object) -> Result<Self> {
        let this = Self { c: Field::new(Default::default()), mutex: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(c)?;
        this.c.set(_t0);
        this.mutex.set(this);
        Ok(this)
    }

    // java: <init>(Ljava/util/Collection;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/Collection;Ljava/lang/Object;)V
    pub fn new__coll_obj(c: Object, mutex: Object) -> Result<Self> {
        let this = Self { c: Field::new(Default::default()), mutex: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(c)?;
        this.c.set(_t0);
        let _t1: Object = Objects::requireNonNull__obj(mutex)?;
        this.mutex.set(_t1);
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().size()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: i32 = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().isEmpty()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().contains(o)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().toArray()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Vec<Object> = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().toArray(a)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Vec<Object> = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().toArray(f)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Vec<Object> = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.c.get().iterator()?;
        Ok(_t0)
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: E) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().add(e)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().remove(o)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, coll: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().containsAll(coll)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, coll: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().addAll(coll)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, coll: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().removeAll(coll)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, coll: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().retainAll(coll)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.c.get().clear()?;
        /* TODO: monitorexit  */
        let mut local_2: Object = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().toString()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: String = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, consumer: Object) -> Result<()> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.c.get().forEach(consumer)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().removeIf(filter)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.c.get().spliterator()?;
        Ok(_t0)
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.c.get().stream()?;
        Ok(_t0)
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.c.get().parallelStream()?;
        Ok(_t0)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        s.defaultWriteObject()?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
