#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayList$SubList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/RandomAccess",
    access      = "",
    source      = "ArrayList.java",
))]
pub struct ArrayList_SubList<E> {
    #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/ArrayList;", access = "private final"))]
    pub root: Field<Object>,
    #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/ArrayList$SubList;", access = "private final"))]
    pub parent: Field<Object>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private"))]
    pub size: Field<i32>,
}

impl<E: Clone + 'static> ArrayList_SubList<E> {
    // java: <init>(Ljava/util/ArrayList;II)V
    // java: <init>(Ljava/util/ArrayList;II)V
    pub fn new__arrayl_i_i(root: Object, fromIndex: i32, toIndex: i32) -> Result<Self> {
        let this = Self { root: Field::new(Default::default()), parent: Field::new(Default::default()), offset: Field::new(0), size: Field::new(0) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        this.root.set(root);
        /* TODO: aconst_null  */
        todo!("stack underflow").parent.set(this);
        this.offset.set(fromIndex);
        this.size.set((toIndex).wrapping_sub(fromIndex));
        this.modCount.set(root.modCount.get());
        Ok(this)
    }

    // java: <init>(Ljava/util/ArrayList$SubList;II)V
    // java: <init>(Ljava/util/ArrayList$SubList;II)V
    pub fn new__arrayl_i_i(parent: Object, fromIndex: i32, toIndex: i32) -> Result<Self> {
        let this = Self { root: Field::new(Default::default()), parent: Field::new(Default::default()), offset: Field::new(0), size: Field::new(0) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        this.root.set(parent.root.get());
        this.parent.set(parent);
        this.offset.set((parent.offset.get()).wrapping_add(fromIndex));
        this.size.set((toIndex).wrapping_sub(fromIndex));
        this.modCount.set(parent.modCount.get());
        Ok(this)
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        this.checkForComodification()?;
        let _t1 = this.root.get().elementData((this.offset.get()).wrapping_add(index))?;
        let mut oldValue: Object = _t1;
        this.root.get().elementData.get()[(this.offset.get()).wrapping_add(index) as usize] = element;
        Ok(oldValue)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        this.checkForComodification()?;
        let _t1 = this.root.get().elementData((this.offset.get()).wrapping_add(index))?;
        Ok(_t1)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        this.checkForComodification()?;
        Ok(this.size.get())
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: E) -> Result<()> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        this.checkForComodification()?;
        this.root.get().add((this.offset.get()).wrapping_add(index), element)?;
        this.updateSizeAndModCount(1i32)?;
        Ok(())
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        this.checkForComodification()?;
        let _t1 = this.root.get().remove((this.offset.get()).wrapping_add(index))?;
        let mut result: Object = _t1;
        this.updateSizeAndModCount(-1i32)?;
        Ok(result)
    }

    // java: removeRange(II)V
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        let this = self;
        this.checkForComodification()?;
        this.root.get().removeRange((this.offset.get()).wrapping_add(fromIndex), (this.offset.get()).wrapping_add(toIndex))?;
        this.updateSizeAndModCount((fromIndex).wrapping_sub(toIndex))?;
        Ok(())
    }

    // java: addAll(Ljava/util/Collection;)Z
    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.addAll(this.size.get(), c)?;
        Ok(_t0)
    }

    // java: addAll(ILjava/util/Collection;)Z
    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, index: i32, c: Object) -> Result<bool> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        let _t0 = c.size()?;
        let mut cSize: i32 = _t0;
        return Ok(0i32);
        this.checkForComodification()?;
        let _t1 = this.root.get().addAll((this.offset.get()).wrapping_add(index), c)?;
        this.updateSizeAndModCount(cSize)?;
        Ok(1i32)
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        this.root.get().replaceAllRange(operator, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        Ok(())
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.batchRemove(c, 0i32)?;
        Ok(_t0)
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.batchRemove(c, 1i32)?;
        Ok(_t0)
    }

    // java: batchRemove(Ljava/util/Collection;Z)Z
    pub fn batchRemove(&self, c: Object, complement: bool) -> Result<bool> {
        let this = self;
        this.checkForComodification()?;
        let mut oldSize: i32 = this.root.get().size.get();
        let _t0 = this.root.get().batchRemove(c, complement, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut modified: i32 = _t0;
        this.updateSizeAndModCount((this.root.get().size.get()).wrapping_sub(oldSize))?;
        Ok(modified)
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        let this = self;
        this.checkForComodification()?;
        let mut oldSize: i32 = this.root.get().size.get();
        let _t0 = this.root.get().removeIf(filter, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut modified: i32 = _t0;
        this.updateSizeAndModCount((this.root.get().size.get()).wrapping_sub(oldSize))?;
        Ok(modified)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        this.checkForComodification()?;
        let _t0: Vec<Object> = Arrays::copyOfRange(&this.root.get().elementData.get(), this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        Ok(_t0)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        this.checkForComodification()?;
        let _t0 = a.getClass()?;
        let _t1: Vec<Object> = Arrays::copyOfRange(&this.root.get().elementData.get(), this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()), _t0)?;
        return Ok(_t1);
        System::arraycopy(&this.root.get().elementData.get(), this.offset.get(), &a, 0i32, this.size.get())?;
        /* TODO: aconst_null  */
        this.size.get()[a as usize] = this.size.get();
        Ok(a)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let _t0 = this.root.get().equalsRange(o, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut equal: i32 = _t0;
        this.checkForComodification()?;
        Ok(equal)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.root.get().hashCodeRange(this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut hash: i32 = _t0;
        this.checkForComodification()?;
        Ok(hash)
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.root.get().indexOfRange(o, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut index: i32 = _t0;
        this.checkForComodification()?;
        Ok(-1i32)
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.root.get().lastIndexOfRange(o, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut index: i32 = _t0;
        this.checkForComodification()?;
        Ok(-1i32)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.indexOf(o)?;
        Ok(_t0>=0i32)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.listIterator()?;
        Ok(_t0)
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator(&self, index: i32) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        this.rangeCheckForAdd(index)?;
        Ok(ArrayList_SubList_1::new(this, index)?)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        ArrayList_SubList::subListRangeCheck(fromIndex, toIndex, this.size.get())?;
        Ok(ArrayList_SubList::new(this, fromIndex, toIndex)?)
    }

    // java: rangeCheckForAdd(I)V
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        let this = self;
        let _t0 = this.outOfBoundsMsg(index)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: outOfBoundsMsg(I)Ljava/lang/String;
    pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Index:"))?;
        String::new().append(&index)?;
        String::new().append(&String::from(", Size:"))?;
        String::new().append(&this.size.get())?;
        Ok(String::new())
    }

    // java: checkForComodification()V
    pub fn checkForComodification(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: updateSizeAndModCount(I)V
    pub fn updateSizeAndModCount(&self, sizeChange: i32) -> Result<()> {
        let this = self;
        let mut slist: ArrayList_SubList = this;
        slist.size.set((slist.size.get()).wrapping_add(sizeChange));
        slist.modCount.set(this.root.get().modCount.get());
        slist = slist.parent.get();
        Ok(())
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        Ok(ArrayList_SubList_2::new(this)?)
    }
}
