#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Comparable",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Comparable.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Comparable<T: Clone + Default + 'static>(std::marker::PhantomData<T>);

impl<T: Clone + Default + 'static> Comparable<T> {
    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TT;)I"))]
    pub fn compareTo(&self, arg0: Object) -> Result<i32> {
        panic!("stub: java/lang/Comparable.compareTo:(Ljava/lang/Object;)I")
    }
}
