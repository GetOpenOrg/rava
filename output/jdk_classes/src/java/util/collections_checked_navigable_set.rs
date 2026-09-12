#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedNavigableSet",
    super_class = "java/util/Collections$CheckedSortedSet",
    interfaces  = "java/util/NavigableSet,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedNavigableSet<E> {
    #[cfg_attr(any(), java_field(name = "ns", descriptor = "Ljava/util/NavigableSet;", access = "private final"))]
    pub ns: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CheckedNavigableSet<E> {
    // java: <init>(Ljava/util/NavigableSet;Ljava/lang/Class;)V
    pub fn new(s: Object, type_: Object) -> Result<Self> {
        let this = Self { ns: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$CheckedSortedSet.<init>:(Ljava/util/SortedSet;Ljava/lang/Class;)V */
        this.ns.set(s);
        Ok(this)
    }

    // java: lower(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lower(&self, e: E) -> Result<E> {
        let this = self;
        let _t0 = this.ns.get().lower(e)?;
        Ok(_t0)
    }

    // java: floor(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floor(&self, e: E) -> Result<E> {
        let this = self;
        let _t0 = this.ns.get().floor(e)?;
        Ok(_t0)
    }

    // java: ceiling(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceiling(&self, e: E) -> Result<E> {
        let this = self;
        let _t0 = this.ns.get().ceiling(e)?;
        Ok(_t0)
    }

    // java: higher(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higher(&self, e: E) -> Result<E> {
        let this = self;
        let _t0 = this.ns.get().higher(e)?;
        Ok(_t0)
    }

    // java: pollFirst()Ljava/lang/Object;
    pub fn pollFirst(&self) -> Result<E> {
        let this = self;
        let _t0 = this.ns.get().pollFirst()?;
        Ok(_t0)
    }

    // java: pollLast()Ljava/lang/Object;
    pub fn pollLast(&self) -> Result<E> {
        let this = self;
        let _t0 = this.ns.get().pollLast()?;
        Ok(_t0)
    }

    // java: descendingSet()Ljava/util/NavigableSet;
    pub fn descendingSet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().descendingSet()?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.type_.get())?;
        Ok(_t1)
    }

    // java: descendingIterator()Ljava/util/Iterator;
    pub fn descendingIterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().descendingSet()?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.type_.get())?;
        let _t2 = _t1.iterator()?;
        Ok(_t2)
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableSet;
    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn subSet__obj_obj(&self, fromElement: E, toElement: E) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().subSet(fromElement, 1i32, toElement, 0i32)?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.type_.get())?;
        Ok(_t1)
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    // java: headSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn headSet__obj(&self, toElement: E) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().headSet(toElement, 0i32)?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.type_.get())?;
        Ok(_t1)
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    // java: tailSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn tailSet__obj(&self, fromElement: E) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().tailSet(fromElement, 1i32)?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.type_.get())?;
        Ok(_t1)
    }

    // java: subSet(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;
    // java: subSet(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn subSet__obj_z_obj_z(&self, fromElement: E, fromInclusive: bool, toElement: E, toInclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().subSet(fromElement, fromInclusive, toElement, toInclusive)?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.type_.get())?;
        Ok(_t1)
    }

    // java: headSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    // java: headSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn headSet__obj_z(&self, toElement: E, inclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().headSet(toElement, inclusive)?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.type_.get())?;
        Ok(_t1)
    }

    // java: tailSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    // java: tailSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn tailSet__obj_z(&self, fromElement: E, inclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().tailSet(fromElement, inclusive)?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.type_.get())?;
        Ok(_t1)
    }
}
