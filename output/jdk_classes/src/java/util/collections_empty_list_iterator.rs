#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$EmptyListIterator",
    super_class = "java/util/Collections$EmptyIterator",
    interfaces  = "java/util/ListIterator",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_EmptyListIterator<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_EmptyListIterator<E> {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/Collections$EmptyIterator.<init>:()V */
        Ok(this)
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        let this = self;
        Ok(0i32)
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        let this = self;
        Ok(-1i32)
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: E) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: E) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
