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
    pub fn new(this_0: Object, size: i32, pos: i32) -> Result<Self> {
        let this = Self { it: Field::new(Default::default()), this_0: Field::new(Default::default()) };
        this.this_0.set(this_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this_0.base.get().listIterator((size).wrapping_sub(pos))?;
        this.it.set(_t0);
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

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.it.get().hasNext()?;
        Ok(_t0)
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.it.get().next()?;
        Ok(_t0)
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.this_0.get().base.get().size()?;
        let _t1 = this.it.get().nextIndex()?;
        Ok((_t0).wrapping_sub(_t1))
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.nextIndex()?;
        Ok((_t0).wrapping_sub(1i32))
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        let this = self;
        this.this_0.get().checkModifiable()?;
        this.it.get().remove()?;
        Ok(())
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: Object) -> Result<()> {
        let this = self;
        this.this_0.get().checkModifiable()?;
        this.it.get().set(e)?;
        Ok(())
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: Object) -> Result<()> {
        let this = self;
        this.this_0.get().checkModifiable()?;
        this.it.get().add(e)?;
        let _t0 = this.it.get().previous()?;
        Ok(())
    }
}
