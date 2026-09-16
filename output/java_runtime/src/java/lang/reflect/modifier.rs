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
    #[binary_name       = "java/lang/reflect/Modifier"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Modifier.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/reflect/Modifier"]

    pub struct Modifier;

    impl Modifier {
        #[cfg_attr(any(), java_field(name = "PUBLIC", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: PUBLIC:I
        pub fn PUBLIC() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "PRIVATE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: PRIVATE:I
        pub fn PRIVATE() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "PROTECTED", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: PROTECTED:I
        pub fn PROTECTED() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "STATIC", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: STATIC:I
        pub fn STATIC() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "FINAL", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: FINAL:I
        pub fn FINAL() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "SYNCHRONIZED", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: SYNCHRONIZED:I
        pub fn SYNCHRONIZED() -> i32 {
            32
        }

        #[cfg_attr(any(), java_field(name = "VOLATILE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "64"))]
        // static field: VOLATILE:I
        pub fn VOLATILE() -> i32 {
            64
        }

        #[cfg_attr(any(), java_field(name = "TRANSIENT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "128"))]
        // static field: TRANSIENT:I
        pub fn TRANSIENT() -> i32 {
            128
        }

        #[cfg_attr(any(), java_field(name = "NATIVE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "256"))]
        // static field: NATIVE:I
        pub fn NATIVE() -> i32 {
            256
        }

        #[cfg_attr(any(), java_field(name = "INTERFACE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "512"))]
        // static field: INTERFACE:I
        pub fn INTERFACE() -> i32 {
            512
        }

        #[cfg_attr(any(), java_field(name = "ABSTRACT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1024"))]
        // static field: ABSTRACT:I
        pub fn ABSTRACT() -> i32 {
            1024
        }

        #[cfg_attr(any(), java_field(name = "STRICT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2048"))]
        // static field: STRICT:I
        pub fn STRICT() -> i32 {
            2048
        }

        #[cfg_attr(any(), java_field(name = "BRIDGE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "64"))]
        // static field: BRIDGE:I
        pub fn BRIDGE() -> i32 {
            64
        }

        #[cfg_attr(any(), java_field(name = "VARARGS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "128"))]
        // static field: VARARGS:I
        pub fn VARARGS() -> i32 {
            128
        }

        #[cfg_attr(any(), java_field(name = "SYNTHETIC", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4096"))]
        // static field: SYNTHETIC:I
        pub fn SYNTHETIC() -> i32 {
            4096
        }

        #[cfg_attr(any(), java_field(name = "ANNOTATION", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "8192"))]
        // static field: ANNOTATION:I
        pub fn ANNOTATION() -> i32 {
            8192
        }

        #[cfg_attr(any(), java_field(name = "ENUM", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "16384"))]
        // static field: ENUM:I
        pub fn ENUM() -> i32 {
            16384
        }

        #[cfg_attr(any(), java_field(name = "MANDATED", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "32768"))]
        // static field: MANDATED:I
        pub fn MANDATED() -> i32 {
            32768
        }

        #[cfg_attr(any(), java_field(name = "CLASS_MODIFIERS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3103"))]
        // static field: CLASS_MODIFIERS:I
        pub fn CLASS_MODIFIERS() -> i32 {
            3103
        }

        #[cfg_attr(any(), java_field(name = "INTERFACE_MODIFIERS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3087"))]
        // static field: INTERFACE_MODIFIERS:I
        pub fn INTERFACE_MODIFIERS() -> i32 {
            3087
        }

        #[cfg_attr(any(), java_field(name = "CONSTRUCTOR_MODIFIERS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: CONSTRUCTOR_MODIFIERS:I
        pub fn CONSTRUCTOR_MODIFIERS() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "METHOD_MODIFIERS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3391"))]
        // static field: METHOD_MODIFIERS:I
        pub fn METHOD_MODIFIERS() -> i32 {
            3391
        }

        #[cfg_attr(any(), java_field(name = "FIELD_MODIFIERS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "223"))]
        // static field: FIELD_MODIFIERS:I
        pub fn FIELD_MODIFIERS() -> i32 {
            223
        }

        #[cfg_attr(any(), java_field(name = "PARAMETER_MODIFIERS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: PARAMETER_MODIFIERS:I
        pub fn PARAMETER_MODIFIERS() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "ACCESS_MODIFIERS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: ACCESS_MODIFIERS:I
        pub fn ACCESS_MODIFIERS() -> i32 {
            7
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/reflect/Modifier.<init>:()V")
        }

        #[java_method(name = "isPublic", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPublic(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isPublic:(I)Z")
        }

        #[java_method(name = "isPrivate", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPrivate(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isPrivate:(I)Z")
        }

        #[java_method(name = "isProtected", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isProtected(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isProtected:(I)Z")
        }

        #[java_method(name = "isStatic", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isStatic(mut mod_: i32) -> Result<bool> {
            Ok(((mod_&8i32)!=0))
        }

        #[java_method(name = "isFinal", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFinal(mut mod_: i32) -> Result<bool> {
            Ok(((mod_&16i32)!=0))
        }

        #[java_method(name = "isSynchronized", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSynchronized(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isSynchronized:(I)Z")
        }

        #[java_method(name = "isVolatile", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isVolatile(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isVolatile:(I)Z")
        }

        #[java_method(name = "isTransient", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isTransient(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isTransient:(I)Z")
        }

        #[java_method(name = "isNative", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNative(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isNative:(I)Z")
        }

        #[java_method(name = "isInterface", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isInterface(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isInterface:(I)Z")
        }

        #[java_method(name = "isAbstract", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAbstract(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isAbstract:(I)Z")
        }

        #[java_method(name = "isStrict", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isStrict(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isStrict:(I)Z")
        }

        #[java_method(name = "toString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(mod_: i32) -> Result<String> {
            panic!("stub: java/lang/reflect/Modifier.toString:(I)Ljava/lang/String;")
        }

        #[java_method(name = "isSynthetic", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSynthetic(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isSynthetic:(I)Z")
        }

        #[java_method(name = "isMandated", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMandated(mod_: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/Modifier.isMandated:(I)Z")
        }

        #[java_method(name = "classModifiers", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn classModifiers() -> Result<i32> {
            panic!("stub: java/lang/reflect/Modifier.classModifiers:()I")
        }

        #[java_method(name = "interfaceModifiers", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn interfaceModifiers() -> Result<i32> {
            panic!("stub: java/lang/reflect/Modifier.interfaceModifiers:()I")
        }

        #[java_method(name = "constructorModifiers", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn constructorModifiers() -> Result<i32> {
            panic!("stub: java/lang/reflect/Modifier.constructorModifiers:()I")
        }

        #[java_method(name = "methodModifiers", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn methodModifiers() -> Result<i32> {
            panic!("stub: java/lang/reflect/Modifier.methodModifiers:()I")
        }

        #[java_method(name = "fieldModifiers", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fieldModifiers() -> Result<i32> {
            panic!("stub: java/lang/reflect/Modifier.fieldModifiers:()I")
        }

        #[java_method(name = "parameterModifiers", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parameterModifiers() -> Result<i32> {
            panic!("stub: java/lang/reflect/Modifier.parameterModifiers:()I")
        }
    }
}
