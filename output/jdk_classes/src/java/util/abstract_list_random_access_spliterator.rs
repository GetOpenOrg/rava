#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$RandomAccessSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "final",
    source      = "AbstractList.java",
))]
pub struct AbstractList_RandomAccessSpliterator<E> {
    #[cfg_attr(any(), java_field(name = "list", descriptor = "Ljava/util/List;", access = "private final"))]
    pub list: Field<Object>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "alist", descriptor = "Ljava/util/AbstractList;", access = "private final"))]
    pub alist: Field<Object>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I", access = "private"))]
    pub expectedModCount: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> AbstractList_RandomAccessSpliterator<E> {
    // java: <init>(Ljava/util/List;)V
    // java: <init>(Ljava/util/List;)V
    pub fn new__list(list: Object) -> Result<Self> {
        let this = Self { list: Field::new(Default::default()), index: Field::new(0), fence: Field::new(0), alist: Field::new(Default::default()), expectedModCount: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        return Err(JvmError::Custom("athrow".to_owned()));
        this.list.set(list);
        this.index.set(0i32);
        this.fence.set(-1i32);
        /* TODO: aconst_null  */
        true.alist.set(list);
        this.alist.get().modCount.get().expectedModCount.set(0i32);
        Ok(this)
    }

    // java: <init>(Ljava/util/AbstractList$RandomAccessSpliterator;II)V
    // java: <init>(Ljava/util/AbstractList$RandomAccessSpliterator;II)V
    pub fn new__abstra_i_i(parent: Object, origin: i32, fence: i32) -> Result<Self> {
        let this = Self { list: Field::new(Default::default()), index: Field::new(0), fence: Field::new(0), alist: Field::new(Default::default()), expectedModCount: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.list.set(parent.list.get());
        this.index.set(origin);
        this.fence.set(fence);
        this.alist.set(parent.alist.get());
        this.expectedModCount.set(parent.expectedModCount.get());
        Ok(this)
    }

    // java: getFence()I
    pub fn getFence(&self) -> Result<i32> {
        let this = self;
        let mut lst: Object = this.list.get();
        let mut hi: i32 = this.fence.get();
        this.expectedModCount.set(this.alist.get().modCount.get());
        let _t0 = lst.size()?;
        this.fence.set(_t0);
        hi = _t0;
        Ok(hi)
    }

    // java: trySplit()Ljava/util/Spliterator;
    pub fn trySplit(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getFence()?;
        let mut hi: i32 = _t0;
        let mut lo: i32 = this.index.get();
        let mut mid: i32 = (((lo).wrapping_add(hi) as u32>>(1i32&0x1f)) as i32);
        /* TODO: aconst_null  */
        this.index.set(mid);
        Ok(AbstractList_RandomAccessSpliterator::new(this, lo, mid)?)
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.getFence()?;
        let mut hi: i32 = _t0;
        let mut i: i32 = this.index.get();
        this.index.set((i).wrapping_add(1i32));
        let _t1: Object = AbstractList_RandomAccessSpliterator::get(this.list.get(), i)?;
        action.accept(_t1)?;
        AbstractList_RandomAccessSpliterator::checkAbstractListModCount(this.alist.get(), this.expectedModCount.get())?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        let mut lst: Object = this.list.get();
        let _t1 = this.getFence()?;
        let mut hi: i32 = _t1;
        let mut i: i32 = this.index.get();
        this.index.set(hi);
        loop {
            if i >= hi { break; }
            let _t0: Object = AbstractList_RandomAccessSpliterator::get(lst, i)?;
            action.accept(_t0)?;
            i = i.wrapping_add(1i32);
        }
        AbstractList_RandomAccessSpliterator::checkAbstractListModCount(this.alist.get(), this.expectedModCount.get())?;
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

    // java: get(Ljava/util/List;I)Ljava/lang/Object;
    pub fn get(list: Object, i: i32) -> Result<E> {
        let _t0 = list.get(i)?;
        return Ok(_t0);
        let mut ex: i32 = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: checkAbstractListModCount(Ljava/util/AbstractList;I)V
    pub fn checkAbstractListModCount(alist: Object, expectedModCount: i32) -> Result<()> {
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
