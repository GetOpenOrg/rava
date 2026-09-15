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
    binary_name       = "sun/reflect/generics/scope/AbstractScope",
    super_class       = "java/lang/Object",
    interfaces        = "sun/reflect/generics/scope/Scope",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<D::Ljava/lang/reflect/GenericDeclaration;>Ljava/lang/Object;Lsun/reflect/generics/scope/Scope;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractScope.java",
    all_supertypes    = "java/lang/Object;sun/reflect/generics/scope/AbstractScope;sun/reflect/generics/scope/Scope",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractScope<D: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "recvr", descriptor = "Ljava/lang/reflect/GenericDeclaration;", access = "private", modifiers = "final", is_static = false, generic_signature = "TD;"))]
    pub recvr: JField<D>,
    #[cfg_attr(any(), java_field(name = "enclosingScope", descriptor = "Lsun/reflect/generics/scope/Scope;", access = "private", modifiers = "volatile", is_static = false))]
    pub enclosingScope: JField<Object>,
    pub _phantom: std::marker::PhantomData<D>,
}

impl<D: Clone + Default + 'static> AbstractScope<D> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/reflect/GenericDeclaration;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TD;)V"))]
    pub fn new(decl: Object) -> Result<Self> {
        panic!("stub: sun/reflect/generics/scope/AbstractScope.<init>:(Ljava/lang/reflect/GenericDeclaration;)V")
    }

    #[cfg_attr(any(), java_method(name = "getRecvr", descriptor = "()Ljava/lang/reflect/GenericDeclaration;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TD;"))]
    pub fn getRecvr(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/scope/AbstractScope.getRecvr:()Ljava/lang/reflect/GenericDeclaration;")
    }

    #[cfg_attr(any(), java_method(name = "computeEnclosingScope", descriptor = "()Lsun/reflect/generics/scope/Scope;", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn computeEnclosingScope(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/scope/AbstractScope.computeEnclosingScope:()Lsun/reflect/generics/scope/Scope;")
    }

    #[cfg_attr(any(), java_method(name = "getEnclosingScope", descriptor = "()Lsun/reflect/generics/scope/Scope;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getEnclosingScope(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/scope/AbstractScope.getEnclosingScope:()Lsun/reflect/generics/scope/Scope;")
    }

    #[cfg_attr(any(), java_method(name = "lookup", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable<*>;"))]
    pub fn lookup(&self, name: String) -> Result<Object> {
        panic!("stub: sun/reflect/generics/scope/AbstractScope.lookup:(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable;")
    }
}
