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
    #[binary_name       = "java/lang/Boolean"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/Comparable,java/lang/constant/Constable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/Comparable<Ljava/lang/Boolean;>;Ljava/lang/constant/Constable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Boolean.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Boolean;java/lang/Comparable;java/lang/Object;java/lang/constant/Constable"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Boolean {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "Z", access = "private", modifiers = "final", is_static = false))]
        pub value: bool,
    }

    impl Boolean {
        #[cfg_attr(any(), java_field(name = "TRUE", descriptor = "Ljava/lang/Boolean;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TRUE:Ljava/lang/Boolean;
        pub fn TRUE() -> bool {
            panic!("stub: java/lang/Boolean.TRUE:Ljava/lang/Boolean;")
        }

        #[cfg_attr(any(), java_field(name = "FALSE", descriptor = "Ljava/lang/Boolean;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FALSE:Ljava/lang/Boolean;
        pub fn FALSE() -> bool {
            panic!("stub: java/lang/Boolean.FALSE:Ljava/lang/Boolean;")
        }

        #[cfg_attr(any(), java_field(name = "TYPE", descriptor = "Ljava/lang/Class;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/Class<Ljava/lang/Boolean;>;"))]
        // static field: TYPE:Ljava/lang/Class;
        pub fn TYPE() -> Class<bool> {
            panic!("stub: java/lang/Boolean.TYPE:Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-3665804199014368530"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -3665804199014368530i64
        }

        #[java_method(name = "<init>", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_z(value: bool) -> Result<Self> {
            panic!("stub: java/lang/Boolean.<init>:(Z)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_str(s: String) -> Result<Self> {
            panic!("stub: java/lang/Boolean.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "parseBoolean", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseBoolean(s: String) -> Result<bool> {
            panic!("stub: java/lang/Boolean.parseBoolean:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "booleanValue", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn booleanValue(&self) -> Result<bool> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "valueOf", descriptor = "(Z)Ljava/lang/Boolean;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(Z)Ljava/lang/Boolean;
        pub fn valueOf_z(mut b: bool) -> Result<bool> {
            Ok((if b { Boolean::TRUE() } else { Boolean::FALSE() }))
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Boolean;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_str(s: String) -> Result<bool> {
            panic!("stub: java/lang/Boolean.valueOf:(Ljava/lang/String;)Ljava/lang/Boolean;")
        }

        #[java_method(name = "toString", descriptor = "(Z)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString(Z)Ljava/lang/String;
        pub fn toString_z(mut b: bool) -> Result<String> {
            Ok(String::from_owned(format!("{}", b)))
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString()Ljava/lang/String;
        pub fn toString(&self) -> Result<String> {
            let this = self;
            Ok(String::from_owned(format!("{}", this.__get_value())))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "hashCode", descriptor = "(Z)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_z(value: bool) -> Result<i32> {
            panic!("stub: java/lang/Boolean.hashCode:(Z)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/Boolean.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "getBoolean", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBoolean(name: String) -> Result<bool> {
            panic!("stub: java/lang/Boolean.getBoolean:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/Boolean;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut b: bool) -> Result<i32> {
            let this = self;
            let _t0: i32 = Boolean::compare(this.__get_value(), b.__get_value())?;
            Ok(_t0)
        }

        #[java_method(name = "compare", descriptor = "(ZZ)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(mut x: bool, mut y: bool) -> Result<i32> {
            Ok(((if x == y { (0i32 != 0) } else { !(x) })) as i32)
        }

        #[java_method(name = "logicalAnd", descriptor = "(ZZ)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn logicalAnd(a: bool, b: bool) -> Result<bool> {
            panic!("stub: java/lang/Boolean.logicalAnd:(ZZ)Z")
        }

        #[java_method(name = "logicalOr", descriptor = "(ZZ)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn logicalOr(a: bool, b: bool) -> Result<bool> {
            panic!("stub: java/lang/Boolean.logicalOr:(ZZ)Z")
        }

        #[java_method(name = "logicalXor", descriptor = "(ZZ)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn logicalXor(a: bool, b: bool) -> Result<bool> {
            panic!("stub: java/lang/Boolean.logicalXor:(ZZ)Z")
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/constant/DynamicConstantDesc<Ljava/lang/Boolean;>;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Boolean.describeConstable:()Ljava/util/Optional;")
        }
    }
}
