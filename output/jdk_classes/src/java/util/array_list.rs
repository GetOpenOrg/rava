#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/List,java/util/RandomAccess,java/lang/Cloneable,java/io/Serializable",
    access      = "public",
    source      = "ArrayList.java",
))]
pub struct ArrayList<E> {
    #[cfg_attr(any(), java_field(name = "elementData", descriptor = "[Ljava/lang/Object;", access = ""))]
    pub elementData: Field<JvmObject>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private"))]
    pub size: Field<i32>,
}

impl<E: Clone + 'static> ArrayList<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    // java: <init>(I)V
    pub fn new__i(initialCapacity: i32) -> Result<Self> {
        let this = Self { elementData: Field::new(Default::default()), size: Field::new(0) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(initialCapacity as usize);
        this.elementData.set(_arr0);
        this.elementData.set(ArrayList::EMPTY_ELEMENTDATA());
        String::new().append(&String::from("Illegal Capacity:"))?;
        String::new().append(&initialCapacity)?;
        panic!("{}", /* IllegalArgumentException::new(String::new())? */);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { elementData: Field::new(Default::default()), size: Field::new(0) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        this.elementData.set(ArrayList::DEFAULTCAPACITY_EMPTY_ELEMENTDATA());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Collection;)V", access = "public"))]
    // java: <init>(Ljava/util/Collection;)V
    pub fn new__coll(c: JvmObject) -> Result<Self> {
        let this = Self { elementData: Field::new(Default::default()), size: Field::new(0) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        let _t0 = c.toArray()?;
        let mut a: JvmObject = _t0;
        this.size.set((a.len() as i32));
        let _t1 = c.getClass()?;
        this.elementData.set(a);
        let _t2: JvmObject = Arrays::copyOf(a, this.size.get(), 56i32)?;
        this.elementData.set(_t2);
        this.elementData.set(ArrayList::EMPTY_ELEMENTDATA());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "trimToSize", descriptor = "()V", access = "public"))]
    pub fn trimToSize(&self) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let _t0: JvmObject = Arrays::copyOf(this.elementData.get(), this.size.get())?;
        ArrayList::EMPTY_ELEMENTDATA().elementData.set(_t0);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "ensureCapacity", descriptor = "(I)V", access = "public"))]
    pub fn ensureCapacity(&self, minCapacity: i32) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let _t0 = this.grow(minCapacity)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "grow", descriptor = "(I)[Ljava/lang/Object;", access = "private"))]
    // java: grow(I)[Ljava/lang/Object;
    pub fn grow__i(&self, minCapacity: i32) -> Result<JvmObject> {
        let this = self;
        let mut oldCapacity: i32 = (this.elementData.get().len() as i32);
        let _t0: i32 = ArraysSupport::newLength(oldCapacity, (minCapacity).wrapping_sub(oldCapacity), (oldCapacity>>((1i32&0x1f))))?;
        let mut newCapacity: i32 = _t0;
        let _t1: JvmObject = Arrays::copyOf(this.elementData.get(), newCapacity)?;
        this.elementData.set(_t1);
        return Ok(_t1);
        let _t2: i32 = (10i32).max(minCapacity);
        let mut _arr3: Vec<JvmObject> = Vec::with_capacity(_t2 as usize);
        this.elementData.set(_arr3);
        Ok(_arr3)
    }

    #[cfg_attr(any(), java_method(name = "grow", descriptor = "()[Ljava/lang/Object;", access = "private"))]
    // java: grow()[Ljava/lang/Object;
    pub fn grow(&self) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.grow((this.size.get()).wrapping_add(1i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(this.size.get()==0i32)
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn contains(&self, o: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = this.indexOf(o)?;
        Ok(_t0>=0i32)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn indexOf(&self, o: JvmObject) -> Result<i32> {
        let this = self;
        let _t0 = this.indexOfRange(o, 0i32, this.size.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOfRange", descriptor = "(Ljava/lang/Object;II)I"))]
    pub fn indexOfRange(&self, o: JvmObject, start: i32, end: i32) -> Result<i32> {
        let this = self;
        let mut es: JvmObject = this.elementData.get();
        let mut i: i32 = start;
        loop {
            if i >= end { break; }
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        i = start;
        loop {
            if i >= end { break; }
            let _t0 = o.equals(es[i as usize].clone())?;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn lastIndexOf(&self, o: JvmObject) -> Result<i32> {
        let this = self;
        let _t0 = this.lastIndexOfRange(o, 0i32, this.size.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOfRange", descriptor = "(Ljava/lang/Object;II)I"))]
    pub fn lastIndexOfRange(&self, o: JvmObject, start: i32, end: i32) -> Result<i32> {
        let this = self;
        let mut es: JvmObject = this.elementData.get();
        let mut i: i32 = (end).wrapping_sub(1i32);
        loop {
            if i < start { break; }
            return Ok(i);
            i = i.wrapping_sub(1i32);
        }
        i = (end).wrapping_sub(1i32);
        loop {
            if i < start { break; }
            let _t0 = o.equals(es[i as usize].clone())?;
            return Ok(i);
            i = i.wrapping_sub(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn clone(&self) -> Result<JvmObject> {
        let this = self;
        let mut v: java/util/ArrayList = this;
        let _t0: JvmObject = Arrays::copyOf(this.elementData.get(), this.size.get())?;
        v.elementData.set(_t0);
        v.modCount.set(0i32);
        return Ok(v);
        v = /* UNDERFLOW */;
        panic!("{}", /* InternalError::new(v)? */);
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public"))]
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = Arrays::copyOf(this.elementData.get(), this.size.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public"))]
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = a.getClass()?;
        let _t1: JvmObject = Arrays::copyOf(this.elementData.get(), this.size.get(), _t0)?;
        return Ok(_t1);
        System::arraycopy(this.elementData.get(), 0i32, a, 0i32, this.size.get())?;
        /* TODO: aconst_null  */
        this.size.get()[a as usize] = this.size.get();
        Ok(a)
    }

    #[cfg_attr(any(), java_method(name = "elementData", descriptor = "(I)Ljava/lang/Object;"))]
    pub fn elementData(&self, index: i32) -> Result<E> {
        let this = self;
        Ok(this.elementData.get()[index as usize].clone())
    }

    #[cfg_attr(any(), java_method(name = "elementAt", descriptor = "([Ljava/lang/Object;I)Ljava/lang/Object;", access = "static"))]
    pub fn elementAt(es: JvmObject, index: i32) -> Result<E> {
        Ok(es[index as usize].clone())
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        let _t1 = this.elementData(index)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getFirst", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn getFirst(&self) -> Result<E> {
        let this = self;
        panic!("{}", /* NoSuchElementException::new()? */);
        let _t0 = this.elementData(0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getLast", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn getLast(&self) -> Result<E> {
        let this = self;
        let mut last: i32 = (this.size.get()).wrapping_sub(1i32);
        panic!("{}", /* NoSuchElementException::new()? */);
        let _t0 = this.elementData(last)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        let _t1 = this.elementData(index)?;
        let mut oldValue: JvmObject = _t1;
        this.elementData.get()[index as usize] = element;
        Ok(oldValue)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;[Ljava/lang/Object;I)V", access = "private"))]
    // java: add(Ljava/lang/Object;[Ljava/lang/Object;I)V
    pub fn add__obj_arr_obj_i(&self, e: E, elementData: JvmObject, s: i32) -> Result<()> {
        let this = self;
        let _t0 = this.grow()?;
        elementData = _t0;
        elementData[s as usize] = e;
        this.size.set((s).wrapping_add(1i32));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, e: E) -> Result<bool> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.add(e, this.elementData.get(), this.size.get())?;
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public"))]
    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, index: i32, element: E) -> Result<()> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut s: i32 = this.size.get();
        let mut elementData: JvmObject = this.elementData.get();
        let _t0 = this.grow()?;
        elementData = _t0;
        System::arraycopy(elementData, index, elementData, (index).wrapping_add(1i32), (s).wrapping_sub(index))?;
        elementData[index as usize] = element;
        this.size.set((s).wrapping_add(1i32));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addFirst", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn addFirst(&self, element: E) -> Result<()> {
        let this = self;
        this.add(0i32, element)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addLast", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn addLast(&self, element: E) -> Result<()> {
        let this = self;
        let _t0 = this.add(element)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public"))]
    // java: remove(I)Ljava/lang/Object;
    pub fn remove__i(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        let mut es: JvmObject = this.elementData.get();
        let mut oldValue: JvmObject = es[index as usize].clone();
        this.fastRemove(es, index)?;
        Ok(oldValue)
    }

    #[cfg_attr(any(), java_method(name = "removeFirst", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn removeFirst(&self) -> Result<E> {
        let this = self;
        panic!("{}", /* NoSuchElementException::new()? */);
        let mut es: JvmObject = this.elementData.get();
        let mut oldValue: JvmObject = es[0i32 as usize].clone();
        this.fastRemove(es, 0i32)?;
        Ok(oldValue)
    }

    #[cfg_attr(any(), java_method(name = "removeLast", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn removeLast(&self) -> Result<E> {
        let this = self;
        let mut last: i32 = (this.size.get()).wrapping_sub(1i32);
        panic!("{}", /* NoSuchElementException::new()? */);
        let mut es: JvmObject = this.elementData.get();
        let mut oldValue: JvmObject = es[last as usize].clone();
        this.fastRemove(es, last)?;
        Ok(oldValue)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, o: JvmObject) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let mut expectedModCount: i32 = this.modCount.get();
        let _t0 = o.getClass()?;
        let _t1 = this.equalsArrayList(o)?;
        let _t2 = this.equalsRange(o, 0i32, this.size.get())?;
        let mut equal: i32 = _t2;
        this.checkForComodification(expectedModCount)?;
        Ok(equal)
    }

    #[cfg_attr(any(), java_method(name = "equalsRange", descriptor = "(Ljava/util/List;II)Z"))]
    pub fn equalsRange(&self, other: JvmObject, from: i32, to: i32) -> Result<bool> {
        let this = self;
        let mut es: JvmObject = this.elementData.get();
        panic!("{}", /* ConcurrentModificationException::new()? */);
        let _t0 = other.iterator()?;
        let mut oit: JvmObject = _t0;
        loop {
            if from >= to { break; }
            let _t0 = oit.hasNext()?;
            let _t1 = oit.next()?;
            let _t2: bool = Objects::equals(es[from as usize].clone(), _t1)?;
            return Ok(0i32);
            from = from.wrapping_add(1i32);
        }
        let _t1 = oit.hasNext()?;
        Ok(_t1==0i32)
    }

    #[cfg_attr(any(), java_method(name = "equalsArrayList", descriptor = "(Ljava/util/ArrayList;)Z", access = "private"))]
    pub fn equalsArrayList(&self, other: JvmObject) -> Result<bool> {
        let this = self;
        let mut otherModCount: i32 = other.modCount.get();
        let mut s: i32 = this.size.get();
        let mut equal: i32 = s == other.size.get();
        let mut otherEs: JvmObject = other.elementData.get();
        let mut es: JvmObject = this.elementData.get();
        panic!("{}", /* ConcurrentModificationException::new()? */);
        let mut i: i32 = 0i32;
        loop {
            if i >= s { break; }
            let _t0: bool = Objects::equals(es[i as usize].clone(), otherEs[i as usize].clone())?;
            equal = 0i32;
            i = i.wrapping_add(1i32);
        }
        other.checkForComodification(otherModCount)?;
        Ok(equal)
    }

    #[cfg_attr(any(), java_method(name = "checkForComodification", descriptor = "(I)V", access = "private"))]
    pub fn checkForComodification(&self, expectedModCount: i32) -> Result<()> {
        let this = self;
        panic!("{}", /* ConcurrentModificationException::new()? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut expectedModCount: i32 = this.modCount.get();
        let _t0 = this.hashCodeRange(0i32, this.size.get())?;
        let mut hash: i32 = _t0;
        this.checkForComodification(expectedModCount)?;
        Ok(hash)
    }

    #[cfg_attr(any(), java_method(name = "hashCodeRange", descriptor = "(II)I"))]
    pub fn hashCodeRange(&self, from: i32, to: i32) -> Result<i32> {
        let this = self;
        let mut es: JvmObject = this.elementData.get();
        panic!("{}", /* ConcurrentModificationException::new()? */);
        let mut hashCode: i32 = 1i32;
        let mut i: i32 = from;
        loop {
            if i >= to { break; }
            let mut e: JvmObject = es[i as usize].clone();
            let _t0 = e.hashCode()?;
            hashCode = (0i32).wrapping_add(_t0);
            i = i.wrapping_add(1i32);
        }
        Ok(hashCode)
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, o: JvmObject) -> Result<bool> {
        let this = self;
        let mut es: JvmObject = this.elementData.get();
        let mut size: i32 = this.size.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            i = i.wrapping_add(1i32);
        }
        loop {
            if i >= size { break; }
            let _t0 = o.equals(es[i as usize].clone())?;
            i = i.wrapping_add(1i32);
        }
        return Ok(0i32);
        this.fastRemove(es, i)?;
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "fastRemove", descriptor = "([Ljava/lang/Object;I)V", access = "private"))]
    pub fn fastRemove(&self, es: JvmObject, i: i32) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut newSize: i32 = (this.size.get()).wrapping_sub(1i32);
        System::arraycopy(es, (i).wrapping_add(1i32), es, i, (newSize).wrapping_sub(i))?;
        this.size.set(newSize);
        /* TODO: aconst_null  */
        i[es as usize] = newSize;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public"))]
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut es: JvmObject = this.elementData.get();
        let mut to: i32 = this.size.get();
        this.size.set(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= to { break; }
            /* TODO: aconst_null  */
            /* UNDERFLOW */[es as usize] = i;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public"))]
    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, c: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = c.toArray()?;
        let mut a: JvmObject = _t0;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut numNew: i32 = (a.len() as i32);
        return Ok(0i32);
        let mut elementData: JvmObject = this.elementData.get();
        let mut s: i32 = this.size.get();
        let _t1 = this.grow((s).wrapping_add(numNew))?;
        elementData = _t1;
        System::arraycopy(a, 0i32, elementData, s, numNew)?;
        this.size.set((s).wrapping_add(numNew));
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public"))]
    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, index: i32, c: JvmObject) -> Result<bool> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        let _t0 = c.toArray()?;
        let mut a: JvmObject = _t0;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut numNew: i32 = (a.len() as i32);
        return Ok(0i32);
        let mut elementData: JvmObject = this.elementData.get();
        let mut s: i32 = this.size.get();
        let _t1 = this.grow((s).wrapping_add(numNew))?;
        elementData = _t1;
        let mut numMoved: i32 = (s).wrapping_sub(index);
        System::arraycopy(elementData, index, elementData, (index).wrapping_add(numNew), numMoved)?;
        System::arraycopy(a, 0i32, elementData, index, numNew)?;
        this.size.set((s).wrapping_add(numNew));
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "removeRange", descriptor = "(II)V", access = "protected"))]
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        let this = self;
        let _t0: String = ArrayList::outOfBoundsMsg(fromIndex, toIndex)?;
        panic!("{}", /* IndexOutOfBoundsException::new(_t0)? */);
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.shiftTailOverGap(this.elementData.get(), fromIndex, toIndex)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "shiftTailOverGap", descriptor = "([Ljava/lang/Object;II)V", access = "private"))]
    pub fn shiftTailOverGap(&self, es: JvmObject, lo: i32, hi: i32) -> Result<()> {
        let this = self;
        System::arraycopy(es, hi, es, lo, (this.size.get()).wrapping_sub(hi))?;
        let mut to: i32 = this.size.get();
        this.size.set((this.size.get()).wrapping_sub((hi).wrapping_sub(lo)));
        let mut i: i32 = (this.size.get()).wrapping_sub((hi).wrapping_sub(lo));
        loop {
            if i >= to { break; }
            /* TODO: aconst_null  */
            /* UNDERFLOW */[es as usize] = i;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "rangeCheckForAdd", descriptor = "(I)V", access = "private"))]
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        let this = self;
        let _t0 = this.outOfBoundsMsg(index)?;
        panic!("{}", /* IndexOutOfBoundsException::new(_t0)? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "outOfBoundsMsg", descriptor = "(I)Ljava/lang/String;", access = "private"))]
    // java: outOfBoundsMsg(I)Ljava/lang/String;
    pub fn outOfBoundsMsg__i(&self, index: i32) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Index:"))?;
        String::new().append(&index)?;
        String::new().append(&String::from(", Size:"))?;
        String::new().append(&this.size.get())?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "outOfBoundsMsg", descriptor = "(II)Ljava/lang/String;", access = "private static"))]
    // java: outOfBoundsMsg(II)Ljava/lang/String;
    pub fn outOfBoundsMsg__i_i(fromIndex: i32, toIndex: i32) -> Result<String> {
        String::new().append(&String::from("From Index:"))?;
        String::new().append(&fromIndex)?;
        String::new().append(&String::from("> To Index:"))?;
        String::new().append(&toIndex)?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public"))]
    pub fn removeAll(&self, c: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = this.batchRemove(c, 0i32, 0i32, this.size.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public"))]
    pub fn retainAll(&self, c: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = this.batchRemove(c, 1i32, 0i32, this.size.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "batchRemove", descriptor = "(Ljava/util/Collection;ZII)Z"))]
    pub fn batchRemove(&self, c: JvmObject, complement: bool, from: i32, end: i32) -> Result<bool> {
        let this = self;
        let _t0: JvmObject = Objects::requireNonNull(c)?;
        let mut es: JvmObject = this.elementData.get();
        let mut r: i32 = from;
        return Ok(0i32);
        let _t1 = c.contains(es[r as usize].clone())?;
        r = r.wrapping_add(1i32);
        r = r.wrapping_add(1i32);
        let mut w: i32 = r;
        loop {
            if r >= end { break; }
            let mut e: JvmObject = es[r as usize].clone();
            let _t0 = c.contains(es[r as usize].clone())?;
            w = w.wrapping_add(1i32);
            es[w as usize] = e;
            r = r.wrapping_add(1i32);
        }
        this.modCount.set((this.modCount.get()).wrapping_add((end).wrapping_sub(w)));
        this.shiftTailOverGap(es, w, end)?;
        e = complement;
        System::arraycopy(es, r, es, w, (end).wrapping_sub(r))?;
        w = (w).wrapping_add((end).wrapping_sub(r));
        panic!("{}", /* e */);
        let mut local_9: bool = _t1;
        this.modCount.set((this.modCount.get()).wrapping_add((end).wrapping_sub(w)));
        this.shiftTailOverGap(es, w, end)?;
        panic!("{}", /* local_9 */);
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private"))]
    pub fn writeObject(&self, s: JvmObject) -> Result<()> {
        let this = self;
        let mut expectedModCount: i32 = this.modCount.get();
        s.defaultWriteObject()?;
        s.writeInt(this.size.get())?;
        let mut i: i32 = 0i32;
        loop {
            if i >= this.size.get() { break; }
            s.writeObject(this.elementData.get()[i as usize].clone())?;
            i = i.wrapping_add(1i32);
        }
        panic!("{}", /* ConcurrentModificationException::new()? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, s: JvmObject) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        let _t0 = s.readInt()?;
        let _t1: JvmObject = SharedSecrets::getJavaObjectInputStreamAccess()?;
        _t1.checkArray(s, 56i32, this.size.get())?;
        let mut _arr2: Vec<JvmObject> = Vec::with_capacity(this.size.get() as usize);
        let mut elements: Vec<JvmObject> = _arr2;
        let mut i: i32 = 0i32;
        loop {
            if i >= this.size.get() { break; }
            let _t0 = s.readObject()?;
            elements[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        this.elementData.set(elements);
        this.elementData.set(ArrayList::EMPTY_ELEMENTDATA());
        String::new().append(&String::from("Invalid size:"))?;
        String::new().append(&this.size.get())?;
        panic!("{}", /* InvalidObjectException::new(String::new())? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public"))]
    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<JvmObject> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        Ok(ArrayList$ListItr::new(this, index)?)
    }

    #[cfg_attr(any(), java_method(name = "listIterator", descriptor = "()Ljava/util/ListIterator;", access = "public"))]
    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<JvmObject> {
        let this = self;
        Ok(ArrayList$ListItr::new(this, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public"))]
    pub fn iterator(&self) -> Result<JvmObject> {
        let this = self;
        Ok(ArrayList$Itr::new(this)?)
    }

    #[cfg_attr(any(), java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public"))]
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<JvmObject> {
        let this = self;
        ArrayList::subListRangeCheck(fromIndex, toIndex, this.size.get())?;
        Ok(ArrayList$SubList::new(this, fromIndex, toIndex)?)
    }

    #[cfg_attr(any(), java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public"))]
    pub fn forEach(&self, action: JvmObject) -> Result<()> {
        let this = self;
        let _t0: JvmObject = Objects::requireNonNull(action)?;
        let mut expectedModCount: i32 = this.modCount.get();
        let mut es: JvmObject = this.elementData.get();
        let mut size: i32 = this.size.get();
        let mut i: i32 = 0i32;
        loop {
            if this.modCount.get() != expectedModCount { break; }
            let _t0: JvmObject = ArrayList::elementAt(es, i)?;
            action.accept(_t0)?;
            i = i.wrapping_add(1i32);
        }
        panic!("{}", /* ConcurrentModificationException::new()? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public"))]
    pub fn spliterator(&self) -> Result<JvmObject> {
        let this = self;
        Ok(ArrayList$ArrayListSpliterator::new(this, 0i32, -1i32, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "nBits", descriptor = "(I)[J", access = "private static"))]
    pub fn nBits(n: i32) -> Result<Vec<i64>> {
        let mut _arr0: Vec<i64> = vec![0i64; (((n).wrapping_sub(1i32)>>((6i32&0x1f)))).wrapping_add(1i32) as usize];
        Ok(_arr0)
    }

    #[cfg_attr(any(), java_method(name = "setBit", descriptor = "([JI)V", access = "private static"))]
    pub fn setBit(bits: &[i64], i: i32) -> Result<()> {
        /* TODO: dup2  */
        /* TODO: lshl  */
        /* TODO: lor  */
        bits[(i>>((6i32&0x1f))) as usize][1i64 as usize] = i;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "isClear", descriptor = "([JI)Z", access = "private static"))]
    pub fn isClear(bits: &[i64], i: i32) -> Result<bool> {
        /* TODO: lshl  */
        /* TODO: land  */
        /* TODO: lcmp  */
        Ok(0i64==0i32)
    }

    #[cfg_attr(any(), java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public"))]
    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf__predic(&self, filter: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = this.removeIf(filter, 0i32, this.size.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;II)Z"))]
    // java: removeIf(Ljava/util/function/Predicate;II)Z
    pub fn removeIf__predic_i_i(&self, filter: JvmObject, i: i32, end: i32) -> Result<bool> {
        let this = self;
        let _t0: JvmObject = Objects::requireNonNull(filter)?;
        let mut expectedModCount: i32 = this.modCount.get();
        let mut es: JvmObject = this.elementData.get();
        loop {
            if i >= end { break; }
            let _t0: JvmObject = ArrayList::elementAt(es, i)?;
            let _t1 = filter.test(_t0)?;
            i = i.wrapping_add(1i32);
        }
        let mut beg: i32 = i;
        let _t1: Vec<i64> = ArrayList::nBits((end).wrapping_sub(beg))?;
        let mut deathRow: Vec<i64> = _t1;
        deathRow[0i32 as usize] = 1i64;
        i = (beg).wrapping_add(1i32);
        loop {
            if i >= end { break; }
            let _t0: JvmObject = ArrayList::elementAt(es, i)?;
            let _t1 = filter.test(_t0)?;
            ArrayList::setBit(&deathRow, (i).wrapping_sub(beg))?;
            i = i.wrapping_add(1i32);
        }
        panic!("{}", /* ConcurrentModificationException::new()? */);
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut w: i32 = beg;
        i = beg;
        loop {
            if i >= end { break; }
            let _t0: bool = ArrayList::isClear(&deathRow, (i).wrapping_sub(beg))?;
            w = w.wrapping_add(1i32);
            es[w as usize] = es[i as usize].clone();
            i = i.wrapping_add(1i32);
        }
        this.shiftTailOverGap(es, w, end)?;
        return Ok(1i32);
        panic!("{}", /* ConcurrentModificationException::new()? */);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public"))]
    pub fn replaceAll(&self, operator: JvmObject) -> Result<()> {
        let this = self;
        this.replaceAllRange(operator, 0i32, this.size.get())?;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "replaceAllRange", descriptor = "(Ljava/util/function/UnaryOperator;II)V", access = "private"))]
    pub fn replaceAllRange(&self, operator: JvmObject, i: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0: JvmObject = Objects::requireNonNull(operator)?;
        let mut expectedModCount: i32 = this.modCount.get();
        let mut es: JvmObject = this.elementData.get();
        loop {
            if this.modCount.get() != expectedModCount { break; }
            let _t0: JvmObject = ArrayList::elementAt(es, i)?;
            let _t1 = operator.apply(_t0)?;
            es[i as usize] = _t1;
            i = i.wrapping_add(1i32);
        }
        panic!("{}", /* ConcurrentModificationException::new()? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "sort", descriptor = "(Ljava/util/Comparator;)V", access = "public"))]
    pub fn sort(&self, c: JvmObject) -> Result<()> {
        let this = self;
        let mut expectedModCount: i32 = this.modCount.get();
        Arrays::sort(this.elementData.get(), 0i32, this.size.get(), c)?;
        panic!("{}", /* ConcurrentModificationException::new()? */);
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkInvariants", descriptor = "()V"))]
    pub fn checkInvariants(&self) -> Result<()> {
        let this = self;
        Ok(())
    }
}
