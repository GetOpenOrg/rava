#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayList$SubList$2",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "",
    source      = "ArrayList.java",
))]
pub struct ArrayList_SubList_2 {
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I", access = "private"))]
    pub expectedModCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/ArrayList$SubList;", access = "final"))]
    pub this$0: Field<Object>,
}

impl ArrayList_SubList_2 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/ArrayList$SubList;)V"))]
    pub fn new(this_0: Object) -> Result<Self> {
        let this = Self { index: Field::new(0), fence: Field::new(0), expectedModCount: Field::new(0), this$0: Field::new(Default::default()) };
        this.this$0.set(this_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.index.set(this.this$0.get().offset.get());
        this.fence.set(-1i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getFence", descriptor = "()I", access = "private"))]
    pub fn getFence(&self) -> Result<i32> {
        let this = self;
        let mut hi: i32 = this.fence.get();
        this.expectedModCount.set(this.this$0.get().modCount.get());
        this.fence.set((this.this$0.get().offset.get()).wrapping_add(this.this$0.get().size.get()));
        hi = (this.this$0.get().offset.get()).wrapping_add(this.this$0.get().size.get());
        Ok(hi)
    }

    #[cfg_attr(any(), java_method(name = "trySplit", descriptor = "()Ljava/util/ArrayList$ArrayListSpliterator;", access = "public"))]
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getFence()?;
        let mut hi: i32 = _t0;
        let mut lo: i32 = this.index.get();
        let mut mid: i32 = (((lo).wrapping_add(hi) as u32>>(1i32&0x1f)) as i32);
        /* TODO: aconst_null  */
        let _t1: Object = Objects::requireNonNull(this.this$0.get().root.get())?;
        this.index.set(mid);
        Ok(ArrayList_ArrayListSpliterator::new(this.this$0.get().root.get(), lo, mid, this.expectedModCount.get())?)
    }

    #[cfg_attr(any(), java_method(name = "tryAdvance", descriptor = "(Ljava/util/function/Consumer;)Z", access = "public"))]
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let _t1 = this.getFence()?;
        let mut hi: i32 = _t1;
        let mut i: i32 = this.index.get();
        this.index.set((i).wrapping_add(1i32));
        let mut e: Object = this.this$0.get().root.get().elementData.get()[i as usize].clone();
        action.accept(e)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(1i32);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public"))]
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let mut lst: Object = this.this$0.get().root.get();
        let mut a: Vec<Object> = lst.elementData.get();
        let mut hi: i32 = this.fence.get();
        let mut mc: i32 = this.this$0.get().modCount.get();
        hi = (this.this$0.get().offset.get()).wrapping_add(this.this$0.get().size.get());
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
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "estimateSize", descriptor = "()J", access = "public"))]
    pub fn estimateSize(&self) -> Result<i64> {
        let this = self;
        let _t0 = this.getFence()?;
        Ok(((_t0).wrapping_sub(this.index.get()) as i64))
    }

    #[cfg_attr(any(), java_method(name = "characteristics", descriptor = "()I", access = "public"))]
    pub fn characteristics(&self) -> Result<i32> {
        let this = self;
        Ok(16464i32)
    }
}
