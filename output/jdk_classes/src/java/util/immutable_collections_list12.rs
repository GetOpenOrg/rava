#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$List12",
    super_class = "java/util/ImmutableCollections$AbstractImmutableList",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_List12<E> {
    #[cfg_attr(any(), java_field(name = "e0", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub e0: Field<Object>,
    #[cfg_attr(any(), java_field(name = "e1", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub e1: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_List12<E> {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new__obj(&self, e0: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$List12.<init>:(Ljava/lang/Object;)V")
    }

    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new__obj_obj(&self, e0: Object, e1: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$List12.<init>:(Ljava/lang/Object;Ljava/lang/Object;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$List12.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$List12.isEmpty:()Z")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$List12.get:(I)Ljava/lang/Object;")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$List12.indexOf:(Ljava/lang/Object;)I")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$List12.lastIndexOf:(Ljava/lang/Object;)I")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$List12.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$List12.writeReplace:()Ljava/lang/Object;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$List12.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$List12.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }
}
