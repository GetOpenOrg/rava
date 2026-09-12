#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/List",
    super_class = "java/lang/Object",
    interfaces  = "java/util/SequencedCollection",
    access      = "public abstract",
    source      = "List.java",
))]
pub struct List<E>;

impl<E: Clone + 'static> List<E> {
    #[cfg_attr(any(), java_native(name = "size", descriptor = "()I", access = "public abstract"))]
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/List.size")
    }

    #[cfg_attr(any(), java_native(name = "isEmpty", descriptor = "()Z", access = "public abstract"))]
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/List.isEmpty")
    }

    #[cfg_attr(any(), java_native(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public abstract"))]
    pub fn contains(&self, arg0: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.contains")
    }

    #[cfg_attr(any(), java_native(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public abstract"))]
    pub fn iterator(&self) -> Result<JvmObject> {
        todo!("abstract java/util/List.iterator")
    }

    #[cfg_attr(any(), java_native(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public abstract"))]
    pub fn toArray(&self) -> Result<JvmObject> {
        todo!("abstract java/util/List.toArray")
    }

    #[cfg_attr(any(), java_native(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public abstract"))]
    pub fn toArray__arr_obj(&self, arg0: JvmObject) -> Result<JvmObject> {
        todo!("abstract java/util/List.toArray")
    }

    #[cfg_attr(any(), java_native(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public abstract"))]
    pub fn add__obj(&self, arg0: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.add")
    }

    #[cfg_attr(any(), java_native(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public abstract"))]
    pub fn remove__obj(&self, arg0: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.remove")
    }

    #[cfg_attr(any(), java_native(name = "containsAll", descriptor = "(Ljava/util/Collection;)Z", access = "public abstract"))]
    pub fn containsAll(&self, arg0: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.containsAll")
    }

    #[cfg_attr(any(), java_native(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public abstract"))]
    pub fn addAll__coll(&self, arg0: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.addAll")
    }

    #[cfg_attr(any(), java_native(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public abstract"))]
    pub fn addAll__i_coll(&self, arg0: i32, arg1: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.addAll")
    }

    #[cfg_attr(any(), java_native(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public abstract"))]
    pub fn removeAll(&self, arg0: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.removeAll")
    }

    #[cfg_attr(any(), java_native(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public abstract"))]
    pub fn retainAll(&self, arg0: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.retainAll")
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public"))]
    pub fn replaceAll(&self, operator: JvmObject) -> Result<()> {
        let this = self;
        let _t0: JvmObject = Objects::requireNonNull(operator)?;
        let _t1 = this.listIterator()?;
        let mut li: JvmObject = _t1;
        loop {
            let _t0 = li.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = li.next()?;
            let _t1 = operator.apply(_t0)?;
            li.set(_t1)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "sort", descriptor = "(Ljava/util/Comparator;)V", access = "public"))]
    pub fn sort(&self, c: JvmObject) -> Result<()> {
        let this = self;
        let _t0 = this.toArray()?;
        let mut a: JvmObject = _t0;
        Arrays::sort(a, c)?;
        let _t1 = this.listIterator()?;
        let mut i: JvmObject = _t1;
        let mut local_4: JvmObject = a;
        let mut local_5: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut e: JvmObject = local_4[local_6 as usize].clone();
            let _t0 = i.next()?;
            i.set(e)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_native(name = "clear", descriptor = "()V", access = "public abstract"))]
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/List.clear")
    }

    #[cfg_attr(any(), java_native(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public abstract"))]
    pub fn equals(&self, arg0: JvmObject) -> Result<bool> {
        todo!("abstract java/util/List.equals")
    }

    #[cfg_attr(any(), java_native(name = "hashCode", descriptor = "()I", access = "public abstract"))]
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/List.hashCode")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public abstract"))]
    pub fn get(&self, arg0: i32) -> Result<JvmObject> {
        todo!("abstract java/util/List.get")
    }

    #[cfg_attr(any(), java_native(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public abstract"))]
    pub fn set(&self, arg0: i32, arg1: JvmObject) -> Result<JvmObject> {
        todo!("abstract java/util/List.set")
    }

    #[cfg_attr(any(), java_native(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public abstract"))]
    pub fn add__i_obj(&self, arg0: i32, arg1: JvmObject) -> Result<()> {
        todo!("abstract java/util/List.add")
    }

    #[cfg_attr(any(), java_native(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public abstract"))]
    pub fn remove__i(&self, arg0: i32) -> Result<JvmObject> {
        todo!("abstract java/util/List.remove")
    }

    #[cfg_attr(any(), java_native(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public abstract"))]
    pub fn indexOf(&self, arg0: JvmObject) -> Result<i32> {
        todo!("abstract java/util/List.indexOf")
    }

    #[cfg_attr(any(), java_native(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public abstract"))]
    pub fn lastIndexOf(&self, arg0: JvmObject) -> Result<i32> {
        todo!("abstract java/util/List.lastIndexOf")
    }

    #[cfg_attr(any(), java_native(name = "listIterator", descriptor = "()Ljava/util/ListIterator;", access = "public abstract"))]
    pub fn listIterator(&self) -> Result<JvmObject> {
        todo!("abstract java/util/List.listIterator")
    }

    #[cfg_attr(any(), java_native(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public abstract"))]
    pub fn listIterator__i(&self, arg0: i32) -> Result<JvmObject> {
        todo!("abstract java/util/List.listIterator")
    }

    #[cfg_attr(any(), java_native(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public abstract"))]
    pub fn subList(&self, arg0: i32, arg1: i32) -> Result<JvmObject> {
        todo!("abstract java/util/List.subList")
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public"))]
    pub fn spliterator(&self) -> Result<JvmObject> {
        let this = self;
        return Ok(AbstractList$RandomAccessSpliterator::new(this)?);
        let _t0: JvmObject = Spliterators::spliterator(this, 16i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "addFirst", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn addFirst(&self, e: E) -> Result<()> {
        let this = self;
        this.add(0i32, e)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addLast", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn addLast(&self, e: E) -> Result<()> {
        let this = self;
        let _t0 = this.add(e)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getFirst", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn getFirst(&self) -> Result<E> {
        let this = self;
        let _t0 = this.isEmpty()?;
        panic!("{}", /* NoSuchElementException::new()? */);
        let _t1 = this.get(0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getLast", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn getLast(&self) -> Result<E> {
        let this = self;
        let _t0 = this.isEmpty()?;
        panic!("{}", /* NoSuchElementException::new()? */);
        let _t1 = this.size()?;
        let _t2 = this.get((_t1).wrapping_sub(1i32))?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "removeFirst", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn removeFirst(&self) -> Result<E> {
        let this = self;
        let _t0 = this.isEmpty()?;
        panic!("{}", /* NoSuchElementException::new()? */);
        let _t1 = this.remove(0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "removeLast", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn removeLast(&self) -> Result<E> {
        let this = self;
        let _t0 = this.isEmpty()?;
        panic!("{}", /* NoSuchElementException::new()? */);
        let _t1 = this.size()?;
        let _t2 = this.remove((_t1).wrapping_sub(1i32))?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "reversed", descriptor = "()Ljava/util/List;", access = "public"))]
    pub fn reversed(&self) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = ReverseOrderListView::of(this, 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "()Ljava/util/List;", access = "public static"))]
    // java: of()Ljava/util/List;
    pub fn of() -> Result<JvmObject> {
        Ok(ImmutableCollections::EMPTY_LIST())
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj(e1: E) -> Result<JvmObject> {
        Ok(ImmutableCollections$List12::new(e1)?)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj(e1: E, e2: E) -> Result<JvmObject> {
        Ok(ImmutableCollections$List12::new(e1, e2)?)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj(e1: E, e2: E, e3: E) -> Result<JvmObject> {
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(3i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        let _t1: JvmObject = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E) -> Result<JvmObject> {
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(4i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        let _t1: JvmObject = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E) -> Result<JvmObject> {
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(5i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        let _t1: JvmObject = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E) -> Result<JvmObject> {
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(6i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        let _t1: JvmObject = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E) -> Result<JvmObject> {
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(7i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        let _t1: JvmObject = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E) -> Result<JvmObject> {
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(8i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        _arr0[7i32 as usize] = e8;
        let _t1: JvmObject = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E) -> Result<JvmObject> {
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(9i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        _arr0[7i32 as usize] = e8;
        _arr0[8i32 as usize] = e9;
        let _t1: JvmObject = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E, e10: E) -> Result<JvmObject> {
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(10i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        _arr0[7i32 as usize] = e8;
        _arr0[8i32 as usize] = e9;
        _arr0[9i32 as usize] = e10;
        let _t1: JvmObject = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "([Ljava/lang/Object;)Ljava/util/List;", access = "public static"))]
    // java: of([Ljava/lang/Object;)Ljava/util/List;
    pub fn of__arr_obj(elements: JvmObject) -> Result<JvmObject> {
        /* TODO: tableswitch default:59 low:0 high:2 */
        let mut list: JvmObject = ImmutableCollections::EMPTY_LIST();
        return Ok(list);
        return Ok(ImmutableCollections$List12::new(elements[0i32 as usize].clone())?);
        return Ok(ImmutableCollections$List12::new(elements[0i32 as usize].clone(), elements[1i32 as usize].clone())?);
        let _t0: JvmObject = ImmutableCollections::listFromArray(elements)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "copyOf", descriptor = "(Ljava/util/Collection;)Ljava/util/List;", access = "public static"))]
    pub fn copyOf(coll: JvmObject) -> Result<JvmObject> {
        let _t0: JvmObject = ImmutableCollections::listCopy(coll)?;
        Ok(_t0)
    }
}
