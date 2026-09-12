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
    pub fn nsee(&self, e: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.nsee")
    }

    // java: map()Ljava/util/SequencedMap;
    pub fn map(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.map")
    }

    // java: <init>(Ljava/util/SequencedMap;)V
    pub fn new(&self, map: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.<init>")
    }

    // java: reversed()Ljava/util/SequencedSet;
    pub fn reversed(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.reversed")
    }

    // java: addFirst(Ljava/lang/Object;)V
    pub fn addFirst(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.addFirst")
    }

    // java: addLast(Ljava/lang/Object;)V
    pub fn addLast(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.addLast")
    }

    // java: getFirst()Ljava/lang/Object;
    pub fn getFirst(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.getFirst")
    }

    // java: getLast()Ljava/lang/Object;
    pub fn getLast(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.getLast")
    }

    // java: removeFirst()Ljava/lang/Object;
    pub fn removeFirst(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.removeFirst")
    }

    // java: removeLast()Ljava/lang/Object;
    pub fn removeLast(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SequencedSetFromMap.removeLast")
    }
}
