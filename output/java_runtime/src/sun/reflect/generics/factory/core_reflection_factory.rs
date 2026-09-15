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
    binary_name       = "sun/reflect/generics/factory/CoreReflectionFactory",
    super_class       = "java/lang/Object",
    interfaces        = "sun/reflect/generics/factory/GenericsFactory",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "CoreReflectionFactory.java",
    all_supertypes    = "java/lang/Object;sun/reflect/generics/factory/CoreReflectionFactory;sun/reflect/generics/factory/GenericsFactory",
)]
#[derive(Clone, Default, PartialEq)]
pub struct CoreReflectionFactory {
    #[cfg_attr(any(), java_field(name = "decl", descriptor = "Ljava/lang/reflect/GenericDeclaration;", access = "private", modifiers = "final", is_static = false))]
    pub decl: JField<Object>,
    #[cfg_attr(any(), java_field(name = "scope", descriptor = "Lsun/reflect/generics/scope/Scope;", access = "private", modifiers = "final", is_static = false))]
    pub scope: JField<Object>,
}

impl CoreReflectionFactory {
    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(d: Object, s: Object) -> Result<Self> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.<init>:(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)V")
    }

    #[cfg_attr(any(), java_method(name = "getDecl", descriptor = "()Ljava/lang/reflect/GenericDeclaration;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getDecl(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.getDecl:()Ljava/lang/reflect/GenericDeclaration;")
    }

    #[cfg_attr(any(), java_method(name = "getScope", descriptor = "()Lsun/reflect/generics/scope/Scope;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getScope(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.getScope:()Lsun/reflect/generics/scope/Scope;")
    }

    #[cfg_attr(any(), java_method(name = "getDeclsLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getDeclsLoader(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.getDeclsLoader:()Ljava/lang/ClassLoader;")
    }

    #[cfg_attr(any(), java_method(name = "make", descriptor = "(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)Lsun/reflect/generics/factory/CoreReflectionFactory;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn make(d: Object, s: Object) -> Result<CoreReflectionFactory> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.make:(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)Lsun/reflect/generics/factory/CoreReflectionFactory;")
    }

    #[cfg_attr(any(), java_method(name = "makeTypeVariable", descriptor = "(Ljava/lang/String;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/TypeVariable<*>;"))]
    pub fn makeTypeVariable(&self, name: String, bounds: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeTypeVariable:(Ljava/lang/String;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/TypeVariable;")
    }

    #[cfg_attr(any(), java_method(name = "makeWildcard", descriptor = "([Lsun/reflect/generics/tree/FieldTypeSignature;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/WildcardType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeWildcard(&self, ubs: Rc<RefCell<Vec<Object>>>, lbs: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeWildcard:([Lsun/reflect/generics/tree/FieldTypeSignature;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/WildcardType;")
    }

    #[cfg_attr(any(), java_method(name = "makeParameterizedType", descriptor = "(Ljava/lang/reflect/Type;[Ljava/lang/reflect/Type;Ljava/lang/reflect/Type;)Ljava/lang/reflect/ParameterizedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeParameterizedType(&self, declaration: Object, typeArgs: Rc<RefCell<Vec<Object>>>, owner: Object) -> Result<ParameterizedType> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeParameterizedType:(Ljava/lang/reflect/Type;[Ljava/lang/reflect/Type;Ljava/lang/reflect/Type;)Ljava/lang/reflect/ParameterizedType;")
    }

    #[cfg_attr(any(), java_method(name = "findTypeVariable", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable<*>;"))]
    pub fn findTypeVariable(&self, name: String) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.findTypeVariable:(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable;")
    }

    #[cfg_attr(any(), java_method(name = "makeNamedType", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeNamedType(&self, name: String) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeNamedType:(Ljava/lang/String;)Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeArrayType", descriptor = "(Ljava/lang/reflect/Type;)Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeArrayType(&self, componentType: Object) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeArrayType:(Ljava/lang/reflect/Type;)Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeByte", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeByte(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeByte:()Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeBool", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeBool(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeBool:()Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeShort", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeShort(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeShort:()Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeChar", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeChar(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeChar:()Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeInt", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeInt(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeInt:()Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeLong", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeLong(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeLong:()Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeFloat", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeFloat(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeFloat:()Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeDouble", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeDouble(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeDouble:()Ljava/lang/reflect/Type;")
    }

    #[cfg_attr(any(), java_method(name = "makeVoid", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn makeVoid(&self) -> Result<Object> {
        panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeVoid:()Ljava/lang/reflect/Type;")
    }
}
