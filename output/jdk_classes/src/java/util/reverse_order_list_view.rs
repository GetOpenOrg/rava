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
        panic!("stub: java/util/ReverseOrderListView.of:(Ljava/util/List;Z)Ljava/util/List;")
    }

    // java: <init>(Ljava/util/List;Z)V
    pub fn new(&self, list: Object, modifiable: bool) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView.<init>:(Ljava/util/List;Z)V")
    }

    // java: checkModifiable()V
    pub fn checkModifiable(&self) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView.checkModifiable:()V")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.iterator:()Ljava/util/Iterator;")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.spliterator:()Ljava/util/Spliterator;")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.add:(Ljava/lang/Object;)Z")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.addAll:(Ljava/util/Collection;)Z")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView.clear:()V")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.contains:(Ljava/lang/Object;)Z")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.containsAll:(Ljava/util/Collection;)Z")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/ReverseOrderListView.hashCode:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.isEmpty:()Z")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.parallelStream:()Ljava/util/stream/Stream;")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove__obj(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.remove:(Ljava/lang/Object;)Z")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.removeAll:(Ljava/util/Collection;)Z")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.retainAll:(Ljava/util/Collection;)Z")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/ReverseOrderListView.size:()I")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.stream:()Ljava/util/stream/Stream;")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/ReverseOrderListView.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/ReverseOrderListView.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, generator: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/ReverseOrderListView.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/ReverseOrderListView.toString:()Ljava/lang/String;")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, index: i32, element: Object) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView.add:(ILjava/lang/Object;)V")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, index: i32, c: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.addAll:(ILjava/util/Collection;)Z")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, i: i32) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.get:(I)Ljava/lang/Object;")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ReverseOrderListView.indexOf:(Ljava/lang/Object;)I")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ReverseOrderListView.lastIndexOf:(Ljava/lang/Object;)I")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.listIterator:()Ljava/util/ListIterator;")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.listIterator:(I)Ljava/util/ListIterator;")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove__i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.remove:(I)Ljava/lang/Object;")
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/ReverseOrderListView.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView.replaceAll:(Ljava/util/function/UnaryOperator;)V")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView.sort:(Ljava/util/Comparator;)V")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/ReverseOrderListView.subList:(II)Ljava/util/List;")
    }

    // java: checkClosedRange(II)V
    pub fn checkClosedRange(index: i32, size: i32) -> Result<()> {
        panic!("stub: java/util/ReverseOrderListView.checkClosedRange:(II)V")
    }
}
