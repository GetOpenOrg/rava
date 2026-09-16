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
    binary_name       = "java/util/function/Predicate",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Predicate.java",
    inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Predicate<T: Clone + Default + 'static>(std::marker::PhantomData<T>);

impl<T: Clone + Default + 'static> Predicate<T> {
    #[cfg_attr(any(), java_method(name = "test", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TT;)Z"))]
    pub fn test(&self, arg0: T) -> Result<bool> {
        panic!("stub: java/util/function/Predicate.test:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "and", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Ljava/util/function/Predicate<TT;>;"))]
    pub fn and(&self, other: Predicate<T>) -> Result<Object> {
        panic!("stub: java/util/function/Predicate.and:(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;")
    }

    #[cfg_attr(any(), java_method(name = "negate", descriptor = "()Ljava/util/function/Predicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/Predicate<TT;>;"))]
    pub fn negate(&self) -> Result<Object> {
        panic!("stub: java/util/function/Predicate.negate:()Ljava/util/function/Predicate;")
    }

    #[cfg_attr(any(), java_method(name = "or", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Ljava/util/function/Predicate<TT;>;"))]
    pub fn or(&self, other: Predicate<T>) -> Result<Object> {
        panic!("stub: java/util/function/Predicate.or:(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;")
    }

    #[cfg_attr(any(), java_method(name = "isEqual", descriptor = "(Ljava/lang/Object;)Ljava/util/function/Predicate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/lang/Object;)Ljava/util/function/Predicate<TT;>;"))]
    pub fn isEqual(targetRef: Object) -> Result<Object> {
        panic!("stub: java/util/function/Predicate.isEqual:(Ljava/lang/Object;)Ljava/util/function/Predicate;")
    }

    #[cfg_attr(any(), java_method(name = "not", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/Predicate<-TT;>;)Ljava/util/function/Predicate<TT;>;"))]
    pub fn not(target: Predicate<T>) -> Result<Object> {
        panic!("stub: java/util/function/Predicate.not:(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;")
    }
}
