#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$EmptyEnumeration",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Enumeration",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_EmptyEnumeration<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_EmptyEnumeration<E> {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: hasMoreElements()Z
    pub fn hasMoreElements(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    // java: nextElement()Ljava/lang/Object;
    pub fn nextElement(&self) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: asIterator()Ljava/util/Iterator;
    pub fn asIterator(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Collections::emptyIterator()?;
        Ok(_t0)
    }
}
