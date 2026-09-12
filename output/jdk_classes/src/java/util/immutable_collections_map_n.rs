#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$MapN",
    super_class = "java/util/ImmutableCollections$AbstractImmutableMap",
    interfaces  = "",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_MapN<K, V> {
    #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub table: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "final"))]
    pub size: Field<i32>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> ImmutableCollections_MapN<K, V> {
    // java: <init>([Ljava/lang/Object;)V
    pub fn new(&self, input: Vec<Object>) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$MapN.<init>")
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$MapN.containsKey")
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$MapN.containsValue")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/ImmutableCollections$MapN.hashCode")
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, o: Object) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$MapN.get")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/ImmutableCollections$MapN.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$MapN.isEmpty")
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$MapN.entrySet")
    }

    // java: probe(Ljava/lang/Object;)I
    pub fn probe(&self, pk: Object) -> Result<i32> {
        todo!("abstract java/util/ImmutableCollections$MapN.probe")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$MapN.readObject")
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$MapN.writeReplace")
    }
}
