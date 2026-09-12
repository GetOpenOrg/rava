#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedSortedSet",
    super_class = "java/util/Collections$CheckedSet",
    interfaces  = "java/util/SortedSet,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedSortedSet<E> {
    #[cfg_attr(any(), java_field(name = "ss", descriptor = "Ljava/util/SortedSet;", access = "private final"))]
    pub ss: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CheckedSortedSet<E> {
    // java: <init>(Ljava/util/SortedSet;Ljava/lang/Class;)V
    pub fn new(s: Object, type_: Object) -> Result<Self> {
        let this = Self { ss: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$CheckedSet.<init>:(Ljava/util/Set;Ljava/lang/Class;)V */
        this.ss.set(s);
        Ok(this)
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.ss.get().comparator()?;
        Ok(_t0)
    }

    // java: first()Ljava/lang/Object;
    pub fn first(&self) -> Result<E> {
        let this = self;
        let _t0 = this.ss.get().first()?;
        Ok(_t0)
    }

    // java: last()Ljava/lang/Object;
    pub fn last(&self) -> Result<E> {
        let this = self;
        let _t0 = this.ss.get().last()?;
        Ok(_t0)
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn subSet(&self, fromElement: E, toElement: E) -> Result<Object> {
        let this = self;
        let _t0 = this.ss.get().subSet(fromElement, toElement)?;
        let _t1: Object = Collections::checkedSortedSet(_t0, this.type_.get())?;
        Ok(_t1)
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn headSet(&self, toElement: E) -> Result<Object> {
        let this = self;
        let _t0 = this.ss.get().headSet(toElement)?;
        let _t1: Object = Collections::checkedSortedSet(_t0, this.type_.get())?;
        Ok(_t1)
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn tailSet(&self, fromElement: E) -> Result<Object> {
        let this = self;
        let _t0 = this.ss.get().tailSet(fromElement)?;
        let _t1: Object = Collections::checkedSortedSet(_t0, this.type_.get())?;
        Ok(_t1)
    }
}
