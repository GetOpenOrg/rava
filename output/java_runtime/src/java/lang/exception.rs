#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Exception",
    super_class       = "java/lang/Throwable",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Exception.java",
    all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/Object;java/lang/Throwable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Exception {
    pub _super: Throwable,
}

impl Exception {
    pub fn as_throwable(&self) -> &Throwable { &self._super }
    pub fn into_throwable(self) -> Throwable { self._super }
}

impl From<Exception> for Throwable {
    fn from(v: Exception) -> Throwable { v._super }
}

impl Exception {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-3387516993124229948"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        -3387516993124229948i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = Throwable::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new_str(mut message: String) -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = Throwable::new_str(Clone::clone(&message))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str_throwa(message: String, cause: Throwable) -> Result<Self> {
        panic!("stub: java/lang/Exception.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_throwa(cause: Throwable) -> Result<Self> {
        panic!("stub: java/lang/Exception.<init>:(Ljava/lang/Throwable;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str_throwa_z_z(message: String, cause: Throwable, enableSuppression: bool, writableStackTrace: bool) -> Result<Self> {
        panic!("stub: java/lang/Exception.<init>:(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V")
    }
}
