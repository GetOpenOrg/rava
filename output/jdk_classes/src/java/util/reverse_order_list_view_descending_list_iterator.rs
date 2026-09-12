#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ReverseOrderListView$DescendingListIterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/ListIterator",
    access      = "",
    source      = "ReverseOrderListView.java",
))]
pub struct ReverseOrderListView_DescendingListIterator {
    #[cfg_attr(any(), java_field(name = "it", descriptor = "Ljava/util/ListIterator;", access = "final"))]
    pub it: Field<Object>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/ReverseOrderListView;", access = "final"))]
    pub this_0: Field<Object>,
}

impl ReverseOrderListView_DescendingListIterator {
    // java: <init>(Ljava/util/ReverseOrderListView;II)V
    pub fn new(&self, this_0: Object, size: i32, pos: i32) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.<init>")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.hasNext")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.next")
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.hasPrevious")
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.previous")
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.nextIndex")
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.previousIndex")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.remove")
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.set")
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView$DescendingListIterator.add")
    }
}
