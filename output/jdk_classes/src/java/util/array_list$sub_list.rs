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
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/ArrayList;II)V", access = "public"))]
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

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/ArrayList$SubList;II)V", access = "private"))]
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

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        this.checkForComodification()?;
        let _t1 = this.root.get().elementData((this.offset.get()).wrapping_add(index))?;
        let mut oldValue: Object = _t1;
        this.root.get().elementData.get()[(this.offset.get()).wrapping_add(index) as usize] = element;
        Ok(oldValue)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        this.checkForComodification()?;
        let _t1 = this.root.get().elementData((this.offset.get()).wrapping_add(index))?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        this.checkForComodification()?;
        Ok(this.size.get())
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public"))]
    pub fn add(&self, index: i32, element: E) -> Result<()> {
        let this = self;
        this.rangeCheckForAdd(index)?;
        this.checkForComodification()?;
        this.root.get().add((this.offset.get()).wrapping_add(index), element)?;
        this.updateSizeAndModCount(1i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public"))]
    pub fn remove(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex(index, this.size.get())?;
        this.checkForComodification()?;
        let _t1 = this.root.get().remove((this.offset.get()).wrapping_add(index))?;
        let mut result: Object = _t1;
        this.updateSizeAndModCount(-1i32)?;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "removeRange", descriptor = "(II)V", access = "protected"))]
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        let this = self;
        this.checkForComodification()?;
        this.root.get().removeRange((this.offset.get()).wrapping_add(fromIndex), (this.offset.get()).wrapping_add(toIndex))?;
        this.updateSizeAndModCount((fromIndex).wrapping_sub(toIndex))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public"))]
    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.addAll(this.size.get(), c)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public"))]
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

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public"))]
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        this.root.get().replaceAllRange(operator, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public"))]
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.batchRemove(c, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public"))]
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.batchRemove(c, 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "batchRemove", descriptor = "(Ljava/util/Collection;Z)Z", access = "private"))]
    pub fn batchRemove(&self, c: Object, complement: bool) -> Result<bool> {
        let this = self;
        this.checkForComodification()?;
        let mut oldSize: i32 = this.root.get().size.get();
        let _t0 = this.root.get().batchRemove(c, complement, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut modified: i32 = _t0;
        this.updateSizeAndModCount((this.root.get().size.get()).wrapping_sub(oldSize))?;
        Ok(modified)
    }

    #[cfg_attr(any(), java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public"))]
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        let this = self;
        this.checkForComodification()?;
        let mut oldSize: i32 = this.root.get().size.get();
        let _t0 = this.root.get().removeIf(filter, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut modified: i32 = _t0;
        this.updateSizeAndModCount((this.root.get().size.get()).wrapping_sub(oldSize))?;
        Ok(modified)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public"))]
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        this.checkForComodification()?;
        let _t0: Vec<Object> = Arrays::copyOfRange(&this.root.get().elementData.get(), this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public"))]
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

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let _t0 = this.root.get().equalsRange(o, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut equal: i32 = _t0;
        this.checkForComodification()?;
        Ok(equal)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.root.get().hashCodeRange(this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut hash: i32 = _t0;
        this.checkForComodification()?;
        Ok(hash)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.root.get().indexOfRange(o, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut index: i32 = _t0;
        this.checkForComodification()?;
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.root.get().lastIndexOfRange(o, this.offset.get(), (this.offset.get()).wrapping_add(this.size.get()))?;
        let mut index: i32 = _t0;
        this.checkForComodification()?;
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.indexOf(o)?;
        Ok(_t0>=0i32)
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public"))]
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.listIterator()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public"))]
    pub fn listIterator(&self, index: i32) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        this.rangeCheckForAdd(index)?;
        Ok(ArrayList_SubList_1::new(this, index)?)
    }

    #[cfg_attr(any(), java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public"))]
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        ArrayList$SubList::subListRangeCheck(fromIndex, toIndex, this.size.get())?;
        Ok(ArrayList_SubList::new(this, fromIndex, toIndex)?)
    }

    #[cfg_attr(any(), java_method(name = "rangeCheckForAdd", descriptor = "(I)V", access = "private"))]
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        let this = self;
        let _t0 = this.outOfBoundsMsg(index)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "outOfBoundsMsg", descriptor = "(I)Ljava/lang/String;", access = "private"))]
    pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Index:"))?;
        String::new().append(&index)?;
        String::new().append(&String::from(", Size:"))?;
        String::new().append(&this.size.get())?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "checkForComodification", descriptor = "()V", access = "private"))]
    pub fn checkForComodification(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "updateSizeAndModCount", descriptor = "(I)V", access = "private"))]
    pub fn updateSizeAndModCount(&self, sizeChange: i32) -> Result<()> {
        let this = self;
        let mut slist: java/util/ArrayList$SubList = this;
        slist.size.set((slist.size.get()).wrapping_add(sizeChange));
        slist.modCount.set(this.root.get().modCount.get());
        slist = slist.parent.get();
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public"))]
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        Ok(ArrayList_SubList_2::new(this)?)
    }
}
