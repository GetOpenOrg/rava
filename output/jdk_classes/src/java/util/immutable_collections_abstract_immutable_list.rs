#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$AbstractImmutableList",
    super_class = "java/util/ImmutableCollections$AbstractImmutableCollection",
    interfaces  = "java/util/List,java/util/RandomAccess",
    access      = "abstract",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_AbstractImmutableList<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> ImmutableCollections_AbstractImmutableList<E> {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableCollection.<init>:()V */
        Ok(this)
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: E) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.size()?;
        let mut size: i32 = _t0;
        ImmutableCollections_AbstractImmutableList::subListRangeCheck(fromIndex, toIndex, size)?;
        let _t1: Object = ImmutableCollections_SubList::fromList(this, fromIndex, toIndex)?;
        Ok(_t1)
    }

    // java: subListRangeCheck(III)V
    pub fn subListRangeCheck(fromIndex: i32, toIndex: i32, size: i32) -> Result<()> {
        String::new().append(&String::from("fromIndex ="))?;
        String::new().append(&fromIndex)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        String::new().append(&String::from("toIndex ="))?;
        String::new().append(&toIndex)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        String::new().append(&String::from("fromIndex("))?;
        String::new().append(&fromIndex)?;
        String::new().append(&String::from(") > toIndex("))?;
        String::new().append(&toIndex)?;
        String::new().append(&String::from(")"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.size()?;
        Ok(ImmutableCollections_ListItr::new(this, _t0)?)
    }

    // java: listIterator()Ljava/util/ListIterator;
    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.listIterator(0i32)?;
        Ok(_t0)
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.size()?;
        let mut size: i32 = _t0;
        let _t1 = this.outOfBounds(index)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(ImmutableCollections_ListItr::new(this, size, index)?)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let _t0 = o.iterator()?;
        let mut oit: Object = _t0;
        let mut i: i32 = 0i32;
        let _t1 = this.size()?;
        let mut s: i32 = _t1;
        loop {
            if i >= s { break; }
            let _t0 = oit.hasNext()?;
            let _t1 = this.get(i)?;
            let _t2 = oit.next()?;
            let _t3: bool = Objects::equals(_t1, _t2)?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        let _t2 = oit.hasNext()?;
        Ok(_t2==0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut hash: i32 = 1i32;
        let mut i: i32 = 0i32;
        let _t0 = this.size()?;
        let mut s: i32 = _t0;
        loop {
            if i >= s { break; }
            let _t0 = this.get(i)?;
            let _t1: i32 = Objects::hashCode(_t0)?;
            hash = ((31i32).wrapping_mul(hash)).wrapping_add(_t1);
            i = i.wrapping_add(1i32);
        }
        Ok(hash)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.indexOf(o)?;
        Ok(_t0>=0i32)
    }

    // java: reversed()Ljava/util/List;
    pub fn reversed(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = ReverseOrderListView::of(this, 0i32)?;
        Ok(_t0)
    }

    // java: outOfBounds(I)Ljava/lang/IndexOutOfBoundsException;
    pub fn outOfBounds(&self, index: i32) -> Result<Object> {
        let this = self;
        String::new().append(&String::from("Index:"))?;
        String::new().append(&index)?;
        String::new().append(&String::from("Size:"))?;
        let _t0 = this.size()?;
        String::new().append(&_t0)?;
        Ok(IndexOutOfBoundsException::new(String::new())?)
    }
}
