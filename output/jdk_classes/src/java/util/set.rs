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
        panic!("stub: java/util/Set.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Set.isEmpty:()Z")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.contains:(Ljava/lang/Object;)Z")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Set.iterator:()Ljava/util/Iterator;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Set.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, arg0: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Set.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.add:(Ljava/lang/Object;)Z")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.remove:(Ljava/lang/Object;)Z")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.addAll:(Ljava/util/Collection;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Set.clear:()V")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Set.hashCode:()I")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Set.spliterator:()Ljava/util/Spliterator;")
    }

    // java: of()Ljava/util/Set;
    pub fn of() -> Result<Object> {
        panic!("stub: java/util/Set.of:()Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj(e1: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj(e1: Object, e2: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj(e1: Object, e2: Object, e3: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object, e9: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object, e9: Object, e10: Object) -> Result<Object> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: of([Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__arr_obj(elements: Vec<Object>) -> Result<Object> {
        panic!("stub: java/util/Set.of:([Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: copyOf(Ljava/util/Collection;)Ljava/util/Set;
    pub fn copyOf(coll: Object) -> Result<Object> {
        panic!("stub: java/util/Set.copyOf:(Ljava/util/Collection;)Ljava/util/Set;")
    }
}
