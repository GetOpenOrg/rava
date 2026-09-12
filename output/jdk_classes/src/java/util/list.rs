#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/List",
    super_class = "java/lang/Object",
    interfaces  = "java/util/SequencedCollection",
    access      = "public abstract",
    source      = "List.java",
))]
pub struct List<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> List<E> {
    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/List.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/List.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.contains")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/List.iterator")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/List.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, arg0: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/List.toArray")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.add")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.remove")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.containsAll")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.addAll")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, arg0: i32, arg1: Object) -> Result<bool> {
        todo!("abstract java/util/List.addAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.removeAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.retainAll")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(operator)?;
        let _t1 = this.listIterator()?;
        let mut li: Object = _t1;
        loop {
            let _t0 = li.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = li.next()?;
            let _t1 = operator.apply(_t0)?;
            li.set(_t1)?;
        }
        Ok(())
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        let _t0 = this.toArray()?;
        let mut a: Vec<Object> = _t0;
        Arrays::sort__arr_obj_compar(&a, c)?;
        let _t1 = this.listIterator()?;
        let mut i: Object = _t1;
        let mut local_4: Vec<Object> = a;
        let mut local_5: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut e: Object = local_4[local_6 as usize].clone();
            let _t0 = i.next()?;
            i.set(e)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/List.clear")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/List.hashCode")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/util/List.get")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, arg0: i32, arg1: Object) -> Result<Object> {
        todo!("abstract java/util/List.set")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, arg0: i32, arg1: Object) -> Result<()> {
        todo!("abstract java/util/List.add")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove__i(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/util/List.remove")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, arg0: Object) -> Result<i32> {
        todo!("abstract java/util/List.indexOf")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, arg0: Object) -> Result<i32> {
        todo!("abstract java/util/List.lastIndexOf")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        todo!("abstract java/util/List.listIterator")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/util/List.listIterator")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, arg0: i32, arg1: i32) -> Result<Object> {
        todo!("abstract java/util/List.subList")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        return Ok(AbstractList_RandomAccessSpliterator::new(this)?);
        let _t0: Object = Spliterators::spliterator__coll_i(this, 16i32)?;
        Ok(_t0)
    }

    // java: addFirst(Ljava/lang/Object;)V
    pub fn addFirst(&self, e: E) -> Result<()> {
        let this = self;
        this.add__i_obj(0i32, e)?;
        Ok(())
    }

    // java: addLast(Ljava/lang/Object;)V
    pub fn addLast(&self, e: E) -> Result<()> {
        let this = self;
        let _t0 = this.add__obj(e)?;
        Ok(())
    }

    // java: getFirst()Ljava/lang/Object;
    pub fn getFirst(&self) -> Result<E> {
        let this = self;
        let _t0 = this.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.get(0i32)?;
        Ok(_t1)
    }

    // java: getLast()Ljava/lang/Object;
    pub fn getLast(&self) -> Result<E> {
        let this = self;
        let _t0 = this.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.size()?;
        let _t2 = this.get((_t1).wrapping_sub(1i32))?;
        Ok(_t2)
    }

    // java: removeFirst()Ljava/lang/Object;
    pub fn removeFirst(&self) -> Result<E> {
        let this = self;
        let _t0 = this.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.remove__i(0i32)?;
        Ok(_t1)
    }

    // java: removeLast()Ljava/lang/Object;
    pub fn removeLast(&self) -> Result<E> {
        let this = self;
        let _t0 = this.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.size()?;
        let _t2 = this.remove__i((_t1).wrapping_sub(1i32))?;
        Ok(_t2)
    }

    // java: reversed()Ljava/util/List;
    pub fn reversed(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = ReverseOrderListView::of(this, 1i32)?;
        Ok(_t0)
    }

    // java: of()Ljava/util/List;
    // java: of()Ljava/util/List;
    pub fn of() -> Result<Object> {
        Ok(ImmutableCollections::EMPTY_LIST())
    }

    // java: of(Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj(e1: E) -> Result<Object> {
        Ok(ImmutableCollections_List12::new(e1)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj(e1: E, e2: E) -> Result<Object> {
        Ok(ImmutableCollections_List12::new(e1, e2)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj(e1: E, e2: E, e3: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(3i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        let _t1: Object = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(4i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        let _t1: Object = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(5i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        let _t1: Object = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(6i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        let _t1: Object = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(7i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        let _t1: Object = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(8i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        _arr0[7i32 as usize] = e8;
        let _t1: Object = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(9i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        _arr0[7i32 as usize] = e8;
        _arr0[8i32 as usize] = e9;
        let _t1: Object = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E, e10: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(10i32 as usize);
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
        let _t1: Object = ImmutableCollections::listFromTrustedArray(&_arr0)?;
        Ok(_t1)
    }

    // java: of([Ljava/lang/Object;)Ljava/util/List;
    // java: of([Ljava/lang/Object;)Ljava/util/List;
    pub fn of__arr_obj(elements: &[Object]) -> Result<Object> {
        /* TODO: tableswitch default:59 low:0 high:2 */
        let mut list: Object = ImmutableCollections::EMPTY_LIST();
        return Ok(list);
        return Ok(ImmutableCollections_List12::new(elements[0i32 as usize].clone())?);
        return Ok(ImmutableCollections_List12::new(elements[0i32 as usize].clone(), elements[1i32 as usize].clone())?);
        let _t0: Object = ImmutableCollections::listFromArray(&elements)?;
        Ok(_t0)
    }

    // java: copyOf(Ljava/util/Collection;)Ljava/util/List;
    pub fn copyOf(coll: Object) -> Result<Object> {
        let _t0: Object = ImmutableCollections::listCopy(coll)?;
        Ok(_t0)
    }
}
