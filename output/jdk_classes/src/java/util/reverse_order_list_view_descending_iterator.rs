#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ReverseOrderListView$DescendingIterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "ReverseOrderListView.java",
))]
pub struct ReverseOrderListView_DescendingIterator {
    #[cfg_attr(any(), java_field(name = "it", descriptor = "Ljava/util/ListIterator;", access = "final"))]
    pub it: Field<Object>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/ReverseOrderListView;", access = "final"))]
    pub this_0: Field<Object>,
}

impl ReverseOrderListView_DescendingIterator {
    // java: <init>(Ljava/util/ReverseOrderListView;)V
    pub fn new(&self, this_0: Object) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView$DescendingIterator.<init>:(Ljava/util/ReverseOrderListView;)V")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView$DescendingIterator.hasNext:()Z")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView$DescendingIterator.next:()Ljava/lang/Object;")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView$DescendingIterator.remove:()V")
    }
}
