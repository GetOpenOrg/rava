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
    pub elementData: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private"))]
    pub size: Field<i32>,
}

impl<E: Clone + 'static> ArrayList<E> {
    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(initialCapacity: i32) -> Result<Self> {
        let this = Self { elementData: Field::new(Default::default()), size: Field::new(0) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        let mut _arr0: Vec<Object> = Vec::with_capacity(initialCapacity as usize);
        this.elementData.set(_arr0);
        this.elementData.set(ArrayList::EMPTY_ELEMENTDATA());
        String::new().append(&String::from("Illegal Capacity:"))?;
        String::new().append(&initialCapacity)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this)
    }

    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { elementData: Field::new(Default::default()), size: Field::new(0) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        this.elementData.set(ArrayList::DEFAULTCAPACITY_EMPTY_ELEMENTDATA());
        Ok(this)
    }

    // java: <init>(Ljava/util/Collection;)V
    // java: <init>(Ljava/util/Collection;)V
    pub fn new__coll(c: Object) -> Result<Self> {
        let this = Self { elementData: Field::new(Default::default()), size: Field::new(0) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        let _t0 = c.toArray()?;
        let mut a: Vec<Object> = _t0;
        this.size.set((a.len() as i32));
        let _t1 = c.getClass()?;
        this.elementData.set(a);
        let _t2: Vec<Object> = Arrays::copyOf(&a, this.size.get(), 56i32)?;
        this.elementData.set(_t2);
        this.elementData.set(ArrayList::EMPTY_ELEMENTDATA());
        Ok(this)
    }

    // java: trimToSize()V
    pub fn trimToSize(&self) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let _t0: Vec<Object> = Arrays::copyOf(&this.elementData.get(), this.size.get())?;
        ArrayList::EMPTY_ELEMENTDATA().elementData.set(_t0);
        Ok(())
    }

    // java: ensureCapacity(I)V
    pub fn ensureCapacity(&self, minCapacity: i32) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let _t0 = this.grow(minCapacity)?;
        Ok(())
    }

    // java: grow(I)[Ljava/lang/Object;
    // java: grow(I)[Ljava/lang/Object;
    pub fn grow__i(&self, minCapacity: i32) -> Result<Vec<Object>> {
        let this = self;
        let mut oldCapacity: i32 = (this.elementData.get().len() as i32);
        let _t0: i32 = ArraysSupport::newLength(oldCapacity, (minCapacity).wrapping_sub(oldCapacity), (oldCapacity>>((1i32&0x1f))))?;
        let mut newCapacity: i32 = _t0;
        let _t1: Vec<Object> = Arrays::copyOf(&this.elementData.get(), newCapacity)?;
        this.elementData.set(_t1);
        return Ok(_t1);
        let _t2: i32 = (10i32).max(minCapacity);
        let mut _arr3: Vec<Object> = Vec::with_capacity(_t2 as usize);
        this.elementData.set(_arr3);
        Ok(_arr3)
    }

    // java: grow()[Ljava/lang/Object;
    // java: grow()[Ljava/lang/Object;
    pub fn grow(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.grow((this.size.get()).wrapping_add(1i32))?;
        Ok(_t0)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(this.size.get()==0i32)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.indexOf(o)?;
        Ok(_t0>=0i32)
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.indexOfRange(o, 0i32, this.size.get())?;
        Ok(_t0)
    }

    // java: indexOfRange(Ljava/lang/Object;II)I
    pub fn indexOfRange(&self, o: Object, start: i32, end: i32) -> Result<i32> {
        let this = self;
        let mut es: Vec<Object> = this.elementData.get();
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

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.lastIndexOfRange(o, 0i32, this.size.get())?;
        Ok(_t0)
    }

    // java: lastIndexOfRange(Ljava/lang/Object;II)I
    pub fn lastIndexOfRange(&self, o: Object, start: i32, end: i32) -> Result<i32> {
        let this = self;
        let mut es: Vec<Object> = this.elementData.get();
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

    // java: clone()Ljava/lang/Object;
    pub fn clone(&self) -> Result<Object> {
        let this = self;
        let mut v: ArrayList = this;
        let _t0: Vec<Object> = Arrays::copyOf(&this.elementData.get(), this.size.get())?;
        v.elementData.set(_t0);
        v.modCount.set(0i32);
        return Ok(v);
        v = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Arrays::copyOf(&this.elementData.get(), this.size.get())?;
        Ok(_t0)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = a.getClass()?;
        let _t1: Vec<Object> = Arrays::copyOf(&this.elementData.get(), this.size.get(), _t0)?;
        return Ok(_t1);
        System::arraycopy(&this.elementData.get(), 0i32, &a, 0i32, this.size.get())?;
        /* TODO: aconst_null  */
        this.size.get()[a as usize] = this.size.get();
        Ok(a)
    }

    // java: elementData(I)Ljava/lang/Object;
    pub fn elementData(&self, index: i32) -> Result<E> {
        let this = self;
        Ok(this.elementData.get()[index as usize].clone())
    }

    // java: elementAt([Ljava/lang/Object;I)Ljava/lang/Object;
    pub fn elementAt(es: &[Object], index: i32) -> Result<E> {
        Ok(es[index as usize].clone())
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        let _t1 = this.elementData(index)?;
        Ok(_t1)
    }

    // java: getFirst()Ljava/lang/Object;
    pub fn getFirst(&self) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.elementData(0i32)?;
        Ok(_t0)
    }

    // java: getLast()Ljava/lang/Object;
    pub fn getLast(&self) -> Result<E> {
        let this = self;
        let mut last: i32 = (this.size.get()).wrapping_sub(1i32);
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.elementData(last)?;
        Ok(_t0)
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        let _t1 = this.elementData(index)?;
        let mut oldValue: Object = _t1;
        this.elementData.get()[index as usize] = element;
        Ok(oldValue)
    }

    // java: add(Ljava/lang/Object;[Ljava/lang/Object;I)V
    // java: add(Ljava/lang/Object;[Ljava/lang/Object;I)V
    pub fn add__obj_arr_obj_i(&self, e: E, elementData: Vec<Object>, s: i32) -> Result<()> {
        let this = self;
        let _t0 = this.grow()?;
        elementData = _t0;
        elementData[s as usize] = e;
        this.size.set((s).wrapping_add(1i32));
        Ok(())
    }

    // java: add(Ljava/lang/Object;)Z
    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, e: E) -> Result<bool> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.add(e, this.elementData.get(), this.size.get())?;
        Ok(1i32)
    }

    // java: add(ILjava/lang/Object;)V
    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, index: i32, element: E) -> Result<()> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut s: i32 = this.size.get();
        let mut elementData: Vec<Object> = this.elementData.get();
        let _t0 = this.grow()?;
        elementData = _t0;
        System::arraycopy(&elementData, index, &elementData, (index).wrapping_add(1i32), (s).wrapping_sub(index))?;
        elementData[index as usize] = element;
        this.size.set((s).wrapping_add(1i32));
        Ok(())
    }

    // java: addFirst(Ljava/lang/Object;)V
    pub fn addFirst(&self, element: E) -> Result<()> {
        let this = self;
        this.add(0i32, element)?;
        Ok(())
    }

    // java: addLast(Ljava/lang/Object;)V
    pub fn addLast(&self, element: E) -> Result<()> {
        let this = self;
        let _t0 = this.add(element)?;
        Ok(())
    }

    // java: remove(I)Ljava/lang/Object;
    // java: remove(I)Ljava/lang/Object;
    pub fn remove__i(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        let mut es: Vec<Object> = this.elementData.get();
        let mut oldValue: Object = es[index as usize].clone();
        this.fastRemove(es, index)?;
        Ok(oldValue)
    }

    // java: removeFirst()Ljava/lang/Object;
    pub fn removeFirst(&self) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut es: Vec<Object> = this.elementData.get();
        let mut oldValue: Object = es[0i32 as usize].clone();
        this.fastRemove(es, 0i32)?;
        Ok(oldValue)
    }

    // java: removeLast()Ljava/lang/Object;
    pub fn removeLast(&self) -> Result<E> {
        let this = self;
        let mut last: i32 = (this.size.get()).wrapping_sub(1i32);
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut es: Vec<Object> = this.elementData.get();
        let mut oldValue: Object = es[last as usize].clone();
        this.fastRemove(es, last)?;
        Ok(oldValue)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
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

    // java: equalsRange(Ljava/util/List;II)Z
    pub fn equalsRange(&self, other: Object, from: i32, to: i32) -> Result<bool> {
        let this = self;
        let mut es: Vec<Object> = this.elementData.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = other.iterator()?;
        let mut oit: Object = _t0;
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

    // java: equalsArrayList(Ljava/util/ArrayList;)Z
    pub fn equalsArrayList(&self, other: Object) -> Result<bool> {
        let this = self;
        let mut otherModCount: i32 = other.modCount.get();
        let mut s: i32 = this.size.get();
        let mut equal: i32 = s == other.size.get();
        let mut otherEs: Vec<Object> = other.elementData.get();
        let mut es: Vec<Object> = this.elementData.get();
        return Err(JvmError::Custom("athrow".to_owned()));
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

    // java: checkForComodification(I)V
    pub fn checkForComodification(&self, expectedModCount: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut expectedModCount: i32 = this.modCount.get();
        let _t0 = this.hashCodeRange(0i32, this.size.get())?;
        let mut hash: i32 = _t0;
        this.checkForComodification(expectedModCount)?;
        Ok(hash)
    }

    // java: hashCodeRange(II)I
    pub fn hashCodeRange(&self, from: i32, to: i32) -> Result<i32> {
        let this = self;
        let mut es: Vec<Object> = this.elementData.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut hashCode: i32 = 1i32;
        let mut i: i32 = from;
        loop {
            if i >= to { break; }
            let mut e: Object = es[i as usize].clone();
            let _t0 = e.hashCode()?;
            hashCode = (0i32).wrapping_add(_t0);
            i = i.wrapping_add(1i32);
        }
        Ok(hashCode)
    }

    // java: remove(Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, o: Object) -> Result<bool> {
        let this = self;
        let mut es: Vec<Object> = this.elementData.get();
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

    // java: fastRemove([Ljava/lang/Object;I)V
    pub fn fastRemove(&self, es: Vec<Object>, i: i32) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut newSize: i32 = (this.size.get()).wrapping_sub(1i32);
        System::arraycopy(&es, (i).wrapping_add(1i32), &es, i, (newSize).wrapping_sub(i))?;
        this.size.set(newSize);
        /* TODO: aconst_null  */
        i[es as usize] = newSize;
        Ok(())
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut es: Vec<Object> = this.elementData.get();
        let mut to: i32 = this.size.get();
        this.size.set(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= to { break; }
            /* TODO: aconst_null  */
            todo!("stack underflow")[es as usize] = i;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: addAll(Ljava/util/Collection;)Z
    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = c.toArray()?;
        let mut a: Vec<Object> = _t0;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut numNew: i32 = (a.len() as i32);
        return Ok(0i32);
        let mut elementData: Vec<Object> = this.elementData.get();
        let mut s: i32 = this.size.get();
        let _t1 = this.grow((s).wrapping_add(numNew))?;
        elementData = _t1;
        System::arraycopy(&a, 0i32, &elementData, s, numNew)?;
        this.size.set((s).wrapping_add(numNew));
        Ok(1i32)
    }

    // java: addAll(ILjava/util/Collection;)Z
    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, index: i32, c: Object) -> Result<bool> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        let _t0 = c.toArray()?;
        let mut a: Vec<Object> = _t0;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut numNew: i32 = (a.len() as i32);
        return Ok(0i32);
        let mut elementData: Vec<Object> = this.elementData.get();
        let mut s: i32 = this.size.get();
        let _t1 = this.grow((s).wrapping_add(numNew))?;
        elementData = _t1;
        let mut numMoved: i32 = (s).wrapping_sub(index);
        System::arraycopy(&elementData, index, &elementData, (index).wrapping_add(numNew), numMoved)?;
        System::arraycopy(&a, 0i32, &elementData, index, numNew)?;
        this.size.set((s).wrapping_add(numNew));
        Ok(1i32)
    }

    // java: removeRange(II)V
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        let this = self;
        let _t0: String = ArrayList::outOfBoundsMsg(fromIndex, toIndex)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.shiftTailOverGap(this.elementData.get(), fromIndex, toIndex)?;
        Ok(())
    }

    // java: shiftTailOverGap([Ljava/lang/Object;II)V
    pub fn shiftTailOverGap(&self, es: Vec<Object>, lo: i32, hi: i32) -> Result<()> {
        let this = self;
        System::arraycopy(&es, hi, &es, lo, (this.size.get()).wrapping_sub(hi))?;
        let mut to: i32 = this.size.get();
        this.size.set((this.size.get()).wrapping_sub((hi).wrapping_sub(lo)));
        let mut i: i32 = (this.size.get()).wrapping_sub((hi).wrapping_sub(lo));
        loop {
            if i >= to { break; }
            /* TODO: aconst_null  */
            todo!("stack underflow")[es as usize] = i;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: rangeCheckForAdd(I)V
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        let this = self;
        let _t0 = this.outOfBoundsMsg(index)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: outOfBoundsMsg(I)Ljava/lang/String;
    // java: outOfBoundsMsg(I)Ljava/lang/String;
    pub fn outOfBoundsMsg__i(&self, index: i32) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Index:"))?;
        String::new().append(&index)?;
        String::new().append(&String::from(", Size:"))?;
        String::new().append(&this.size.get())?;
        Ok(String::new())
    }

    // java: outOfBoundsMsg(II)Ljava/lang/String;
    // java: outOfBoundsMsg(II)Ljava/lang/String;
    pub fn outOfBoundsMsg__i_i(fromIndex: i32, toIndex: i32) -> Result<String> {
        String::new().append(&String::from("From Index:"))?;
        String::new().append(&fromIndex)?;
        String::new().append(&String::from("> To Index:"))?;
        String::new().append(&toIndex)?;
        Ok(String::new())
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.batchRemove(c, 0i32, 0i32, this.size.get())?;
        Ok(_t0)
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.batchRemove(c, 1i32, 0i32, this.size.get())?;
        Ok(_t0)
    }

    // java: batchRemove(Ljava/util/Collection;ZII)Z
    pub fn batchRemove(&self, c: Object, complement: bool, from: i32, end: i32) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(c)?;
        let mut es: Vec<Object> = this.elementData.get();
        let mut r: i32 = from;
        return Ok(0i32);
        let _t1 = c.contains(es[r as usize].clone())?;
        r = r.wrapping_add(1i32);
        r = r.wrapping_add(1i32);
        let mut w: i32 = r;
        loop {
            if r >= end { break; }
            let mut e: Object = es[r as usize].clone();
            let _t0 = c.contains(es[r as usize].clone())?;
            w = w.wrapping_add(1i32);
            es[w as usize] = e;
            r = r.wrapping_add(1i32);
        }
        this.modCount.set((this.modCount.get()).wrapping_add((end).wrapping_sub(w)));
        this.shiftTailOverGap(es, w, end)?;
        e = complement;
        System::arraycopy(&es, r, &es, w, (end).wrapping_sub(r))?;
        w = (w).wrapping_add((end).wrapping_sub(r));
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut local_9: bool = _t1;
        this.modCount.set((this.modCount.get()).wrapping_add((end).wrapping_sub(w)));
        this.shiftTailOverGap(es, w, end)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(1i32)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
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
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        let _t0 = s.readInt()?;
        let _t1: Object = SharedSecrets::getJavaObjectInputStreamAccess()?;
        _t1.checkArray(s, 56i32, this.size.get())?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(this.size.get() as usize);
        let mut elements: Vec<Object> = _arr2;
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
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        Ok(ArrayList_ListItr::new(this, index)?)
    }

    // java: listIterator()Ljava/util/ListIterator;
    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        let this = self;
        Ok(ArrayList_ListItr::new(this, 0i32)?)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        Ok(ArrayList_Itr::new(this)?)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        ArrayList::subListRangeCheck(fromIndex, toIndex, this.size.get())?;
        Ok(ArrayList_SubList::new(this, fromIndex, toIndex)?)
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let mut expectedModCount: i32 = this.modCount.get();
        let mut es: Vec<Object> = this.elementData.get();
        let mut size: i32 = this.size.get();
        let mut i: i32 = 0i32;
        loop {
            if this.modCount.get() != expectedModCount { break; }
            let _t0: Object = ArrayList::elementAt(&es, i)?;
            action.accept(_t0)?;
            i = i.wrapping_add(1i32);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        Ok(ArrayList_ArrayListSpliterator::new(this, 0i32, -1i32, 0i32)?)
    }

    // java: nBits(I)[J
    pub fn nBits(n: i32) -> Result<Vec<i64>> {
        let mut _arr0: Vec<i64> = vec![0i64; (((n).wrapping_sub(1i32)>>((6i32&0x1f)))).wrapping_add(1i32) as usize];
        Ok(_arr0)
    }

    // java: setBit([JI)V
    pub fn setBit(bits: &[i64], i: i32) -> Result<()> {
        /* TODO: dup2  */
        /* TODO: lshl  */
        /* TODO: lor  */
        bits[(i>>((6i32&0x1f))) as usize][1i64 as usize] = i;
        Ok(())
    }

    // java: isClear([JI)Z
    pub fn isClear(bits: &[i64], i: i32) -> Result<bool> {
        /* TODO: lshl  */
        /* TODO: land  */
        /* TODO: lcmp  */
        Ok(0i64==0i32)
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf__predic(&self, filter: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.removeIf(filter, 0i32, this.size.get())?;
        Ok(_t0)
    }

    // java: removeIf(Ljava/util/function/Predicate;II)Z
    // java: removeIf(Ljava/util/function/Predicate;II)Z
    pub fn removeIf__predic_i_i(&self, filter: Object, i: i32, end: i32) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(filter)?;
        let mut expectedModCount: i32 = this.modCount.get();
        let mut es: Vec<Object> = this.elementData.get();
        loop {
            if i >= end { break; }
            let _t0: Object = ArrayList::elementAt(&es, i)?;
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
            let _t0: Object = ArrayList::elementAt(&es, i)?;
            let _t1 = filter.test(_t0)?;
            ArrayList::setBit(&deathRow, (i).wrapping_sub(beg))?;
            i = i.wrapping_add(1i32);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
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
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(0i32)
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        this.replaceAllRange(operator, 0i32, this.size.get())?;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(())
    }

    // java: replaceAllRange(Ljava/util/function/UnaryOperator;II)V
    pub fn replaceAllRange(&self, operator: Object, i: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(operator)?;
        let mut expectedModCount: i32 = this.modCount.get();
        let mut es: Vec<Object> = this.elementData.get();
        loop {
            if this.modCount.get() != expectedModCount { break; }
            let _t0: Object = ArrayList::elementAt(&es, i)?;
            let _t1 = operator.apply(_t0)?;
            es[i as usize] = _t1;
            i = i.wrapping_add(1i32);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        let mut expectedModCount: i32 = this.modCount.get();
        Arrays::sort(&this.elementData.get(), 0i32, this.size.get(), c)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(())
    }

    // java: checkInvariants()V
    pub fn checkInvariants(&self) -> Result<()> {
        let this = self;
        Ok(())
    }
}
