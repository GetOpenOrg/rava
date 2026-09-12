#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$UnmodifiableNavigableSet",
    super_class = "java/util/Collections$UnmodifiableSortedSet",
    interfaces  = "java/util/NavigableSet,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_UnmodifiableNavigableSet<E> {
    #[cfg_attr(any(), java_field(name = "ns", descriptor = "Ljava/util/NavigableSet;", access = "private final"))]
    pub ns: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_UnmodifiableNavigableSet<E> {
    // java: <init>(Ljava/util/NavigableSet;)V
    pub fn new(s: Object) -> Result<Self> {
        let this = Self { ns: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$UnmodifiableSortedSet.<init>:(Ljava/util/SortedSet;)V */
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
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: pollLast()Ljava/lang/Object;
    pub fn pollLast(&self) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: descendingSet()Ljava/util/NavigableSet;
    pub fn descendingSet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().descendingSet()?;
        Ok(Collections_UnmodifiableNavigableSet::new(_t0)?)
    }

    // java: descendingIterator()Ljava/util/Iterator;
    pub fn descendingIterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.descendingSet()?;
        let _t1 = _t0.iterator()?;
        Ok(_t1)
    }

    // java: subSet(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn subSet(&self, fromElement: E, fromInclusive: bool, toElement: E, toInclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().subSet(fromElement, fromInclusive, toElement, toInclusive)?;
        Ok(Collections_UnmodifiableNavigableSet::new(_t0)?)
    }

    // java: headSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn headSet(&self, toElement: E, inclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().headSet(toElement, inclusive)?;
        Ok(Collections_UnmodifiableNavigableSet::new(_t0)?)
    }

    // java: tailSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn tailSet(&self, fromElement: E, inclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.ns.get().tailSet(fromElement, inclusive)?;
        Ok(Collections_UnmodifiableNavigableSet::new(_t0)?)
    }
}
