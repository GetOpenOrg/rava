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
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.<init>:(Ljava/util/ReverseOrderListView;II)V")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.hasNext:()Z")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.next:()Ljava/lang/Object;")
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.hasPrevious:()Z")
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.previous:()Ljava/lang/Object;")
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.nextIndex:()I")
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.previousIndex:()I")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.remove:()V")
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.set:(Ljava/lang/Object;)V")
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView$DescendingListIterator.add:(Ljava/lang/Object;)V")
    }
}
