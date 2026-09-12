#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedSet",
    super_class = "java/util/Collections$CheckedCollection",
    interfaces  = "java/util/Set,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedSet<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_CheckedSet<E> {
    // java: <init>(Ljava/util/Set;Ljava/lang/Class;)V
    pub fn new(s: Object, elementType: Object) -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/Collections$CheckedCollection.<init>:(Ljava/util/Collection;Ljava/lang/Class;)V */
        Ok(this)
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
}
