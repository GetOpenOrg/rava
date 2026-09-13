#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$Set12",
    super_class = "java/util/ImmutableCollections$AbstractImmutableSet",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_Set12<E> {
    #[cfg_attr(any(), java_field(name = "e0", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub e0: Field<Object>,
    #[cfg_attr(any(), java_field(name = "e1", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub e1: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_Set12<E> {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new__obj(&self, e0: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$Set12.<init>:(Ljava/lang/Object;)V")
    }

    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new__obj_obj(&self, e0: Object, e1: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$Set12.<init>:(Ljava/lang/Object;Ljava/lang/Object;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$Set12.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$Set12.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$Set12.contains:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$Set12.hashCode:()I")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$Set12.iterator:()Ljava/util/Iterator;")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$Set12.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$Set12.writeReplace:()Ljava/lang/Object;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$Set12.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$Set12.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }
}
