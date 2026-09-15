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
    binary_name       = "sun/reflect/generics/scope/ClassScope",
    super_class       = "sun/reflect/generics/scope/AbstractScope",
    interfaces        = "sun/reflect/generics/scope/Scope",
    access            = "public",
    modifiers         = "",
    generic_signature = "Lsun/reflect/generics/scope/AbstractScope<Ljava/lang/Class<*>;>;Lsun/reflect/generics/scope/Scope;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ClassScope.java",
    all_supertypes    = "java/lang/Object;sun/reflect/generics/scope/AbstractScope;sun/reflect/generics/scope/ClassScope;sun/reflect/generics/scope/Scope",
)]
#[derive(Clone, Default, PartialEq)]
pub struct ClassScope {
    pub _super: AbstractScope<Object>,
}

impl ClassScope {
    pub fn as_abstract_scope(&self) -> &AbstractScope<Object> { &self._super }
    pub fn into_abstract_scope(self) -> AbstractScope<Object> { self._super }
}

impl From<ClassScope> for AbstractScope<Object> {
    fn from(v: ClassScope) -> AbstractScope<Object> { v._super }
}

impl ClassScope {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Class;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V"))]
    pub fn new(c: Object) -> Result<Self> {
        panic!("stub: sun/reflect/generics/scope/ClassScope.<init>:(Ljava/lang/Class;)V")
    }

    #[cfg_attr(any(), java_method(name = "computeEnclosingScope", descriptor = "()Lsun/reflect/generics/scope/Scope;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn computeEnclosingScope(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/scope/ClassScope.computeEnclosingScope:()Lsun/reflect/generics/scope/Scope;")
    }

    #[cfg_attr(any(), java_method(name = "make", descriptor = "(Ljava/lang/Class;)Lsun/reflect/generics/scope/ClassScope;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Lsun/reflect/generics/scope/ClassScope;"))]
    pub fn make(c: Object) -> Result<ClassScope> {
        panic!("stub: sun/reflect/generics/scope/ClassScope.make:(Ljava/lang/Class;)Lsun/reflect/generics/scope/ClassScope;")
    }
}
