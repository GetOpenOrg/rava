#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$3",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Enumeration",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_3 {
    #[cfg_attr(any(), java_field(name = "i", descriptor = "Ljava/util/Iterator;", access = "private final"))]
    pub i: Field<Object>,
    #[cfg_attr(any(), java_field(name = "val$c", descriptor = "Ljava/util/Collection;", access = "final"))]
    pub val_c: Field<Object>,
}

impl Collections_3 {
    // java: <init>(Ljava/util/Collection;)V
    pub fn new(arg_0: Object) -> Result<Self> {
        let this = Self { i: Field::new(Default::default()), val_c: Field::new(Default::default()) };
        this.val_c.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0 = this.val_c.get().iterator()?;
        this.i.set(_t0);
        Ok(this)
    }

    // java: hasMoreElements()Z
    pub fn hasMoreElements(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.i.get().hasNext()?;
        Ok(_t0)
    }

    // java: nextElement()Ljava/lang/Object;
    pub fn nextElement(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.i.get().next()?;
        Ok(_t0)
    }
}
