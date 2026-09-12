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
    pub fn new(this_0: Object) -> Result<Self> {
        let this = Self { it: Field::new(Default::default()), this_0: Field::new(Default::default()) };
        this.this_0.set(this_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0 = this.this_0.get().base.get().size()?;
        let _t1 = this.this_0.get().base.get().listIterator(_t0)?;
        this.it.set(_t1);
        Ok(this)
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.it.get().hasPrevious()?;
        Ok(_t0)
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.it.get().previous()?;
        Ok(_t0)
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        let this = self;
        this.this_0.get().checkModifiable()?;
        this.it.get().remove()?;
        Ok(())
    }
}
