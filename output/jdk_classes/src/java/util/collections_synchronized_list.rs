#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedList",
    super_class = "java/util/Collections$SynchronizedCollection",
    interfaces  = "java/util/List",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedList<E> {
    #[cfg_attr(any(), java_field(name = "list", descriptor = "Ljava/util/List;", access = "final"))]
    pub list: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SynchronizedList<E> {
    // java: <init>(Ljava/util/List;)V
    pub fn new__list(&self, list: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedList.<init>:(Ljava/util/List;)V")
    }

    // java: <init>(Ljava/util/List;Ljava/lang/Object;)V
    pub fn new__list_obj(&self, list: Object, mutex: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedList.<init>:(Ljava/util/List;Ljava/lang/Object;)V")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedList.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SynchronizedList.hashCode:()I")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedList.get:(I)Ljava/lang/Object;")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedList.add:(ILjava/lang/Object;)V")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedList.remove:(I)Ljava/lang/Object;")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/Collections$SynchronizedList.indexOf:(Ljava/lang/Object;)I")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/Collections$SynchronizedList.lastIndexOf:(Ljava/lang/Object;)I")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedList.addAll:(ILjava/util/Collection;)Z")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedList.listIterator:()Ljava/util/ListIterator;")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedList.listIterator:(I)Ljava/util/ListIterator;")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedList.subList:(II)Ljava/util/List;")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedList.sort:(Ljava/util/Comparator;)V")
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedList.readResolve:()Ljava/lang/Object;")
    }
}
