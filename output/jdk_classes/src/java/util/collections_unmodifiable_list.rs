#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$UnmodifiableList",
    super_class = "java/util/Collections$UnmodifiableCollection",
    interfaces  = "java/util/List",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_UnmodifiableList<E> {
    #[cfg_attr(any(), java_field(name = "list", descriptor = "Ljava/util/List;", access = "final"))]
    pub list: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_UnmodifiableList<E> {
    // java: <init>(Ljava/util/List;)V
    pub fn new(list: Object) -> Result<Self> {
        let this = Self { list: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$UnmodifiableCollection.<init>:(Ljava/util/Collection;)V */
        this.list.set(list);
        Ok(this)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.list.get().equals(o)?;
        Ok(_t0!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.list.get().hashCode()?;
        Ok(_t0)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0 = this.list.get().get(index)?;
        Ok(_t0)
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: E) -> Result<()> {
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
        let _t0 = this.list.get().indexOf(o)?;
        Ok(_t0)
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.list.get().lastIndexOf(o)?;
        Ok(_t0)
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
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
        Ok(Collections_UnmodifiableList_1::new(this, index)?)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.list.get().subList(fromIndex, toIndex)?;
        Ok(Collections_UnmodifiableList::new(_t0)?)
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        let this = self;
        Ok(this)
    }
}
