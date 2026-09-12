#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/CharSequence$1CodePointIterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator$OfInt",
    access      = "",
    source      = "CharSequence.java",
))]
pub struct CharSequence_1CodePointIterator {
    #[cfg_attr(any(), java_field(name = "cur", descriptor = "I"))]
    pub cur: Field<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/lang/CharSequence;", access = "final"))]
    pub this_0: Field<Object>,
}

impl CharSequence_1CodePointIterator {
    // java: <init>(Ljava/lang/CharSequence;)V
    pub fn new(this_0: Object) -> Result<Self> {
        let this = Self { cur: Field::new(0), this_0: Field::new(Default::default()) };
        this.this_0.set(this_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cur.set(0i32);
        Ok(this)
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining(&self, block: Object) -> Result<()> {
        let this = self;
        let _t0 = this.this_0.get().length()?;
        let mut length: i32 = _t0;
        let mut i: i32 = this.cur.get();
        loop {
            if i >= length { break; }
            i = i.wrapping_add(1i32);
            let _t0 = this.this_0.get().charAt(i)?;
            let mut c1: i32 = _t0;
            let _t1: bool = Character::isHighSurrogate(c1)?;
            block.accept(c1)?;
            let _t2 = this.this_0.get().charAt(i)?;
            let mut c2: i32 = _t2;
            let _t3: bool = Character::isLowSurrogate(c2)?;
            i = i.wrapping_add(1i32);
            let _t4: i32 = Character::toCodePoint(c1, c2)?;
            block.accept(_t4)?;
            block.accept(c1)?;
        }
        this.cur.set(i);
        let mut local_6: i32 = todo!("stack underflow");
        this.cur.set(i);
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
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
        let _t0 = this.this_0.get().length()?;
        let mut length: i32 = _t0;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.cur.set((this.cur.get()).wrapping_add(1i32));
        let _t1 = this.this_0.get().charAt(this.cur.get())?;
        let mut c1: i32 = _t1;
        let _t2: bool = Character::isHighSurrogate(c1)?;
        let _t3 = this.this_0.get().charAt(this.cur.get())?;
        let mut c2: i32 = _t3;
        let _t4: bool = Character::isLowSurrogate(c2)?;
        this.cur.set((this.cur.get()).wrapping_add(1i32));
        let _t5: i32 = Character::toCodePoint(c1, c2)?;
        return Ok(_t5);
        Ok(c1)
    }
}
