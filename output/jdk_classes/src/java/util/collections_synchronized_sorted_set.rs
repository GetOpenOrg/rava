#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedSortedSet",
    super_class = "java/util/Collections$SynchronizedSet",
    interfaces  = "java/util/SortedSet",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedSortedSet<E> {
    #[cfg_attr(any(), java_field(name = "ss", descriptor = "Ljava/util/SortedSet;", access = "private final"))]
    pub ss: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SynchronizedSortedSet<E> {
    // java: <init>(Ljava/util/SortedSet;)V
    // java: <init>(Ljava/util/SortedSet;)V
    pub fn new__sorted(s: Object) -> Result<Self> {
        let this = Self { ss: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedSet.<init>:(Ljava/util/Set;)V */
        this.ss.set(s);
        Ok(this)
    }

    // java: <init>(Ljava/util/SortedSet;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/SortedSet;Ljava/lang/Object;)V
    pub fn new__sorted_obj(s: Object, mutex: Object) -> Result<Self> {
        let this = Self { ss: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedSet.<init>:(Ljava/util/Set;Ljava/lang/Object;)V */
        this.ss.set(s);
        Ok(this)
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ss.get().comparator()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn subSet(&self, fromElement: E, toElement: E) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ss.get().subSet(fromElement, toElement)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedSortedSet = Collections_SynchronizedSortedSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn headSet(&self, toElement: E) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ss.get().headSet(toElement)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Collections_SynchronizedSortedSet = Collections_SynchronizedSortedSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn tailSet(&self, fromElement: E) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ss.get().tailSet(fromElement)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Collections_SynchronizedSortedSet = Collections_SynchronizedSortedSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: first()Ljava/lang/Object;
    pub fn first(&self) -> Result<E> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ss.get().first()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: last()Ljava/lang/Object;
    pub fn last(&self) -> Result<E> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ss.get().last()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
