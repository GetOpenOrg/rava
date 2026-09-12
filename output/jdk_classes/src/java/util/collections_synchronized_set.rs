#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedSet",
    super_class = "java/util/Collections$SynchronizedCollection",
    interfaces  = "java/util/Set",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedSet<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_SynchronizedSet<E> {
    // java: <init>(Ljava/util/Set;)V
    // java: <init>(Ljava/util/Set;)V
    pub fn new__set(s: Object) -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/Collections$SynchronizedCollection.<init>:(Ljava/util/Collection;)V */
        Ok(this)
    }

    // java: <init>(Ljava/util/Set;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/Set;Ljava/lang/Object;)V
    pub fn new__set_obj(s: Object, mutex: Object) -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/Collections$SynchronizedCollection.<init>:(Ljava/util/Collection;Ljava/lang/Object;)V */
        Ok(this)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().equals(o)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.c.get().hashCode()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: i32 = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
