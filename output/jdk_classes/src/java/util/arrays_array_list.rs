#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays$ArrayList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/RandomAccess,java/io/Serializable",
    access      = "",
    source      = "Arrays.java",
))]
pub struct Arrays_ArrayList<E> {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Vec<Object>>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Arrays_ArrayList<E> {
    // java: <init>([Ljava/lang/Object;)V
    pub fn new(&self, array: Vec<Object>) -> Result<()> {
        panic!("stub: java/util/Arrays$ArrayList.<init>:([Ljava/lang/Object;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Arrays$ArrayList.size:()I")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/Arrays$ArrayList.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/Arrays$ArrayList.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays$ArrayList.get:(I)Ljava/lang/Object;")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        panic!("stub: java/util/Arrays$ArrayList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays$ArrayList.indexOf:(Ljava/lang/Object;)I")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Arrays$ArrayList.contains:(Ljava/lang/Object;)Z")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Arrays$ArrayList.spliterator:()Ljava/util/Spliterator;")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Arrays$ArrayList.forEach:(Ljava/util/function/Consumer;)V")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays$ArrayList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/Arrays$ArrayList.sort:(Ljava/util/Comparator;)V")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Arrays$ArrayList.iterator:()Ljava/util/Iterator;")
    }
}
