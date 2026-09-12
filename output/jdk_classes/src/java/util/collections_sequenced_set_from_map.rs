#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SequencedSetFromMap",
    super_class = "java/util/Collections$SetFromMap",
    interfaces  = "java/util/SequencedSet",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SequencedSetFromMap<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_SequencedSetFromMap<E> {
    // java: nsee(Ljava/util/Map$Entry;)Ljava/lang/Object;
    pub fn nsee(&self, e: Object) -> Result<E> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = e.getKey()?;
        Ok(_t0)
    }

    // java: map()Ljava/util/SequencedMap;
    pub fn map(&self) -> Result<Object> {
        let this = self;
        Ok(this.m.get())
    }

    // java: <init>(Ljava/util/SequencedMap;)V
    pub fn new(map: Object) -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/util/Collections$SetFromMap.<init>:(Ljava/util/Map;)V */
        Ok(this)
    }

    // java: reversed()Ljava/util/SequencedSet;
    pub fn reversed(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.map()?;
        let _t1 = _t0.reversed()?;
        Ok(Collections_SequencedSetFromMap::new(_t1)?)
    }

    // java: addFirst(Ljava/lang/Object;)V
    pub fn addFirst(&self, e: E) -> Result<()> {
        let this = self;
        let _t0 = this.map()?;
        let _t1 = _t0.putFirst(e, Boolean::TRUE())?;
        Ok(())
    }

    // java: addLast(Ljava/lang/Object;)V
    pub fn addLast(&self, e: E) -> Result<()> {
        let this = self;
        let _t0 = this.map()?;
        let _t1 = _t0.putLast(e, Boolean::TRUE())?;
        Ok(())
    }

    // java: getFirst()Ljava/lang/Object;
    pub fn getFirst(&self) -> Result<E> {
        let this = self;
        let _t0 = this.map()?;
        let _t1 = _t0.firstEntry()?;
        let _t2 = this.nsee(_t1)?;
        Ok(_t2)
    }

    // java: getLast()Ljava/lang/Object;
    pub fn getLast(&self) -> Result<E> {
        let this = self;
        let _t0 = this.map()?;
        let _t1 = _t0.lastEntry()?;
        let _t2 = this.nsee(_t1)?;
        Ok(_t2)
    }

    // java: removeFirst()Ljava/lang/Object;
    pub fn removeFirst(&self) -> Result<E> {
        let this = self;
        let _t0 = this.map()?;
        let _t1 = _t0.pollFirstEntry()?;
        let _t2 = this.nsee(_t1)?;
        Ok(_t2)
    }

    // java: removeLast()Ljava/lang/Object;
    pub fn removeLast(&self) -> Result<E> {
        let this = self;
        let _t0 = this.map()?;
        let _t1 = _t0.pollLastEntry()?;
        let _t2 = this.nsee(_t1)?;
        Ok(_t2)
    }
}
