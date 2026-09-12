#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayList$ArrayListSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "final",
    source      = "ArrayList.java",
))]
pub struct ArrayList_ArrayListSpliterator {
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I", access = "private"))]
    pub expectedModCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/ArrayList;", access = "final"))]
    pub this_0: Field<Object>,
}

impl ArrayList_ArrayListSpliterator {
    // java: <init>(Ljava/util/ArrayList;III)V
    pub fn new(this_0: Object, origin: i32, fence: i32, expectedModCount: i32) -> Result<Self> {
        let this = Self { index: Field::new(0), fence: Field::new(0), expectedModCount: Field::new(0), this_0: Field::new(Default::default()) };
        this.this_0.set(this_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.index.set(origin);
        this.fence.set(fence);
        this.expectedModCount.set(expectedModCount);
        Ok(this)
    }

    // java: getFence()I
    pub fn getFence(&self) -> Result<i32> {
        let this = self;
        let mut hi: i32 = this.fence.get();
        this.expectedModCount.set(this.this_0.get().modCount.get());
        this.fence.set(this.this_0.get().size.get());
        hi = this.this_0.get().size.get();
        Ok(hi)
    }

    // java: trySplit()Ljava/util/ArrayList$ArrayListSpliterator;
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getFence()?;
        let mut hi: i32 = _t0;
        let mut lo: i32 = this.index.get();
        let mut mid: i32 = (((lo).wrapping_add(hi) as u32>>(1i32&0x1f)) as i32);
        /* TODO: aconst_null  */
        this.index.set(mid);
        Ok(ArrayList_ArrayListSpliterator::new(this.this_0.get(), lo, mid, this.expectedModCount.get())?)
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.getFence()?;
        let mut hi: i32 = _t0;
        let mut i: i32 = this.index.get();
        this.index.set((i).wrapping_add(1i32));
        let mut e: Object = this.this_0.get().elementData.get()[i as usize].clone();
        action.accept(e)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(1i32);
        Ok(0i32)
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut a: Vec<Object> = this.this_0.get().elementData.get();
        let mut hi: i32 = this.fence.get();
        let mut mc: i32 = this.this_0.get().modCount.get();
        hi = this.this_0.get().size.get();
        mc = this.expectedModCount.get();
        let mut i: i32 = this.index.get();
        this.index.set(hi);
        loop {
            if i >= hi { break; }
            let mut e: Object = a[i as usize].clone();
            action.accept(e)?;
            i = i.wrapping_add(1i32);
        }
        return Ok(());
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        let this = self;
        let _t0 = this.getFence()?;
        Ok(((_t0).wrapping_sub(this.index.get()) as i64))
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        let this = self;
        Ok(16464i32)
    }
}
