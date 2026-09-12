#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$1",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_1 {
    #[cfg_attr(any(), java_field(name = "hasNext", descriptor = "Z", access = "private"))]
    pub hasNext: Field<bool>,
    #[cfg_attr(any(), java_field(name = "val$e", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub val_e: Field<Object>,
}

impl Collections_1 {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new(arg_0: Object) -> Result<Self> {
        let this = Self { hasNext: Field::new(false), val_e: Field::new(Default::default()) };
        this.val_e.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.hasNext.set(1i32);
        Ok(this)
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        Ok(this.hasNext.get())
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        let this = self;
        this.hasNext.set(0i32);
        return Ok(this.val_e.get());
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        this.hasNext.set(0i32);
        action.accept(this.val_e.get())?;
        Ok(())
    }
}
