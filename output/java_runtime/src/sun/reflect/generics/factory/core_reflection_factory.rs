#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/reflect/generics/factory/CoreReflectionFactory"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "sun/reflect/generics/factory/GenericsFactory"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CoreReflectionFactory.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/reflect/generics/factory/CoreReflectionFactory;sun/reflect/generics/factory/GenericsFactory"]

    pub struct CoreReflectionFactory {
        #[cfg_attr(any(), java_field(name = "decl", descriptor = "Ljava/lang/reflect/GenericDeclaration;", access = "private", modifiers = "final", is_static = false))]
        pub decl: Object,
        #[cfg_attr(any(), java_field(name = "scope", descriptor = "Lsun/reflect/generics/scope/Scope;", access = "private", modifiers = "final", is_static = false))]
        pub scope: Object,
    }

    impl CoreReflectionFactory {
        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(d: Object, s: Object) -> Result<Self> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.<init>:(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)V")
        }

        #[java_method(name = "getDecl", descriptor = "()Ljava/lang/reflect/GenericDeclaration;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDecl(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.getDecl:()Ljava/lang/reflect/GenericDeclaration;")
        }

        #[java_method(name = "getScope", descriptor = "()Lsun/reflect/generics/scope/Scope;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getScope(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.getScope:()Lsun/reflect/generics/scope/Scope;")
        }

        #[java_method(name = "getDeclsLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDeclsLoader(&self) -> Result<ClassLoader> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.getDeclsLoader:()Ljava/lang/ClassLoader;")
        }

        #[java_method(name = "make", descriptor = "(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)Lsun/reflect/generics/factory/CoreReflectionFactory;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn make(d: Object, s: Object) -> Result<CoreReflectionFactory> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.make:(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)Lsun/reflect/generics/factory/CoreReflectionFactory;")
        }

        #[java_method(name = "makeTypeVariable", descriptor = "(Ljava/lang/String;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/TypeVariable<*>;")]
        pub fn makeTypeVariable(&self, name: String, bounds: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeTypeVariable:(Ljava/lang/String;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/TypeVariable;")
        }

        #[java_method(name = "makeWildcard", descriptor = "([Lsun/reflect/generics/tree/FieldTypeSignature;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/WildcardType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeWildcard(&self, ubs: Rc<RefCell<Vec<Object>>>, lbs: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeWildcard:([Lsun/reflect/generics/tree/FieldTypeSignature;[Lsun/reflect/generics/tree/FieldTypeSignature;)Ljava/lang/reflect/WildcardType;")
        }

        #[java_method(name = "makeParameterizedType", descriptor = "(Ljava/lang/reflect/Type;[Ljava/lang/reflect/Type;Ljava/lang/reflect/Type;)Ljava/lang/reflect/ParameterizedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeParameterizedType(&self, declaration: Object, typeArgs: Rc<RefCell<Vec<Object>>>, owner: Object) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeParameterizedType:(Ljava/lang/reflect/Type;[Ljava/lang/reflect/Type;Ljava/lang/reflect/Type;)Ljava/lang/reflect/ParameterizedType;")
        }

        #[java_method(name = "findTypeVariable", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable<*>;")]
        pub fn findTypeVariable(&self, name: String) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.findTypeVariable:(Ljava/lang/String;)Ljava/lang/reflect/TypeVariable;")
        }

        #[java_method(name = "makeNamedType", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeNamedType(&self, name: String) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeNamedType:(Ljava/lang/String;)Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeArrayType", descriptor = "(Ljava/lang/reflect/Type;)Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeArrayType(&self, componentType: Object) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeArrayType:(Ljava/lang/reflect/Type;)Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeByte", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeByte(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeByte:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeBool", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeBool(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeBool:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeShort", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeShort(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeShort:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeChar", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeChar(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeChar:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeInt", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeInt(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeInt:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeLong", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeLong(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeLong:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeFloat", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeFloat(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeFloat:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeDouble", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeDouble(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeDouble:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "makeVoid", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeVoid(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/factory/CoreReflectionFactory.makeVoid:()Ljava/lang/reflect/Type;")
        }
    }
}
