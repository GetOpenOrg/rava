#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/ListIterator",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/Iterator",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Iterator<TE;>;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ListIterator.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct ListIterator<E: Clone + Default + 'static>(std::marker::PhantomData<E>);

impl<E: Clone + Default + 'static> ListIterator<E> {
    #[cfg_attr(any(), java_method(name = "hasNext", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn hasNext(&self) -> Result<bool> {
        panic!("stub: java/util/ListIterator.hasNext:()Z")
    }

    #[cfg_attr(any(), java_method(name = "next", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn next(&self) -> Result<Object> {
        panic!("stub: java/util/ListIterator.next:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "hasPrevious", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn hasPrevious(&self) -> Result<bool> {
        panic!("stub: java/util/ListIterator.hasPrevious:()Z")
    }

    #[cfg_attr(any(), java_method(name = "previous", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn previous(&self) -> Result<Object> {
        panic!("stub: java/util/ListIterator.previous:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "nextIndex", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn nextIndex(&self) -> Result<i32> {
        panic!("stub: java/util/ListIterator.nextIndex:()I")
    }

    #[cfg_attr(any(), java_method(name = "previousIndex", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn previousIndex(&self) -> Result<i32> {
        panic!("stub: java/util/ListIterator.previousIndex:()I")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn remove(&self) -> Result<()> {
        panic!("stub: java/util/ListIterator.remove:()V")
    }

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TE;)V"))]
    pub fn set(&self, arg0: E) -> Result<()> {
        panic!("stub: java/util/ListIterator.set:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TE;)V"))]
    pub fn add(&self, arg0: E) -> Result<()> {
        panic!("stub: java/util/ListIterator.add:(Ljava/lang/Object;)V")
    }
}
