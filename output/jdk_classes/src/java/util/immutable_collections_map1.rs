#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$Map1",
    super_class = "java/util/ImmutableCollections$AbstractImmutableMap",
    interfaces  = "",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_Map1<K, V> {
    #[cfg_attr(any(), java_field(name = "k0", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub k0: Field<Object>,
    #[cfg_attr(any(), java_field(name = "v0", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub v0: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> ImmutableCollections_Map1<K, V> {
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new(&self, k0: Object, v0: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$Map1.<init>:(Ljava/lang/Object;Ljava/lang/Object;)V")
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$Map1.entrySet:()Ljava/util/Set;")
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, o: Object) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$Map1.get:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$Map1.containsKey:(Ljava/lang/Object;)Z")
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$Map1.containsValue:(Ljava/lang/Object;)Z")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$Map1.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$Map1.isEmpty:()Z")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$Map1.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$Map1.writeReplace:()Ljava/lang/Object;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$Map1.hashCode:()I")
    }
}
