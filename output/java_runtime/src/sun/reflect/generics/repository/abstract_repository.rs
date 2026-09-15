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
    binary_name       = "sun/reflect/generics/repository/AbstractRepository",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<T::Lsun/reflect/generics/tree/Tree;>Ljava/lang/Object;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractRepository.java",
    all_supertypes    = "java/lang/Object;sun/reflect/generics/repository/AbstractRepository",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractRepository<T: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "factory", descriptor = "Lsun/reflect/generics/factory/GenericsFactory;", access = "private", modifiers = "final", is_static = false))]
    pub factory: JField<Object>,
    #[cfg_attr(any(), java_field(name = "tree", descriptor = "Lsun/reflect/generics/tree/Tree;", access = "private", modifiers = "final", is_static = false, generic_signature = "TT;"))]
    pub tree: JField<T>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Default + 'static> AbstractRepository<T> {
    #[cfg_attr(any(), java_method(name = "getFactory", descriptor = "()Lsun/reflect/generics/factory/GenericsFactory;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getFactory(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/repository/AbstractRepository.getFactory:()Lsun/reflect/generics/factory/GenericsFactory;")
    }

    #[cfg_attr(any(), java_method(name = "getTree", descriptor = "()Lsun/reflect/generics/tree/Tree;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;"))]
    pub fn getTree(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/repository/AbstractRepository.getTree:()Lsun/reflect/generics/tree/Tree;")
    }

    #[cfg_attr(any(), java_method(name = "getReifier", descriptor = "()Lsun/reflect/generics/visitor/Reifier;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getReifier(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/repository/AbstractRepository.getReifier:()Lsun/reflect/generics/visitor/Reifier;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(rawSig: String, f: Object) -> Result<Self> {
        panic!("stub: sun/reflect/generics/repository/AbstractRepository.<init>:(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V")
    }

    #[cfg_attr(any(), java_method(name = "parse", descriptor = "(Ljava/lang/String;)Lsun/reflect/generics/tree/Tree;", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/lang/String;)TT;"))]
    pub fn parse(&self, arg0: String) -> Result<Object> {
        panic!("stub: sun/reflect/generics/repository/AbstractRepository.parse:(Ljava/lang/String;)Lsun/reflect/generics/tree/Tree;")
    }
}
