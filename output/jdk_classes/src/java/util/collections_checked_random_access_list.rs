#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedRandomAccessList",
    super_class = "java/util/Collections$CheckedList",
    interfaces  = "java/util/RandomAccess",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedRandomAccessList<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_CheckedRandomAccessList<E> {
    // java: <init>(Ljava/util/List;Ljava/lang/Class;)V
    pub fn new(list: Object, type_: Object) -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/Collections$CheckedList.<init>:(Ljava/util/List;Ljava/lang/Class;)V */
        Ok(this)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.list.get().subList(fromIndex, toIndex)?;
        Ok(Collections_CheckedRandomAccessList::new(_t0, this.type_.get())?)
    }
}
