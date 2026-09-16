#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

// Arch-1: java_class 宏看到 is_interface = true，将此 struct 替换为 pub type List = Object;
// 接口方法通过 invokeinterface downcast 到具体类型（如 ArrayList）调用，无需存根。
#[java_rta_macros::java_class(
    binary_name       = "java/util/List",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/SequencedCollection",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/SequencedCollection<TE;>;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "List.java",
    inner_classes     = "java/util/AbstractList$RandomAccessSpliterator:java/util/AbstractList:RandomAccessSpliterator:24;java/util/ImmutableCollections$ListN:java/util/ImmutableCollections:ListN:24;java/util/ImmutableCollections$List12:java/util/ImmutableCollections:List12:24",
)]
#[derive(Clone, Default, PartialEq)]
pub struct List<E: Clone + Default + 'static>(std::marker::PhantomData<E>);
