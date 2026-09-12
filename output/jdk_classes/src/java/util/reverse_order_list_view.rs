#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ReverseOrderListView",
    super_class = "java/lang/Object",
    interfaces  = "java/util/List",
    access      = "",
    source      = "ReverseOrderListView.java",
))]
pub struct ReverseOrderListView<E> {
    #[cfg_attr(any(), java_field(name = "base", descriptor = "Ljava/util/List;", access = "final"))]
    pub base: Field<Object>,
    #[cfg_attr(any(), java_field(name = "modifiable", descriptor = "Z", access = "final"))]
    pub modifiable: Field<bool>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ReverseOrderListView<E> {
    // java: of(Ljava/util/List;Z)Ljava/util/List;
    pub fn of(list: Object, modifiable: bool) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.of")
    }

    // java: <init>(Ljava/util/List;Z)V
    pub fn new(&self, list: Object, modifiable: bool) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView.<init>")
    }

    // java: checkModifiable()V
    pub fn checkModifiable(&self) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView.checkModifiable")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView.forEach")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.iterator")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.spliterator")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.add")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.addAll")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView.clear")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.contains")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.containsAll")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/ReverseOrderListView.hashCode")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.isEmpty")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.parallelStream")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.remove")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.removeAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.retainAll")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/ReverseOrderListView.size")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.stream")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/ReverseOrderListView.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/ReverseOrderListView.toArray")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, generator: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/ReverseOrderListView.toArray")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/ReverseOrderListView.toString")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, index: i32, element: Object) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView.add")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, index: i32, c: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.addAll")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, i: i32) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.get")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        todo!("abstract java/util/ReverseOrderListView.indexOf")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        todo!("abstract java/util/ReverseOrderListView.lastIndexOf")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.listIterator")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.listIterator")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove__i(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.remove")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        todo!("abstract java/util/ReverseOrderListView.removeIf")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView.replaceAll")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView.sort")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.set")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        todo!("abstract java/util/ReverseOrderListView.subList")
    }

    // java: checkClosedRange(II)V
    pub fn checkClosedRange(index: i32, size: i32) -> Result<()> {
        todo!("abstract java/util/ReverseOrderListView.checkClosedRange")
    }
}
