#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]
use crate::prelude::*;
use crate::java::lang::*;

// Arch-1: java_class 宏看到 is_interface = true，将此 struct 替换为 pub type Iterator = Object;
#[java_rta_macros::java_class(
    binary_name       = "java/util/Iterator",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "interface",
    generic_signature = "<E:Ljava/lang/Object;>",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Iterator.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Iterator<E: Clone + Default + 'static>(pub std::marker::PhantomData<E>);
