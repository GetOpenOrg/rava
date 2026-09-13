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
    pub fn new(&self, list: Object, type_: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedRandomAccessList.<init>:(Ljava/util/List;Ljava/lang/Class;)V")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedRandomAccessList.subList:(II)Ljava/util/List;")
    }
}
