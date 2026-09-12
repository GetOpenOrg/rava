#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$LongIteratorSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfLong",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_LongIteratorSpliterator {
    #[cfg_attr(any(), java_field(name = "it", descriptor = "Ljava/util/PrimitiveIterator$OfLong;", access = "private final"))]
    pub it: Field<Object>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "est", descriptor = "J", access = "private"))]
    pub est: Field<i64>,
    #[cfg_attr(any(), java_field(name = "batch", descriptor = "I", access = "private"))]
    pub batch: Field<i32>,
}

impl Spliterators_LongIteratorSpliterator {
    // java: <init>(Ljava/util/PrimitiveIterator$OfLong;JI)V
    // java: <init>(Ljava/util/PrimitiveIterator$OfLong;JI)V
    pub fn new__primit_l_i(iterator: Object, size: i64, arg_2: i32) -> Result<Self> {
        let this = Self { it: Field::new(Default::default()), characteristics: Field::new(0), est: Field::new(0), batch: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.it.set(iterator);
        this.est.set(size);
        ((local_4|64i32)|16384i32).characteristics.set(local_4);
        Ok(this)
    }

    // java: <init>(Ljava/util/PrimitiveIterator$OfLong;I)V
    // java: <init>(Ljava/util/PrimitiveIterator$OfLong;I)V
    pub fn new__primit_i(iterator: Object, characteristics: i32) -> Result<Self> {
        let this = Self { it: Field::new(Default::default()), characteristics: Field::new(0), est: Field::new(0), batch: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.it.set(iterator);
        this.est.set(9223372036854775807i64);
        this.characteristics.set((characteristics&-16449i32));
        Ok(this)
    }

    // java: trySplit()Ljava/util/Spliterator$OfLong;
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        let mut i: Object = this.it.get();
        let mut s: i64 = this.est.get();
        /* TODO: lcmp  */
        let _t0 = i.hasNext()?;
        let mut n: i32 = (this.batch.get()).wrapping_add(1024i32);
        /* TODO: lcmp  */
        n = (s as i32);
        n = 33554432i32;
        let mut _arr1: Vec<i64> = vec![0i64; n as usize];
        let mut a: Vec<i64> = _arr1;
        let mut j: i32 = 0i32;
        let _t2 = i.nextLong()?;
        a[j as usize] = _t2;
        j = j.wrapping_add(1i32);
        let _t3 = i.hasNext()?;
        this.batch.set(j);
        /* TODO: lcmp  */
        this.est.set((this.est.get()).wrapping_sub((j as i64)));
        return Ok(Spliterators_LongArraySpliterator::new(a, 0i32, j, this.characteristics.get())?);
        return Ok(Spliterators_LongArraySpliterator::new(a, 0i32, j, this.characteristics.get(), 4611686018427387903i64)?);
        /* TODO: aconst_null  */
        Ok(9223372036854775807i64)
    }

    // java: forEachRemaining(Ljava/util/function/LongConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.it.get().forEachRemaining(action)?;
        Ok(())
    }

    // java: tryAdvance(Ljava/util/function/LongConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.it.get().hasNext()?;
        let _t1 = this.it.get().nextLong()?;
        action.accept(_t1)?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        let this = self;
        Ok(this.est.get())
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
