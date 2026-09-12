#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedList",
    super_class = "java/util/Collections$SynchronizedCollection",
    interfaces  = "java/util/List",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedList<E> {
    #[cfg_attr(any(), java_field(name = "list", descriptor = "Ljava/util/List;", access = "final"))]
    pub list: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SynchronizedList<E> {
    // java: <init>(Ljava/util/List;)V
    // java: <init>(Ljava/util/List;)V
    pub fn new__list(list: Object) -> Result<Self> {
        let this = Self { list: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedCollection.<init>:(Ljava/util/Collection;)V */
        this.list.set(list);
        Ok(this)
    }

    // java: <init>(Ljava/util/List;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/List;Ljava/lang/Object;)V
    pub fn new__list_obj(list: Object, mutex: Object) -> Result<Self> {
        let this = Self { list: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedCollection.<init>:(Ljava/util/Collection;Ljava/lang/Object;)V */
        this.list.set(list);
        Ok(this)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().equals(o)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().hashCode()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: i32 = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().get(index)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().set(index, element)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: E) -> Result<()> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.list.get().add(index, element)?;
        /* TODO: monitorexit  */
        let mut local_4: Object = local_3;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<E> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().remove(index)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().indexOf(o)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: i32 = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().lastIndexOf(o)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: i32 = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().addAll(index, c)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: listIterator()Ljava/util/ListIterator;
    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.list.get().listIterator()?;
        Ok(_t0)
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.list.get().listIterator(index)?;
        Ok(_t0)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().subList(fromIndex, toIndex)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedList = Collections_SynchronizedList::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.list.get().replaceAll(operator)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.list.get().sort(c)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        let this = self;
        Ok(this)
    }
}
