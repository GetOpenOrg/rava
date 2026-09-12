#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$2Adapter",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator$OfInt,java/util/function/IntConsumer",
    access      = "",
    source      = "Spliterators.java",
))]
pub struct Spliterators_2Adapter {
    #[cfg_attr(any(), java_field(name = "valueReady", descriptor = "Z"))]
    pub valueReady: Field<bool>,
    #[cfg_attr(any(), java_field(name = "nextElement", descriptor = "I"))]
    pub nextElement: Field<i32>,
    #[cfg_attr(any(), java_field(name = "val$spliterator", descriptor = "Ljava/util/Spliterator$OfInt;", access = "final"))]
    pub val_spliterator: Field<Object>,
}

impl Spliterators_2Adapter {
    // java: <init>(Ljava/util/Spliterator$OfInt;)V
    pub fn new(arg_0: Object) -> Result<Self> {
        let this = Self { valueReady: Field::new(false), nextElement: Field::new(0), val_spliterator: Field::new(Default::default()) };
        this.val_spliterator.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.valueReady.set(0i32);
        Ok(this)
    }

    // java: accept(I)V
    pub fn accept(&self, t: i32) -> Result<()> {
        let this = self;
        this.valueReady.set(1i32);
        this.nextElement.set(t);
        Ok(())
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.val_spliterator.get().tryAdvance(this)?;
        Ok(this.valueReady.get())
    }

    // java: nextInt()I
    pub fn nextInt(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.hasNext()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.valueReady.set(0i32);
        Ok(this.nextElement.get())
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        this.valueReady.set(0i32);
        action.accept(this.nextElement.get())?;
        this.val_spliterator.get().forEachRemaining(action)?;
        Ok(())
    }
}
