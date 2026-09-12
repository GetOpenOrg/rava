#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$2",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_2 {
    #[cfg_attr(any(), java_field(name = "est", descriptor = "J"))]
    pub est: Field<i64>,
    #[cfg_attr(any(), java_field(name = "val$element", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub val_element: Field<Object>,
}

impl Collections_2 {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new(arg_0: Object) -> Result<Self> {
        let this = Self { est: Field::new(0), val_element: Field::new(Default::default()) };
        this.val_element.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.est.set(1i64);
        Ok(this)
    }

    // java: trySplit()Ljava/util/Spliterator;
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, consumer: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(consumer)?;
        /* TODO: lcmp  */
        this.est.set((this.est.get()).wrapping_sub(1i64));
        consumer.accept(this.val_element.get())?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, consumer: Object) -> Result<()> {
        let this = self;
        let _t0 = this.tryAdvance(consumer)?;
        Ok(())
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        let this = self;
        Ok(this.est.get())
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        let this = self;
        let mut value: i32 = 0i32;
        Ok((((((value|64i32)|16384i32)|1024i32)|1i32)|16i32))
    }
}
