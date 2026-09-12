#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringUTF16$CodePointsSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfInt",
    access      = "",
    source      = "StringUTF16.java",
))]
pub struct StringUTF16_CodePointsSpliterator {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[B", access = "private final"))]
    pub array: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "cs", descriptor = "I", access = "private final"))]
    pub cs: Field<i32>,
}

impl StringUTF16_CodePointsSpliterator {
    // java: <init>([BI)V
    // java: <init>([BI)V
    pub fn new__arr_b_i(array: Vec<i8>, acs: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), index: Field::new(0), fence: Field::new(0), cs: Field::new(0) };
        /* invokespecial Method java/lang/StringUTF16$CodePointsSpliterator.<init>:([BIII)V */
        Ok(this)
    }

    // java: <init>([BIII)V
    // java: <init>([BIII)V
    pub fn new__arr_b_i_i_i(array: Vec<i8>, origin: i32, fence: i32, acs: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), index: Field::new(0), fence: Field::new(0), cs: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.array.set(array);
        this.index.set(origin);
        this.fence.set(fence);
        this.cs.set((acs|16i32));
        Ok(this)
    }

    // java: trySplit()Ljava/util/Spliterator$OfInt;
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        let mut lo: i32 = this.index.get();
        let mut mid: i32 = (((lo).wrapping_add(this.fence.get()) as u32>>(1i32&0x1f)) as i32);
        /* TODO: aconst_null  */
        return Ok(mid);
        let _t0: u16 = StringUTF16::charAt(&this.array.get(), mid)?;
        let _t1: bool = Character::isLowSurrogate(_t0)?;
        let mut midOneLess: i32 = (mid).wrapping_sub(1i32);
        let _t2: u16 = StringUTF16::charAt(&this.array.get(), (mid).wrapping_sub(1i32))?;
        let _t3: bool = Character::isHighSurrogate(_t2)?;
        /* TODO: aconst_null  */
        return Ok(midOneLess);
        this.index.set(midOneLess);
        return Ok(StringUTF16_CodePointsSpliterator::new(this.array.get(), lo, midOneLess, this.cs.get())?);
        this.index.set(mid);
        Ok(StringUTF16_CodePointsSpliterator::new(this.array.get(), lo, mid, this.cs.get())?)
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut a: Vec<i8> = this.array.get();
        let mut hi: i32 = this.fence.get();
        let mut i: i32 = this.index.get();
        this.index.set(hi);
        let _t0: i32 = StringUTF16_CodePointsSpliterator::advance(&a, i, hi, action)?;
        i = _t0;
        Ok(())
    }

    // java: tryAdvance(Ljava/util/function/IntConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i32 = StringUTF16_CodePointsSpliterator::advance(&this.array.get(), this.index.get(), this.fence.get(), action)?;
        this.index.set(_t0);
        return Ok(1i32);
        Ok(0i32)
    }

    // java: advance([BIILjava/util/function/IntConsumer;)I
    pub fn advance(a: &[i8], i: i32, hi: i32, action: Object) -> Result<i32> {
        i = i.wrapping_add(1i32);
        let _t0: u16 = StringUTF16::charAt(&a, i)?;
        let mut c1: i32 = _t0;
        let mut cp: i32 = c1;
        let _t1: bool = Character::isHighSurrogate(c1)?;
        let _t2: u16 = StringUTF16::charAt(&a, i)?;
        let mut c2: i32 = _t2;
        let _t3: bool = Character::isLowSurrogate(c2)?;
        i = i.wrapping_add(1i32);
        let _t4: i32 = Character::toCodePoint(c1, c2)?;
        cp = _t4;
        action.accept(cp)?;
        Ok(i)
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        let this = self;
        Ok(((this.fence.get()).wrapping_sub(this.index.get()) as i64))
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        let this = self;
        Ok(this.cs.get())
    }
}
