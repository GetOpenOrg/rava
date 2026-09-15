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
    binary_name       = "java/io/IOException",
    super_class       = "java/lang/Exception",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "IOException.java",
    all_supertypes    = "java/io/IOException;java/io/Serializable;java/lang/Exception;java/lang/Object;java/lang/Throwable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct IOException {
    pub _super: Exception,
}

impl IOException {
    pub fn as_exception(&self) -> &Exception { &self._super }
    pub fn into_exception(self) -> Exception { self._super }
    pub fn as_throwable(&self) -> &Throwable { &self._super._super }
    pub fn into_throwable(self) -> Throwable { self._super._super }
}

impl From<IOException> for Exception {
    fn from(v: IOException) -> Exception { v._super }
}

impl From<IOException> for Throwable {
    fn from(v: IOException) -> Throwable { v._super._super }
}

impl IOException {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "7818375828146090155"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        7818375828146090155i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = Exception::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new_str(mut message: String) -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = Exception::new_str(Clone::clone(&message))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str_throwa(message: String, cause: Throwable) -> Result<Self> {
        panic!("stub: java/io/IOException.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_throwa(cause: Throwable) -> Result<Self> {
        panic!("stub: java/io/IOException.<init>:(Ljava/lang/Throwable;)V")
    }
}
