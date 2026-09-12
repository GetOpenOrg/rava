#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/CharSequence$1CharIterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator$OfInt",
    access      = "",
    source      = "CharSequence.java",
))]
pub struct CharSequence_1CharIterator {
    #[cfg_attr(any(), java_field(name = "cur", descriptor = "I"))]
    pub cur: Field<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/lang/CharSequence;", access = "final"))]
    pub this_0: Field<Object>,
}

impl CharSequence_1CharIterator {
    // java: <init>(Ljava/lang/CharSequence;)V
    pub fn new(this_0: Object) -> Result<Self> {
        let this = Self { cur: Field::new(0), this_0: Field::new(Default::default()) };
        this.this_0.set(this_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cur.set(0i32);
        Ok(this)
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.this_0.get().length()?;
        Ok(this.cur.get() < _t0)
    }

    // java: nextInt()I
    pub fn nextInt(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.hasNext()?;
        this.cur.set((this.cur.get()).wrapping_add(1i32));
        let _t1 = this.this_0.get().charAt(this.cur.get())?;
        return Ok(_t1);
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining(&self, block: Object) -> Result<()> {
        let this = self;
        loop {
            let _t0 = this.this_0.get().length()?;
            if this.cur.get() >= _t0 { break; }
            let _t0 = this.this_0.get().charAt(this.cur.get())?;
            block.accept(_t0)?;
            this.cur.set((this.cur.get()).wrapping_add(1i32));
        }
        Ok(())
    }
}
