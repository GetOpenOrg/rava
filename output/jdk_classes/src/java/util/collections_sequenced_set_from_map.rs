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
        panic!("stub: java/util/Collections$SequencedSetFromMap.nsee:(Ljava/util/Map$Entry;)Ljava/lang/Object;")
    }

    // java: map()Ljava/util/SequencedMap;
    pub fn map(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.map:()Ljava/util/SequencedMap;")
    }

    // java: <init>(Ljava/util/SequencedMap;)V
    pub fn new(&self, map: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.<init>:(Ljava/util/SequencedMap;)V")
    }

    // java: reversed()Ljava/util/SequencedSet;
    pub fn reversed(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.reversed:()Ljava/util/SequencedSet;")
    }

    // java: addFirst(Ljava/lang/Object;)V
    pub fn addFirst(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.addFirst:(Ljava/lang/Object;)V")
    }

    // java: addLast(Ljava/lang/Object;)V
    pub fn addLast(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.addLast:(Ljava/lang/Object;)V")
    }

    // java: getFirst()Ljava/lang/Object;
    pub fn getFirst(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.getFirst:()Ljava/lang/Object;")
    }

    // java: getLast()Ljava/lang/Object;
    pub fn getLast(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.getLast:()Ljava/lang/Object;")
    }

    // java: removeFirst()Ljava/lang/Object;
    pub fn removeFirst(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.removeFirst:()Ljava/lang/Object;")
    }

    // java: removeLast()Ljava/lang/Object;
    pub fn removeLast(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SequencedSetFromMap.removeLast:()Ljava/lang/Object;")
    }
}
