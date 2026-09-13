#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractCollection",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection",
    access      = "public abstract",
    source      = "AbstractCollection.java",
))]
pub struct AbstractCollection<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> AbstractCollection<E> {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/AbstractCollection.<init>:()V")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractCollection.iterator:()Ljava/util/Iterator;")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractCollection.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.contains:(Ljava/lang/Object;)Z")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/AbstractCollection.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/AbstractCollection.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: finishToArray([Ljava/lang/Object;Ljava/util/Iterator;)[Ljava/lang/Object;
    pub fn finishToArray(r: Vec<Object>, it: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/AbstractCollection.finishToArray:([Ljava/lang/Object;Ljava/util/Iterator;)[Ljava/lang/Object;")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.add:(Ljava/lang/Object;)Z")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.remove:(Ljava/lang/Object;)Z")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.addAll:(Ljava/util/Collection;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/AbstractCollection.clear:()V")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/AbstractCollection.toString:()Ljava/lang/String;")
    }
}
