#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$SubList",
    super_class = "java/util/ImmutableCollections$AbstractImmutableList",
    interfaces  = "java/util/RandomAccess",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_SubList<E> {
    #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/ImmutableCollections$AbstractImmutableList;", access = "private final"))]
    pub root: Field<Object>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private final"))]
    pub size: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_SubList<E> {
    // java: <init>(Ljava/util/ImmutableCollections$AbstractImmutableList;II)V
    pub fn new(&self, root: Object, offset: i32, size: i32) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$SubList.<init>:(Ljava/util/ImmutableCollections$AbstractImmutableList;II)V")
    }

    // java: fromSubList(Ljava/util/ImmutableCollections$SubList;II)Ljava/util/ImmutableCollections$SubList;
    pub fn fromSubList(parent: Object, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$SubList.fromSubList:(Ljava/util/ImmutableCollections$SubList;II)Ljava/util/ImmutableCollections$SubList;")
    }

    // java: fromList(Ljava/util/ImmutableCollections$AbstractImmutableList;II)Ljava/util/ImmutableCollections$SubList;
    pub fn fromList(list: Object, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$SubList.fromList:(Ljava/util/ImmutableCollections$AbstractImmutableList;II)Ljava/util/ImmutableCollections$SubList;")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$SubList.get:(I)Ljava/lang/Object;")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$SubList.size:()I")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$SubList.iterator:()Ljava/util/Iterator;")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$SubList.listIterator:(I)Ljava/util/ListIterator;")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$SubList.subList:(II)Ljava/util/List;")
    }

    // java: rangeCheck(I)V
    pub fn rangeCheck(&self, index: i32) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$SubList.rangeCheck:(I)V")
    }

    // java: allowNulls()Z
    pub fn allowNulls(&self) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$SubList.allowNulls:()Z")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$SubList.indexOf:(Ljava/lang/Object;)I")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$SubList.lastIndexOf:(Ljava/lang/Object;)I")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$SubList.toArray:()[Ljava/lang/Object;")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        panic!("stub: java/util/ImmutableCollections$SubList.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }
}
