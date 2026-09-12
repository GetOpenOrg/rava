#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$SubList",
    super_class = "java/util/AbstractList",
    interfaces  = "",
    access      = "",
    source      = "AbstractList.java",
))]
pub struct AbstractList_SubList<E> {
    #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/AbstractList;", access = "private final"))]
    pub root: Field<Object>,
    #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/AbstractList$SubList;", access = "private final"))]
    pub parent: Field<Object>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "protected"))]
    pub size: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> AbstractList_SubList<E> {
    // java: <init>(Ljava/util/AbstractList;II)V
    // java: <init>(Ljava/util/AbstractList;II)V
    pub fn new__abstra_i_i(root: Object, fromIndex: i32, toIndex: i32) -> Result<Self> {
        let this = Self { root: Field::new(Default::default()), parent: Field::new(Default::default()), offset: Field::new(0), size: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        this.root.set(root);
        /* TODO: aconst_null  */
        todo!("stack underflow").parent.set(this);
        this.offset.set(fromIndex);
        this.size.set((toIndex).wrapping_sub(fromIndex));
        this.modCount.set(root.modCount.get());
        Ok(this)
    }

    // java: <init>(Ljava/util/AbstractList$SubList;II)V
    // java: <init>(Ljava/util/AbstractList$SubList;II)V
    pub fn new__abstra_i_i(parent: Object, fromIndex: i32, toIndex: i32) -> Result<Self> {
        let this = Self { root: Field::new(Default::default()), parent: Field::new(Default::default()), offset: Field::new(0), size: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        this.root.set(parent.root.get());
        this.parent.set(parent);
        this.offset.set((parent.offset.get()).wrapping_add(fromIndex));
        this.size.set((toIndex).wrapping_sub(fromIndex));
        this.modCount.set(this.root.get().modCount.get());
        Ok(this)
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex__i_i(index, this.size.get())?;
        this.checkForComodification()?;
        let _t1 = this.root.get().set((this.offset.get()).wrapping_add(index), element)?;
        Ok(_t1)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex__i_i(index, this.size.get())?;
        this.checkForComodification()?;
        let _t1 = this.root.get().get((this.offset.get()).wrapping_add(index))?;
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
        let _t0: i32 = Objects::checkIndex__i_i(index, this.size.get())?;
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
        Ok(AbstractList_SubList_1::new(this, index)?)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        AbstractList_SubList::subListRangeCheck(fromIndex, toIndex, this.size.get())?;
        Ok(AbstractList_SubList::new(this, fromIndex, toIndex)?)
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
        let mut slist: AbstractList_SubList = this;
        slist.size.set((slist.size.get()).wrapping_add(sizeChange));
        slist.modCount.set(this.root.get().modCount.get());
        slist = slist.parent.get();
        Ok(())
    }
}
