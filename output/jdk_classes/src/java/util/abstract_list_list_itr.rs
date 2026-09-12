#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$ListItr",
    super_class = "java/util/AbstractList$Itr",
    interfaces  = "java/util/ListIterator",
    access      = "",
    source      = "AbstractList.java",
))]
pub struct AbstractList_ListItr {
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/AbstractList;", access = "final"))]
    pub this_0: Field<Object>,
}

impl AbstractList_ListItr {
    // java: <init>(Ljava/util/AbstractList;I)V
    pub fn new(&self, arg0: Object, index: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList$ListItr.<init>")
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        todo!("abstract java/util/AbstractList$ListItr.hasPrevious")
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        todo!("abstract java/util/AbstractList$ListItr.previous")
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        todo!("abstract java/util/AbstractList$ListItr.nextIndex")
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        todo!("abstract java/util/AbstractList$ListItr.previousIndex")
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/AbstractList$ListItr.set")
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/AbstractList$ListItr.add")
    }
}
