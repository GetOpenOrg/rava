#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CopiesList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/RandomAccess,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CopiesList<E> {
    #[cfg_attr(any(), java_field(name = "n", descriptor = "I", access = "final"))]
    pub n: Field<i32>,
    #[cfg_attr(any(), java_field(name = "element", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub element: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CopiesList<E> {
    // java: <init>(ILjava/lang/Object;)V
    pub fn new(&self, n: i32, e: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CopiesList.<init>:(ILjava/lang/Object;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$CopiesList.size:()I")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CopiesList.contains:(Ljava/lang/Object;)Z")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/Collections$CopiesList.indexOf:(Ljava/lang/Object;)I")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/Collections$CopiesList.lastIndexOf:(Ljava/lang/Object;)I")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$CopiesList.get:(I)Ljava/lang/Object;")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CopiesList.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$CopiesList.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections$CopiesList.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$CopiesList.subList:(II)Ljava/util/List;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$CopiesList.hashCode:()I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CopiesList.equals:(Ljava/lang/Object;)Z")
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CopiesList.stream:()Ljava/util/stream/Stream;")
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CopiesList.parallelStream:()Ljava/util/stream/Stream;")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CopiesList.spliterator:()Ljava/util/Spliterator;")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, ois: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CopiesList.readObject:(Ljava/io/ObjectInputStream;)V")
    }
}
