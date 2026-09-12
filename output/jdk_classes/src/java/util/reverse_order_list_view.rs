#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ReverseOrderListView",
    super_class = "java/lang/Object",
    interfaces  = "java/util/List",
    access      = "",
    source      = "ReverseOrderListView.java",
))]
pub struct ReverseOrderListView<E> {
    #[cfg_attr(any(), java_field(name = "base", descriptor = "Ljava/util/List;", access = "final"))]
    pub base: Field<Object>,
    #[cfg_attr(any(), java_field(name = "modifiable", descriptor = "Z", access = "final"))]
    pub modifiable: Field<bool>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ReverseOrderListView<E> {
    // java: of(Ljava/util/List;Z)Ljava/util/List;
    pub fn of(list: Object, modifiable: bool) -> Result<Object> {
        let mut rolv: Object = list;
        return Ok(rolv.base.get());
        return Ok(ReverseOrderListView_Rand::new(list, modifiable)?);
        Ok(ReverseOrderListView::new(list, modifiable)?)
    }

    // java: <init>(Ljava/util/List;Z)V
    pub fn new(list: Object, modifiable: bool) -> Result<Self> {
        let this = Self { base: Field::new(Default::default()), modifiable: Field::new(false), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.base.set(list);
        this.modifiable.set(modifiable);
        Ok(this)
    }

    // java: checkModifiable()V
    pub fn checkModifiable(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0 = this.iterator()?;
        let mut local_2: Object = _t0;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut e: Object = _t0;
            action.accept(e)?;
        }
        Ok(())
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        Ok(ReverseOrderListView_DescendingIterator::new(this)?)
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Spliterators::spliterator__coll_i(this, 16i32)?;
        Ok(_t0)
    }

    // java: add(Ljava/lang/Object;)Z
    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, e: E) -> Result<bool> {
        let this = self;
        this.checkModifiable()?;
        this.base.get().add(0i32, e)?;
        Ok(1i32)
    }

    // java: addAll(Ljava/util/Collection;)Z
    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, c: Object) -> Result<bool> {
        let this = self;
        this.checkModifiable()?;
        let _t0 = c.toArray()?;
        let mut adds: Vec<Object> = _t0;
        return Ok(0i32);
        let _t1: Vec<Object> = ArraysSupport::reverse(&adds)?;
        let _t2: Object = Arrays::asList(&_t1)?;
        let _t3 = this.base.get().addAll(0i32, _t2)?;
        Ok(1i32)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.checkModifiable()?;
        this.base.get().clear()?;
        Ok(())
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.base.get().contains(o)?;
        Ok(_t0)
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.base.get().containsAll(c)?;
        Ok(_t0)
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

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.base.get().isEmpty()?;
        Ok(_t0)
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.spliterator()?;
        let _t1: Object = StreamSupport::stream(_t0, 1i32)?;
        Ok(_t1)
    }

    // java: remove(Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, o: Object) -> Result<bool> {
        let this = self;
        this.checkModifiable()?;
        let _t0 = this.iterator()?;
        let mut it: Object = _t0;
        let _t1 = it.hasNext()?;
        let _t2 = it.next()?;
        it.remove()?;
        return Ok(1i32);
        let _t3 = it.hasNext()?;
        let _t4 = it.next()?;
        let _t5 = o.equals(_t4)?;
        it.remove()?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        let this = self;
        this.checkModifiable()?;
        let _t0: Object = Objects::requireNonNull__obj(c)?;
        let mut modified: i32 = 0i32;
        let _t1 = this.iterator()?;
        let mut it: Object = _t1;
        loop {
            let _t0 = it.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = it.next()?;
            let _t1 = c.contains(_t0)?;
            it.remove()?;
            modified = 1i32;
        }
        Ok(modified)
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        let this = self;
        this.checkModifiable()?;
        let _t0: Object = Objects::requireNonNull__obj(c)?;
        let mut modified: i32 = 0i32;
        let _t1 = this.iterator()?;
        let mut it: Object = _t1;
        loop {
            let _t0 = it.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = it.next()?;
            let _t1 = c.contains(_t0)?;
            it.remove()?;
            modified = 1i32;
        }
        Ok(modified)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.base.get().size()?;
        Ok(_t0)
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.spliterator()?;
        let _t1: Object = StreamSupport::stream(_t0, 0i32)?;
        Ok(_t1)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.base.get().toArray()?;
        let _t1: Vec<Object> = ArraysSupport::reverse(&_t0)?;
        Ok(_t1)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = ArraysSupport::toArrayReversed(this.base.get(), &a)?;
        Ok(_t0)
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, generator: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.base.get().toArray(generator)?;
        let _t1: Vec<Object> = ArraysSupport::reverse(&_t0)?;
        Ok(_t1)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.iterator()?;
        let mut it: Object = _t0;
        let _t1 = it.hasNext()?;
        return Ok(String::from("[]"));
        let mut sb: String = String::new();
        sb.append(&91i32)?;
        let _t2 = it.next()?;
        let mut e: Object = _t2;
        String::from("(this Collection)").append(&e)?;
        let _ = String::from("(this Collection)");
        let _t3 = it.hasNext()?;
        sb.append(&93i32)?;
        return Ok(sb);
        sb.append(&44i32)?;
        sb.append(&32i32)?;
    }

    // java: add(ILjava/lang/Object;)V
    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, index: i32, element: E) -> Result<()> {
        let this = self;
        this.checkModifiable()?;
        let _t0 = this.base.get().size()?;
        let mut size: i32 = _t0;
        ReverseOrderListView::checkClosedRange(index, size)?;
        this.base.get().add((size).wrapping_sub(index), element)?;
        Ok(())
    }

    // java: addAll(ILjava/util/Collection;)Z
    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, index: i32, c: Object) -> Result<bool> {
        let this = self;
        this.checkModifiable()?;
        let _t0 = this.base.get().size()?;
        let mut size: i32 = _t0;
        ReverseOrderListView::checkClosedRange(index, size)?;
        let _t1 = c.toArray()?;
        let mut adds: Vec<Object> = _t1;
        return Ok(0i32);
        let _t2: Vec<Object> = ArraysSupport::reverse(&adds)?;
        let _t3: Object = Arrays::asList(&_t2)?;
        let _t4 = this.base.get().addAll((size).wrapping_sub(index), _t3)?;
        Ok(1i32)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, i: i32) -> Result<E> {
        let this = self;
        let _t0 = this.base.get().size()?;
        let mut size: i32 = _t0;
        let _t1: i32 = Objects::checkIndex__i_i(i, size)?;
        let _t2 = this.base.get().get(((size).wrapping_sub(i)).wrapping_sub(1i32))?;
        Ok(_t2)
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.base.get().lastIndexOf(o)?;
        let mut i: i32 = _t0;
        let _t1 = this.base.get().size()?;
        Ok(((_t1).wrapping_sub(i)).wrapping_sub(1i32))
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.base.get().indexOf(o)?;
        let mut i: i32 = _t0;
        let _t1 = this.base.get().size()?;
        Ok(((_t1).wrapping_sub(i)).wrapping_sub(1i32))
    }

    // java: listIterator()Ljava/util/ListIterator;
    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.base.get().size()?;
        Ok(ReverseOrderListView_DescendingListIterator::new(this, _t0, 0i32)?)
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.base.get().size()?;
        let mut size: i32 = _t0;
        ReverseOrderListView::checkClosedRange(index, size)?;
        Ok(ReverseOrderListView_DescendingListIterator::new(this, size, index)?)
    }

    // java: remove(I)Ljava/lang/Object;
    // java: remove(I)Ljava/lang/Object;
    pub fn remove__i(&self, index: i32) -> Result<E> {
        let this = self;
        this.checkModifiable()?;
        let _t0 = this.base.get().size()?;
        let mut size: i32 = _t0;
        let _t1: i32 = Objects::checkIndex__i_i(index, size)?;
        let _t2 = this.base.get().remove(((size).wrapping_sub(index)).wrapping_sub(1i32))?;
        Ok(_t2)
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        let this = self;
        this.checkModifiable()?;
        let _t0 = this.base.get().removeIf(filter)?;
        Ok(_t0)
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        this.checkModifiable()?;
        this.base.get().replaceAll(operator)?;
        Ok(())
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        this.checkModifiable()?;
        let _t0: Object = Collections::reverseOrder__compar(c)?;
        this.base.get().sort(_t0)?;
        Ok(())
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        this.checkModifiable()?;
        let _t0 = this.base.get().size()?;
        let mut size: i32 = _t0;
        let _t1: i32 = Objects::checkIndex__i_i(index, size)?;
        let _t2 = this.base.get().set(((size).wrapping_sub(index)).wrapping_sub(1i32), element)?;
        Ok(_t2)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.base.get().size()?;
        let mut size: i32 = _t0;
        let _t1: i32 = Objects::checkFromToIndex__i_i_i(fromIndex, toIndex, size)?;
        let _t2 = this.base.get().subList((size).wrapping_sub(toIndex), (size).wrapping_sub(fromIndex))?;
        Ok(ReverseOrderListView::new(_t2, this.modifiable.get())?)
    }

    // java: checkClosedRange(II)V
    pub fn checkClosedRange(index: i32, size: i32) -> Result<()> {
        String::new().append(&String::from("Index:"))?;
        String::new().append(&index)?;
        String::new().append(&String::from(", Size:"))?;
        String::new().append(&size)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
