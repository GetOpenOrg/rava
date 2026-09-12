#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$1Adapter",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator,java/util/function/Consumer",
    access      = "",
    source      = "Spliterators.java",
))]
pub struct Spliterators_1Adapter {
    #[cfg_attr(any(), java_field(name = "valueReady", descriptor = "Z"))]
    pub valueReady: Field<bool>,
    #[cfg_attr(any(), java_field(name = "nextElement", descriptor = "Ljava/lang/Object;"))]
    pub nextElement: Field<Object>,
    #[cfg_attr(any(), java_field(name = "val$spliterator", descriptor = "Ljava/util/Spliterator;", access = "final"))]
    pub val_spliterator: Field<Object>,
}

impl Spliterators_1Adapter {
    // java: <init>(Ljava/util/Spliterator;)V
    pub fn new(arg_0: Object) -> Result<Self> {
        let this = Self { valueReady: Field::new(false), nextElement: Field::new(Default::default()), val_spliterator: Field::new(Default::default()) };
        this.val_spliterator.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.valueReady.set(0i32);
        Ok(this)
    }

    // java: accept(Ljava/lang/Object;)V
    pub fn accept(&self, t: Object) -> Result<()> {
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

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.hasNext()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.valueReady.set(0i32);
        let mut t: Object = this.nextElement.get();
        /* TODO: aconst_null  */
        _t0.nextElement.set(this);
        Ok(t)
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        this.valueReady.set(0i32);
        let mut t: Object = this.nextElement.get();
        /* TODO: aconst_null  */
        this.valueReady.get().nextElement.set(this);
        action.accept(t)?;
        this.val_spliterator.get().forEachRemaining(action)?;
        Ok(())
    }
}
