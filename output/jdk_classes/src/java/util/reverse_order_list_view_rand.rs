#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ReverseOrderListView$Rand",
    super_class = "java/util/ReverseOrderListView",
    interfaces  = "java/util/RandomAccess",
    access      = "",
    source      = "ReverseOrderListView.java",
))]
pub struct ReverseOrderListView_Rand<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> ReverseOrderListView_Rand<E> {
    // java: <init>(Ljava/util/List;Z)V
    pub fn new(list: Object, modifiable: bool) -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/ReverseOrderListView.<init>:(Ljava/util/List;Z)V */
        Ok(this)
    }
}
