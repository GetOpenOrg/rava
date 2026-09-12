#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$LongArraySpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfLong",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_LongArraySpliterator {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[J", access = "private final"))]
    pub array: Field<Vec<i64>>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "estimatedSize", descriptor = "J", access = "private"))]
    pub estimatedSize: Field<i64>,
}

impl Spliterators_LongArraySpliterator {
    // java: <init>([JI)V
    // java: <init>([JI)V
    pub fn new__arr_l_i(array: Vec<i64>, additionalCharacteristics: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), index: Field::new(0), fence: Field::new(0), characteristics: Field::new(0), estimatedSize: Field::new(0) };
        /* invokespecial Method java/util/Spliterators$LongArraySpliterator.<init>:([JIII)V */
        Ok(this)
    }

    // java: <init>([JIII)V
    // java: <init>([JIII)V
    pub fn new__arr_l_i_i_i(array: Vec<i64>, origin: i32, fence: i32, additionalCharacteristics: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), index: Field::new(0), fence: Field::new(0), characteristics: Field::new(0), estimatedSize: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.array.set(array);
        this.index.set(origin);
        this.fence.set(fence);
        this.characteristics.set(((additionalCharacteristics|64i32)|16384i32));
        this.estimatedSize.set(18446744073709551615i64);
        Ok(this)
    }

    // java: <init>([JIIIJ)V
    // java: <init>([JIIIJ)V
    pub fn new__arr_l_i_i_i_l(array: Vec<i64>, origin: i32, fence: i32, characteristics: i32, estimatedSize: i64) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), index: Field::new(0), fence: Field::new(0), characteristics: Field::new(0), estimatedSize: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.array.set(array);
        this.index.set(origin);
        this.fence.set(fence);
        this.characteristics.set((characteristics&-16449i32));
        this.estimatedSize.set(estimatedSize);
        Ok(this)
    }

    // java: trySplit()Ljava/util/Spliterator$OfLong;
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        let mut lo: i32 = this.index.get();
        let mut mid: i32 = (((lo).wrapping_add(this.fence.get()) as u32>>(1i32&0x1f)) as i32);
        /* TODO: aconst_null  */
        return Ok(mid);
        /* TODO: lcmp  */
        this.index.set(mid);
        return Ok(Spliterators_LongArraySpliterator::new(this.array.get(), lo, mid, this.characteristics.get())?);
        /* TODO: lushr  */
        let mut prefixEstimatedSize: i64 = 1i32;
        this.estimatedSize.set((this.estimatedSize.get()).wrapping_sub(prefixEstimatedSize));
        this.index.set(mid);
        Ok(Spliterators_LongArraySpliterator::new(this.array.get(), lo, mid, this.characteristics.get(), prefixEstimatedSize)?)
    }

    // java: forEachRemaining(Ljava/util/function/LongConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut a: Vec<i64> = this.array.get();
        let mut hi: i32 = this.fence.get();
        let mut i: i32 = this.index.get();
        this.index.set(hi);
        action.accept(a[i as usize])?;
        i = i.wrapping_add(1i32);
        Ok(())
    }

    // java: tryAdvance(Ljava/util/function/LongConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.index.set((this.index.get()).wrapping_add(1i32));
        action.accept(this.array.get()[this.index.get() as usize])?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        let this = self;
        /* TODO: lcmp  */
        Ok(((this.fence.get()).wrapping_sub(this.index.get()) as i64))
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        let this = self;
        Ok(this.characteristics.get())
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.hasCharacteristics(4i32)?;
        /* TODO: aconst_null  */
        return Ok(_t0);
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
