#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedNavigableSet",
    super_class = "java/util/Collections$SynchronizedSortedSet",
    interfaces  = "java/util/NavigableSet",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedNavigableSet<E> {
    #[cfg_attr(any(), java_field(name = "ns", descriptor = "Ljava/util/NavigableSet;", access = "private final"))]
    pub ns: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SynchronizedNavigableSet<E> {
    // java: <init>(Ljava/util/NavigableSet;)V
    // java: <init>(Ljava/util/NavigableSet;)V
    pub fn new__naviga(s: Object) -> Result<Self> {
        let this = Self { ns: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedSortedSet.<init>:(Ljava/util/SortedSet;)V */
        this.ns.set(s);
        Ok(this)
    }

    // java: <init>(Ljava/util/NavigableSet;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/NavigableSet;Ljava/lang/Object;)V
    pub fn new__naviga_obj(s: Object, mutex: Object) -> Result<Self> {
        let this = Self { ns: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedSortedSet.<init>:(Ljava/util/SortedSet;Ljava/lang/Object;)V */
        this.ns.set(s);
        Ok(this)
    }

    // java: lower(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lower(&self, e: E) -> Result<E> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().lower(e)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: floor(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floor(&self, e: E) -> Result<E> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().floor(e)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: ceiling(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceiling(&self, e: E) -> Result<E> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().ceiling(e)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: higher(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higher(&self, e: E) -> Result<E> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().higher(e)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: pollFirst()Ljava/lang/Object;
    pub fn pollFirst(&self) -> Result<E> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().pollFirst()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: pollLast()Ljava/lang/Object;
    pub fn pollLast(&self) -> Result<E> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().pollLast()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: descendingSet()Ljava/util/NavigableSet;
    pub fn descendingSet(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().descendingSet()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: descendingIterator()Ljava/util/Iterator;
    pub fn descendingIterator(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.descendingSet()?;
        let _t1 = _t0.iterator()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableSet;
    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn subSet__obj_obj(&self, fromElement: E, toElement: E) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().subSet(fromElement, 1i32, toElement, 0i32)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    // java: headSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn headSet__obj(&self, toElement: E) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().headSet(toElement, 0i32)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    // java: tailSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn tailSet__obj(&self, fromElement: E) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().tailSet(fromElement, 1i32)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: subSet(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;
    // java: subSet(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn subSet__obj_z_obj_z(&self, fromElement: E, fromInclusive: bool, toElement: E, toInclusive: bool) -> Result<Object> {
        let this = self;
        let mut local_5: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().subSet(fromElement, fromInclusive, toElement, toInclusive)?;
        /* TODO: monitorexit  */
        return Ok(local_5);
        let mut local_6: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: headSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    // java: headSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn headSet__obj_z(&self, toElement: E, inclusive: bool) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().headSet(toElement, inclusive)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: tailSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    // java: tailSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn tailSet__obj_z(&self, fromElement: E, inclusive: bool) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.ns.get().tailSet(fromElement, inclusive)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
