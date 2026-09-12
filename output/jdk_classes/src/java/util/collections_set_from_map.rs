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
    pub fn new(map: Object) -> Result<Self> {
        let this = Self { m: Field::new(Default::default()), s: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.m.set(map);
        let _t0 = map.keySet()?;
        this.s.set(_t0);
        Ok(this)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.m.get().clear()?;
        Ok(())
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.m.get().size()?;
        Ok(_t0)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().isEmpty()?;
        Ok(_t0)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().containsKey(o)?;
        Ok(_t0)
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().remove(o)?;
        Ok(!_t0.is_none())
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: E) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().put(e, Boolean::TRUE())?;
        Ok(_t0.is_none())
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.s.get().iterator()?;
        Ok(_t0)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.s.get().toArray()?;
        Ok(_t0)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.s.get().toArray(a)?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.s.get().toString()?;
        Ok(_t0)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.s.get().hashCode()?;
        Ok(_t0)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.s.get().equals(o)?;
        Ok(_t0!=0i32)
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.s.get().containsAll(c)?;
        Ok(_t0)
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.s.get().removeAll(c)?;
        Ok(_t0)
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.s.get().retainAll(c)?;
        Ok(_t0)
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        this.s.get().forEach(action)?;
        Ok(())
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.s.get().removeIf(filter)?;
        Ok(_t0)
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.s.get().spliterator()?;
        Ok(_t0)
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.s.get().stream()?;
        Ok(_t0)
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.s.get().parallelStream()?;
        Ok(_t0)
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, stream: Object) -> Result<()> {
        let this = self;
        stream.defaultReadObject()?;
        let _t0 = this.m.get().keySet()?;
        this.s.set(_t0);
        Ok(())
    }

    // java: readObjectNoData()V
    pub fn readObjectNoData(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
