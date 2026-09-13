#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedCollection",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedCollection<E> {
    #[cfg_attr(any(), java_field(name = "c", descriptor = "Ljava/util/Collection;", access = "final"))]
    pub c: Field<Object>,
    #[cfg_attr(any(), java_field(name = "type", descriptor = "Ljava/lang/Class;", access = "final"))]
    pub type_: Field<Object>,
    #[cfg_attr(any(), java_field(name = "zeroLengthElementArray", descriptor = "[Ljava/lang/Object;", access = "private"))]
    pub zeroLengthElementArray: Field<Vec<Object>>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CheckedCollection<E> {
    // java: typeCheck(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn typeCheck(&self, o: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedCollection.typeCheck:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: badElementMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badElementMsg(&self, o: Object) -> Result<String> {
        panic!("stub: java/util/Collections$CheckedCollection.badElementMsg:(Ljava/lang/Object;)Ljava/lang/String;")
    }

    // java: <init>(Ljava/util/Collection;Ljava/lang/Class;)V
    pub fn new(&self, c: Object, type_: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedCollection.<init>:(Ljava/util/Collection;Ljava/lang/Class;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$CheckedCollection.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.contains:(Ljava/lang/Object;)Z")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$CheckedCollection.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$CheckedCollection.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$CheckedCollection.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/Collections$CheckedCollection.toString:()Ljava/lang/String;")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.remove:(Ljava/lang/Object;)Z")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedCollection.clear:()V")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedCollection.iterator:()Ljava/util/Iterator;")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.add:(Ljava/lang/Object;)Z")
    }

    // java: zeroLengthElementArray()[Ljava/lang/Object;
    pub fn zeroLengthElementArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$CheckedCollection.zeroLengthElementArray:()[Ljava/lang/Object;")
    }

    // java: checkedCopyOf(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn checkedCopyOf(&self, coll: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedCollection.checkedCopyOf:(Ljava/util/Collection;)Ljava/util/Collection;")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, coll: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.addAll:(Ljava/util/Collection;)Z")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedCollection.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedCollection.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedCollection.spliterator:()Ljava/util/Spliterator;")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedCollection.stream:()Ljava/util/stream/Stream;")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedCollection.parallelStream:()Ljava/util/stream/Stream;")
    }
}
