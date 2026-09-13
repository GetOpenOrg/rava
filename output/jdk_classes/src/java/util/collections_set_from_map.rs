#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SetFromMap",
    super_class = "java/util/AbstractSet",
    interfaces  = "java/util/Set,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SetFromMap<E> {
    #[cfg_attr(any(), java_field(name = "m", descriptor = "Ljava/util/Map;", access = "final"))]
    pub m: Field<Object>,
    #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/util/Set;", access = "private"))]
    pub s: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SetFromMap<E> {
    // java: <init>(Ljava/util/Map;)V
    pub fn new(&self, map: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SetFromMap.<init>:(Ljava/util/Map;)V")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collections$SetFromMap.clear:()V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SetFromMap.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.contains:(Ljava/lang/Object;)Z")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.remove:(Ljava/lang/Object;)Z")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.add:(Ljava/lang/Object;)Z")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SetFromMap.iterator:()Ljava/util/Iterator;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$SetFromMap.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$SetFromMap.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/Collections$SetFromMap.toString:()Ljava/lang/String;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SetFromMap.hashCode:()I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.equals:(Ljava/lang/Object;)Z")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SetFromMap.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SetFromMap.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SetFromMap.spliterator:()Ljava/util/Spliterator;")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SetFromMap.stream:()Ljava/util/stream/Stream;")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SetFromMap.parallelStream:()Ljava/util/stream/Stream;")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, stream: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SetFromMap.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    // java: readObjectNoData()V
    pub fn readObjectNoData(&self) -> Result<()> {
        panic!("stub: java/util/Collections$SetFromMap.readObjectNoData:()V")
    }
}
