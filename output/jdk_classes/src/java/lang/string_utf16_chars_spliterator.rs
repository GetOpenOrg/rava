#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringUTF16$CharsSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfInt",
    access      = "",
    source      = "StringUTF16.java",
))]
pub struct StringUTF16_CharsSpliterator {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[B", access = "private final"))]
    pub array: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "cs", descriptor = "I", access = "private final"))]
    pub cs: Field<i32>,
}

impl StringUTF16_CharsSpliterator {
    // java: <init>([BI)V
    // java: <init>([BI)V
    pub fn new__arr_b_i(array: Vec<i8>, acs: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), index: Field::new(0), fence: Field::new(0), cs: Field::new(0) };
        /* invokespecial Method java/lang/StringUTF16$CharsSpliterator.<init>:([BIII)V */
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
        this.cs.set((((acs|16i32)|64i32)|16384i32));
        Ok(this)
    }

    // java: trySplit()Ljava/util/Spliterator$OfInt;
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        let mut lo: i32 = this.index.get();
        let mut mid: i32 = (((lo).wrapping_add(this.fence.get()) as u32>>(1i32&0x1f)) as i32);
        /* TODO: aconst_null  */
        this.index.set(mid);
        Ok(StringUTF16_CharsSpliterator::new(this.array.get(), lo, mid, this.cs.get())?)
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut a: Vec<i8> = this.array.get();
        let mut hi: i32 = this.fence.get();
        let mut i: i32 = this.index.get();
        this.index.set(hi);
        let _t0: u16 = StringUTF16::charAt(&a, i)?;
        action.accept(_t0)?;
        i = i.wrapping_add(1i32);
        Ok(())
    }

    // java: tryAdvance(Ljava/util/function/IntConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut i: i32 = this.index.get();
        let _t0: u16 = StringUTF16::charAt(&this.array.get(), i)?;
        action.accept(_t0)?;
        this.index.set((this.index.get()).wrapping_add(1i32));
        return Ok(1i32);
        Ok(0i32)
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
