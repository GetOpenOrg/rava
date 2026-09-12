#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Set",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection",
    access      = "public abstract",
    source      = "Set.java",
))]
pub struct Set<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Set<E> {
    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Set.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Set.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.contains")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/Set.iterator")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/Set.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, arg0: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/Set.toArray")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.add")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.remove")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.containsAll")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.addAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.retainAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.removeAll")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Set.clear")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Set.hashCode")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        todo!("abstract java/util/Set.spliterator")
    }

    // java: of()Ljava/util/Set;
    pub fn of() -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj(e1: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj(e1: Object, e2: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj(e1: Object, e2: Object, e3: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object, e9: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object, e9: Object, e10: Object) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: of([Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__arr_obj(elements: Vec<Object>) -> Result<Object> {
        todo!("abstract java/util/Set.of")
    }

    // java: copyOf(Ljava/util/Collection;)Ljava/util/Set;
    pub fn copyOf(coll: Object) -> Result<Object> {
        todo!("abstract java/util/Set.copyOf")
    }
}
