#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/List",
    super_class = "java/lang/Object",
    interfaces  = "java/util/SequencedCollection",
    access      = "public abstract",
    source      = "List.java",
))]
pub struct List<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> List<E> {
    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/List.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/List.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.contains")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/List.iterator")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/List.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, arg0: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/List.toArray")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.add")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.remove")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.containsAll")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.addAll")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, arg0: i32, arg1: Object) -> Result<bool> {
        todo!("abstract java/util/List.addAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.removeAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.retainAll")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        todo!("abstract java/util/List.replaceAll")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        todo!("abstract java/util/List.sort")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/List.clear")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/List.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/List.hashCode")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/util/List.get")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, arg0: i32, arg1: Object) -> Result<Object> {
        todo!("abstract java/util/List.set")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, arg0: i32, arg1: Object) -> Result<()> {
        todo!("abstract java/util/List.add")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove__i(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/util/List.remove")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, arg0: Object) -> Result<i32> {
        todo!("abstract java/util/List.indexOf")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, arg0: Object) -> Result<i32> {
        todo!("abstract java/util/List.lastIndexOf")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        todo!("abstract java/util/List.listIterator")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/util/List.listIterator")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, arg0: i32, arg1: i32) -> Result<Object> {
        todo!("abstract java/util/List.subList")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        todo!("abstract java/util/List.spliterator")
    }

    // java: addFirst(Ljava/lang/Object;)V
    pub fn addFirst(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/List.addFirst")
    }

    // java: addLast(Ljava/lang/Object;)V
    pub fn addLast(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/List.addLast")
    }

    // java: getFirst()Ljava/lang/Object;
    pub fn getFirst(&self) -> Result<Object> {
        todo!("abstract java/util/List.getFirst")
    }

    // java: getLast()Ljava/lang/Object;
    pub fn getLast(&self) -> Result<Object> {
        todo!("abstract java/util/List.getLast")
    }

    // java: removeFirst()Ljava/lang/Object;
    pub fn removeFirst(&self) -> Result<Object> {
        todo!("abstract java/util/List.removeFirst")
    }

    // java: removeLast()Ljava/lang/Object;
    pub fn removeLast(&self) -> Result<Object> {
        todo!("abstract java/util/List.removeLast")
    }

    // java: reversed()Ljava/util/List;
    pub fn reversed(&self) -> Result<Object> {
        todo!("abstract java/util/List.reversed")
    }

    // java: of()Ljava/util/List;
    pub fn of() -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj(e1: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj(e1: Object, e2: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj(e1: Object, e2: Object, e3: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object, e9: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: Object, e2: Object, e3: Object, e4: Object, e5: Object, e6: Object, e7: Object, e8: Object, e9: Object, e10: Object) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: of([Ljava/lang/Object;)Ljava/util/List;
    pub fn of__arr_obj(elements: Vec<Object>) -> Result<Object> {
        todo!("abstract java/util/List.of")
    }

    // java: copyOf(Ljava/util/Collection;)Ljava/util/List;
    pub fn copyOf(coll: Object) -> Result<Object> {
        todo!("abstract java/util/List.copyOf")
    }
}
