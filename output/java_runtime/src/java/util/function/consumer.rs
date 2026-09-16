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
    binary_name       = "java/util/function/Consumer",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Consumer.java",
    inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Consumer<T: Clone + Default + 'static>(std::marker::PhantomData<T>);

impl<T: Clone + Default + 'static> Consumer<T> {
    #[cfg_attr(any(), java_method(name = "accept", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TT;)V"))]
    pub fn accept(&self, arg0: T) -> Result<()> {
        panic!("stub: java/util/function/Consumer.accept:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "andThen", descriptor = "(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)Ljava/util/function/Consumer<TT;>;"))]
    pub fn andThen(&self, after: Consumer<T>) -> Result<Object> {
        panic!("stub: java/util/function/Consumer.andThen:(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;")
    }
}
