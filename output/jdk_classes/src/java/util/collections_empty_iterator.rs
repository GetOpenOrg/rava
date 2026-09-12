#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$EmptyIterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_EmptyIterator<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_EmptyIterator<E> {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        Ok(())
    }
}
