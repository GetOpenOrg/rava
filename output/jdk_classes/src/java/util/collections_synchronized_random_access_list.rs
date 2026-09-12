#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedRandomAccessList",
    super_class = "java/util/Collections$SynchronizedList",
    interfaces  = "java/util/RandomAccess",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedRandomAccessList<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_SynchronizedRandomAccessList<E> {
    // java: <init>(Ljava/util/List;)V
    // java: <init>(Ljava/util/List;)V
    pub fn new__list(list: Object) -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/Collections$SynchronizedList.<init>:(Ljava/util/List;)V */
        Ok(this)
    }

    // java: <init>(Ljava/util/List;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/List;Ljava/lang/Object;)V
    pub fn new__list_obj(list: Object, mutex: Object) -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/Collections$SynchronizedList.<init>:(Ljava/util/List;Ljava/lang/Object;)V */
        Ok(this)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.list.get().subList(fromIndex, toIndex)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedRandomAccessList = Collections_SynchronizedRandomAccessList::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        let this = self;
        Ok(Collections_SynchronizedList::new(this.list.get())?)
    }
}
