#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$UnmodifiableCollection",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_UnmodifiableCollection<E> {
    #[cfg_attr(any(), java_field(name = "c", descriptor = "Ljava/util/Collection;", access = "final"))]
    pub c: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_UnmodifiableCollection<E> {
    // java: <init>(Ljava/util/Collection;)V
    pub fn new(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.<init>:(Ljava/util/Collection;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.contains:(Ljava/lang/Object;)Z")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.toString:()Ljava/lang/String;")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.iterator:()Ljava/util/Iterator;")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.add:(Ljava/lang/Object;)Z")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.remove:(Ljava/lang/Object;)Z")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.addAll:(Ljava/util/Collection;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.clear:()V")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.spliterator:()Ljava/util/Spliterator;")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.stream:()Ljava/util/stream/Stream;")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableCollection.parallelStream:()Ljava/util/stream/Stream;")
    }
}
