#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList",
    super_class = "java/util/AbstractCollection",
    interfaces  = "java/util/List",
    access      = "public abstract",
    source      = "AbstractList.java",
))]
pub struct AbstractList<E> {
    #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = "protected"))]
    pub modCount: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> AbstractList<E> {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { modCount: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractCollection.<init>:()V */
        this.modCount.set(0i32);
        Ok(this)
    }

    // java: add(Ljava/lang/Object;)Z
    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, e: E) -> Result<bool> {
        let this = self;
        let _t0 = this.size()?;
        this.add__i_obj(_t0, e)?;
        Ok(1i32)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList.get")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: add(ILjava/lang/Object;)V
    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, index: i32, element: E) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.listIterator()?;
        let mut it: Object = _t0;
        let _t1 = it.hasNext()?;
        let _t2 = it.next()?;
        let _t3 = it.previousIndex()?;
        return Ok(_t3);
        let _t4 = it.hasNext()?;
        let _t5 = it.next()?;
        let _t6 = o.equals(_t5)?;
        let _t7 = it.previousIndex()?;
        return Ok(_t7);
        Ok(-1i32)
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.size()?;
        let _t1 = this.listIterator__i(_t0)?;
        let mut it: Object = _t1;
        let _t2 = it.hasPrevious()?;
        let _t3 = it.previous()?;
        let _t4 = it.nextIndex()?;
        return Ok(_t4);
        let _t5 = it.hasPrevious()?;
        let _t6 = it.previous()?;
        let _t7 = o.equals(_t6)?;
        let _t8 = it.nextIndex()?;
        return Ok(_t8);
        Ok(-1i32)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        let _t0 = this.size()?;
        this.removeRange(0i32, _t0)?;
        Ok(())
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        let mut modified: i32 = 0i32;
        let _t0 = c.iterator()?;
        let mut local_4: Object = _t0;
        loop {
            let _t0 = local_4.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_4.next()?;
            let mut e: Object = _t0;
            index = index.wrapping_add(1i32);
            this.add__i_obj(index, e)?;
            modified = 1i32;
        }
        Ok(modified)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        Ok(AbstractList_Itr::new(this)?)
    }

    // java: listIterator()Ljava/util/ListIterator;
    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.listIterator__i(0i32)?;
        Ok(_t0)
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        Ok(AbstractList_ListItr::new(this, index)?)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.size()?;
        AbstractList::subListRangeCheck(fromIndex, toIndex, _t0)?;
        Ok(AbstractList_SubList::new(this, fromIndex, toIndex)?)
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

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let _t0 = this.listIterator()?;
        let mut e1: Object = _t0;
        let _t1 = o.listIterator()?;
        let mut e2: Object = _t1;
        loop {
            let _t0 = e1.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = e2.hasNext()?;
            let _t1 = e1.next()?;
            let mut o1: Object = _t1;
            let _t2 = e2.next()?;
            let mut o2: Object = _t2;
            let _t3 = o1.equals(o2)?;
            return Ok(0i32);
        }
        let _t2 = e1.hasNext()?;
        let _t3 = e2.hasNext()?;
        Ok(_t3==0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut hashCode: i32 = 1i32;
        let _t0 = this.iterator()?;
        let mut local_2: Object = _t0;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut e: Object = _t0;
            let _t1 = e.hashCode()?;
            hashCode = (0i32).wrapping_add(_t1);
        }
        Ok(hashCode)
    }

    // java: removeRange(II)V
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        let this = self;
        let _t0 = this.listIterator__i(fromIndex)?;
        let mut it: Object = _t0;
        let mut i: i32 = 0i32;
        let mut n: i32 = (toIndex).wrapping_sub(fromIndex);
        loop {
            if i >= n { break; }
            let _t0 = it.next()?;
            it.remove()?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: rangeCheckForAdd(I)V
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        let this = self;
        let _t0 = this.size()?;
        let _t1 = this.outOfBoundsMsg(index)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: outOfBoundsMsg(I)Ljava/lang/String;
    pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Index:"))?;
        String::new().append(&index)?;
        String::new().append(&String::from(", Size:"))?;
        let _t0 = this.size()?;
        String::new().append(&_t0)?;
        Ok(String::new())
    }
}
