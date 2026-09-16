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

impl From<BasicPermission> for Permission {
    fn from(v: BasicPermission) -> Permission { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/security/BasicPermission"]
    #[super_class       = "java/security/Permission"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BasicPermission.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Permission"]
    #[superclass_fields(name: String)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/security/BasicPermission;java/security/Guard;java/security/Permission"]
    #[has_hash_code_method = true]

    pub struct BasicPermission {
        #[cfg_attr(any(), java_field(name = "wildcard", descriptor = "Z", access = "private", modifiers = "transient", is_static = false))]
        pub wildcard: bool,
        #[cfg_attr(any(), java_field(name = "path", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
        pub path: String,
        #[cfg_attr(any(), java_field(name = "exitVM", descriptor = "Z", access = "private", modifiers = "transient", is_static = false))]
        pub exitVM: bool,
    }

    impl BasicPermission {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "6279438298436773498"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            6279438298436773498i64
        }

        #[java_method(name = "init", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn init(&self, name: String) -> Result<()> {
            panic!("stub: java/security/BasicPermission.init:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(name: String) -> Result<Self> {
            panic!("stub: java/security/BasicPermission.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_str(name: String, actions: String) -> Result<Self> {
            panic!("stub: java/security/BasicPermission.<init>:(Ljava/lang/String;Ljava/lang/String;)V")
        }

        #[java_method(name = "implies", descriptor = "(Ljava/security/Permission;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implies(&self, p: Permission) -> Result<bool> {
            panic!("stub: java/security/BasicPermission.implies:(Ljava/security/Permission;)Z")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/security/BasicPermission.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "getActions", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getActions(&self) -> Result<String> {
            panic!("stub: java/security/BasicPermission.getActions:()Ljava/lang/String;")
        }

        #[java_method(name = "newPermissionCollection", descriptor = "()Ljava/security/PermissionCollection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newPermissionCollection(&self) -> Result<Object> {
            panic!("stub: java/security/BasicPermission.newPermissionCollection:()Ljava/security/PermissionCollection;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/security/BasicPermission.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "getCanonicalName", descriptor = "()Ljava/lang/String;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCanonicalName(&self) -> Result<String> {
            panic!("stub: java/security/BasicPermission.getCanonicalName:()Ljava/lang/String;")
        }
    }
}
