#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$SetN",
    super_class = "java/util/ImmutableCollections$AbstractImmutableSet",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_SetN<E> {
    #[cfg_attr(any(), java_field(name = "elements", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub elements: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "final"))]
    pub size: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_SetN<E> {
    // java: <init>([Ljava/lang/Object;)V
    pub fn new(&self, input: Vec<Object>) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$SetN.<init>:([Ljava/lang/Object;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$SetN.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$SetN.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$SetN.contains:(Ljava/lang/Object;)Z")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$SetN.iterator:()Ljava/util/Iterator;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$SetN.hashCode:()I")
    }

    // java: probe(Ljava/lang/Object;)I
    pub fn probe(&self, pe: Object) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$SetN.probe:(Ljava/lang/Object;)I")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$SetN.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$SetN.writeReplace:()Ljava/lang/Object;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$SetN.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$SetN.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }
}
