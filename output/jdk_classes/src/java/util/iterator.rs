#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Iterator",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Iterator.java",
))]
pub struct Iterator<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Iterator<E> {
    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/Iterator.hasNext")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        todo!("abstract java/util/Iterator.next")
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
        loop {
            let _t0 = this.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = this.next()?;
            action.accept(_t0)?;
        }
        Ok(())
    }
}
