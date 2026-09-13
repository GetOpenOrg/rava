#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$UnmodifiableList",
    super_class = "java/util/Collections$UnmodifiableCollection",
    interfaces  = "java/util/List",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_UnmodifiableList<E> {
    #[cfg_attr(any(), java_field(name = "list", descriptor = "Ljava/util/List;", access = "final"))]
    pub list: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_UnmodifiableList<E> {
    // java: <init>(Ljava/util/List;)V
    pub fn new(&self, list: Object) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableList.<init>:(Ljava/util/List;)V")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableList.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$UnmodifiableList.hashCode:()I")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableList.get:(I)Ljava/lang/Object;")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: Object) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableList.add:(ILjava/lang/Object;)V")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableList.remove:(I)Ljava/lang/Object;")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/Collections$UnmodifiableList.indexOf:(Ljava/lang/Object;)I")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/Collections$UnmodifiableList.lastIndexOf:(Ljava/lang/Object;)I")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$UnmodifiableList.addAll:(ILjava/util/Collection;)Z")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableList.sort:(Ljava/util/Comparator;)V")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableList.listIterator:()Ljava/util/ListIterator;")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableList.listIterator:(I)Ljava/util/ListIterator;")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableList.subList:(II)Ljava/util/List;")
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableList.readResolve:()Ljava/lang/Object;")
    }
}
