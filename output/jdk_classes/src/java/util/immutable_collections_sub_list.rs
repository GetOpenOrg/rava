#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$SubList",
    super_class = "java/util/ImmutableCollections$AbstractImmutableList",
    interfaces  = "java/util/RandomAccess",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_SubList<E> {
    #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/ImmutableCollections$AbstractImmutableList;", access = "private final"))]
    pub root: Field<Object>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private final"))]
    pub size: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_SubList<E> {
    // java: <init>(Ljava/util/ImmutableCollections$AbstractImmutableList;II)V
    pub fn new(root: Object, offset: i32, size: i32) -> Result<Self> {
        let this = Self { root: Field::new(Default::default()), offset: Field::new(0), size: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableList.<init>:()V */
        return Err(JvmError::Custom("athrow".to_owned()));
        this.root.set(root);
        this.offset.set(offset);
        this.size.set(size);
        Ok(this)
    }

    // java: fromSubList(Ljava/util/ImmutableCollections$SubList;II)Ljava/util/ImmutableCollections$SubList;
    pub fn fromSubList(parent: Object, fromIndex: i32, toIndex: i32) -> Result<Object> {
        Ok(ImmutableCollections_SubList::new(parent.root.get(), (parent.offset.get()).wrapping_add(fromIndex), (toIndex).wrapping_sub(fromIndex))?)
    }

    // java: fromList(Ljava/util/ImmutableCollections$AbstractImmutableList;II)Ljava/util/ImmutableCollections$SubList;
    pub fn fromList(list: Object, fromIndex: i32, toIndex: i32) -> Result<Object> {
        Ok(ImmutableCollections_SubList::new(list, fromIndex, (toIndex).wrapping_sub(fromIndex))?)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex__i_i(index, this.size.get())?;
        let _t1 = this.root.get().get((this.offset.get()).wrapping_add(index))?;
        Ok(_t1)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.size()?;
        Ok(ImmutableCollections_ListItr::new(this, _t0)?)
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator(&self, index: i32) -> Result<Object> {
        let this = self;
        this.rangeCheck(index)?;
        let _t0 = this.size()?;
        Ok(ImmutableCollections_ListItr::new(this, _t0, index)?)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        ImmutableCollections_SubList::subListRangeCheck(fromIndex, toIndex, this.size.get())?;
        let _t0: Object = ImmutableCollections_SubList::fromSubList(this, fromIndex, toIndex)?;
        Ok(_t0)
    }

    // java: rangeCheck(I)V
    pub fn rangeCheck(&self, index: i32) -> Result<()> {
        let this = self;
        let _t0 = this.outOfBounds(index)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: allowNulls()Z
    pub fn allowNulls(&self) -> Result<bool> {
        let this = self;
        Ok(this.root.get().allowNulls.get()!=0i32)
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.allowNulls()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut i: i32 = 0i32;
        let _t1 = this.size()?;
        let mut s: i32 = _t1;
        loop {
            if i >= s { break; }
            let _t0 = this.get(i)?;
            let _t1: bool = Objects::equals(o, _t0)?;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.allowNulls()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.size()?;
        let mut i: i32 = (_t1).wrapping_sub(1i32);
        loop {
            if i<0i32 { break; }
            let _t0 = this.get(i)?;
            let _t1: bool = Objects::equals(o, _t0)?;
            return Ok(i);
            i = i.wrapping_sub(1i32);
        }
        Ok(-1i32)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(this.size.get() as usize);
        let mut array: Vec<Object> = _arr0;
        let mut i: i32 = 0i32;
        loop {
            if i >= this.size.get() { break; }
            let _t0 = this.get(i)?;
            array[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(array)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = a.getClass()?;
        let _t1 = _t0.getComponentType()?;
        let _t2: Object = Array::newInstance(_t1, this.size.get())?;
        let mut array: Object = _t2;
        let mut i: i32 = 0i32;
        loop {
            if i >= this.size.get() { break; }
            let _t0 = this.get(i)?;
            array[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        /* TODO: aconst_null  */
        this.size.get()[array as usize] = this.size.get();
        Ok(array)
    }
}
