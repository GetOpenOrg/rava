#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$AbstractImmutableSet",
    super_class = "java/util/ImmutableCollections$AbstractImmutableCollection",
    interfaces  = "java/util/Set",
    access      = "abstract",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_AbstractImmutableSet<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> ImmutableCollections_AbstractImmutableSet<E> {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableCollection.<init>:()V */
        Ok(this)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let mut c: Object = o;
        let _t0 = c.size()?;
        let _t1 = this.size()?;
        return Ok(0i32);
        let _t2 = c.iterator()?;
        let mut local_3: Object = _t2;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut e: Object = _t0;
            let _t1 = this.contains(e)?;
            return Ok(0i32);
        }
        Ok(1i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableSet.hashCode")
    }
}
