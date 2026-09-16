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
use crate::jdk::internal::misc::Unsafe;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/ByteOrder"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ByteOrder.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/nio/ByteOrder"]
    #[has_to_string_method = true]

    pub struct ByteOrder {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
    }

    impl ByteOrder {
        #[cfg_attr(any(), java_field(name = "BIG_ENDIAN", descriptor = "Ljava/nio/ByteOrder;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BIG_ENDIAN:Ljava/nio/ByteOrder;
        pub fn BIG_ENDIAN() -> ByteOrder {
            panic!("stub: java/nio/ByteOrder.BIG_ENDIAN:Ljava/nio/ByteOrder;")
        }

        #[cfg_attr(any(), java_field(name = "LITTLE_ENDIAN", descriptor = "Ljava/nio/ByteOrder;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LITTLE_ENDIAN:Ljava/nio/ByteOrder;
        pub fn LITTLE_ENDIAN() -> ByteOrder {
            panic!("stub: java/nio/ByteOrder.LITTLE_ENDIAN:Ljava/nio/ByteOrder;")
        }

        #[cfg_attr(any(), java_field(name = "NATIVE_ORDER", descriptor = "Ljava/nio/ByteOrder;", access = "private", modifiers = "static final", is_static = true))]
        // static field: NATIVE_ORDER:Ljava/nio/ByteOrder;
        pub fn NATIVE_ORDER() -> ByteOrder {
            panic!("stub: java/nio/ByteOrder.NATIVE_ORDER:Ljava/nio/ByteOrder;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(name: String) -> Result<Self> {
            panic!("stub: java/nio/ByteOrder.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "nativeOrder", descriptor = "()Ljava/nio/ByteOrder;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nativeOrder() -> Result<ByteOrder> {
            Ok(ByteOrder::NATIVE_ORDER())
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }
    }
}
