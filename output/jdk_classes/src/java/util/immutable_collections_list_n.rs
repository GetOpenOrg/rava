#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$ListN",
    super_class = "java/util/ImmutableCollections$AbstractImmutableList",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_ListN<E> {
    #[cfg_attr(any(), java_field(name = "elements", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub elements: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "allowNulls", descriptor = "Z", access = "private final"))]
    pub allowNulls: Field<bool>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_ListN<E> {
    // java: <init>([Ljava/lang/Object;Z)V
    pub fn new(&self, elements: Vec<Object>, allowNulls: bool) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$ListN.<init>:([Ljava/lang/Object;Z)V")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$ListN.isEmpty:()Z")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$ListN.size:()I")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$ListN.get:(I)Ljava/lang/Object;")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$ListN.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$ListN.writeReplace:()Ljava/lang/Object;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$ListN.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$ListN.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$ListN.indexOf:(Ljava/lang/Object;)I")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$ListN.lastIndexOf:(Ljava/lang/Object;)I")
    }
}
