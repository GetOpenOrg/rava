#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$IteratorSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "",
    source      = "Spliterators.java",
))]
pub struct Spliterators_IteratorSpliterator<T> {
    #[cfg_attr(any(), java_field(name = "collection", descriptor = "Ljava/util/Collection;", access = "private final"))]
    pub collection: Field<Object>,
    #[cfg_attr(any(), java_field(name = "it", descriptor = "Ljava/util/Iterator;", access = "private"))]
    pub it: Field<Object>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "est", descriptor = "J", access = "private"))]
    pub est: Field<i64>,
    #[cfg_attr(any(), java_field(name = "batch", descriptor = "I", access = "private"))]
    pub batch: Field<i32>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Spliterators_IteratorSpliterator<T> {
    // java: <init>(Ljava/util/Collection;I)V
    // java: <init>(Ljava/util/Collection;I)V
    pub fn new__coll_i(collection: Object, characteristics: i32) -> Result<Self> {
        let this = Self { collection: Field::new(Default::default()), it: Field::new(Default::default()), characteristics: Field::new(0), est: Field::new(0), batch: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.collection.set(collection);
        /* TODO: aconst_null  */
        todo!("stack underflow").it.set(this);
        ((characteristics|64i32)|16384i32).characteristics.set(characteristics);
        Ok(this)
    }

    // java: <init>(Ljava/util/Iterator;JI)V
    // java: <init>(Ljava/util/Iterator;JI)V
    pub fn new__iterat_l_i(iterator: Object, size: i64, arg_2: i32) -> Result<Self> {
        let this = Self { collection: Field::new(Default::default()), it: Field::new(Default::default()), characteristics: Field::new(0), est: Field::new(0), batch: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").collection.set(this);
        this.it.set(iterator);
        this.est.set(size);
        ((local_4|64i32)|16384i32).characteristics.set(local_4);
        Ok(this)
    }

    // java: <init>(Ljava/util/Iterator;I)V
    // java: <init>(Ljava/util/Iterator;I)V
    pub fn new__iterat_i(iterator: Object, characteristics: i32) -> Result<Self> {
        let this = Self { collection: Field::new(Default::default()), it: Field::new(Default::default()), characteristics: Field::new(0), est: Field::new(0), batch: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").collection.set(this);
        this.it.set(iterator);
        this.est.set(9223372036854775807i64);
        this.characteristics.set((characteristics&-16449i32));
        Ok(this)
    }

    // java: trySplit()Ljava/util/Spliterator;
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        let mut i: Object = this.it.get();
        let _t0 = this.collection.get().iterator()?;
        this.it.set(_t0);
        i = _t0;
        let _t1 = this.collection.get().size()?;
        /* TODO: dup2_x1  */
        this.est.set((_t1 as i64));
        let mut s: i64 = this.it.get();
        s = this.est.get();
        /* TODO: lcmp  */
        let _t2 = i.hasNext()?;
        let mut n: i32 = (this.batch.get()).wrapping_add(1024i32);
        /* TODO: lcmp  */
        n = (s as i32);
        n = 33554432i32;
        let mut _arr3: Vec<Object> = Vec::with_capacity(n as usize);
        let mut a: Vec<Object> = _arr3;
        let mut j: i32 = 0i32;
        let _t4 = i.next()?;
        a[j as usize] = _t4;
        j = j.wrapping_add(1i32);
        let _t5 = i.hasNext()?;
        this.batch.set(j);
        /* TODO: lcmp  */
        this.est.set((this.est.get()).wrapping_sub((j as i64)));
        return Ok(Spliterators_ArraySpliterator::new(a, 0i32, j, this.characteristics.get())?);
        return Ok(Spliterators_ArraySpliterator::new(a, 0i32, j, this.characteristics.get(), 4611686018427387903i64)?);
        /* TODO: aconst_null  */
        Ok(9223372036854775807i64)
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut i: Object = this.it.get();
        let _t0 = this.collection.get().iterator()?;
        this.it.set(_t0);
        i = _t0;
        let _t1 = this.collection.get().size()?;
        this.est.set((_t1 as i64));
        i.forEachRemaining(action)?;
        Ok(())
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.collection.get().iterator()?;
        this.it.set(_t0);
        let _t1 = this.collection.get().size()?;
        this.est.set((_t1 as i64));
        let _t2 = this.it.get().hasNext()?;
        let _t3 = this.it.get().next()?;
        action.accept(_t3)?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        let this = self;
        let _t0 = this.collection.get().iterator()?;
        this.it.set(_t0);
        let _t1 = this.collection.get().size()?;
        /* TODO: dup2_x1  */
        this.est.set((_t1 as i64));
        return Ok(this.it.get());
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
