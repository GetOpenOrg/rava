#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedCollection",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedCollection<E> {
    #[cfg_attr(any(), java_field(name = "c", descriptor = "Ljava/util/Collection;", access = "final"))]
    pub c: Field<Object>,
    #[cfg_attr(any(), java_field(name = "mutex", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub mutex: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SynchronizedCollection<E> {
    // java: <init>(Ljava/util/Collection;)V
    pub fn new__coll(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedCollection.<init>:(Ljava/util/Collection;)V")
    }

    // java: <init>(Ljava/util/Collection;Ljava/lang/Object;)V
    pub fn new__coll_obj(&self, c: Object, mutex: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedCollection.<init>:(Ljava/util/Collection;Ljava/lang/Object;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SynchronizedCollection.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.contains:(Ljava/lang/Object;)Z")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$SynchronizedCollection.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$SynchronizedCollection.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$SynchronizedCollection.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedCollection.iterator:()Ljava/util/Iterator;")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.add:(Ljava/lang/Object;)Z")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.remove:(Ljava/lang/Object;)Z")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.addAll:(Ljava/util/Collection;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedCollection.clear:()V")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/Collections$SynchronizedCollection.toString:()Ljava/lang/String;")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, consumer: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedCollection.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedCollection.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedCollection.spliterator:()Ljava/util/Spliterator;")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedCollection.stream:()Ljava/util/stream/Stream;")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedCollection.parallelStream:()Ljava/util/stream/Stream;")
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedCollection.writeObject:(Ljava/io/ObjectOutputStream;)V")
    }
}
