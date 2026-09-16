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
    binary_name       = "java/util/ConcurrentModificationException",
    super_class       = "java/lang/RuntimeException",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ConcurrentModificationException.java",
    all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable;java/util/ConcurrentModificationException",
)]
#[derive(Clone, Default, PartialEq)]
pub struct ConcurrentModificationException {
    pub _super: RuntimeException,
}

impl ConcurrentModificationException {
    pub fn as_runtime_exception(&self) -> &RuntimeException { &self._super }
    pub fn into_runtime_exception(self) -> RuntimeException { self._super }
    pub fn as_exception(&self) -> &Exception { &self._super._super }
    pub fn into_exception(self) -> Exception { self._super._super }
    pub fn as_throwable(&self) -> &Throwable { &self._super._super._super }
    pub fn into_throwable(self) -> Throwable { self._super._super._super }
}

impl From<ConcurrentModificationException> for RuntimeException {
    fn from(v: ConcurrentModificationException) -> RuntimeException { v._super }
}

impl From<ConcurrentModificationException> for Exception {
    fn from(v: ConcurrentModificationException) -> Exception { v._super._super }
}

impl From<ConcurrentModificationException> for Throwable {
    fn from(v: ConcurrentModificationException) -> Throwable { v._super._super._super }
}

impl ConcurrentModificationException {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-3666751008965953603"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        -3666751008965953603i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = RuntimeException::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str(message: String) -> Result<Self> {
        panic!("stub: java/util/ConcurrentModificationException.<init>:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_throwa(cause: Throwable) -> Result<Self> {
        panic!("stub: java/util/ConcurrentModificationException.<init>:(Ljava/lang/Throwable;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str_throwa(message: String, cause: Throwable) -> Result<Self> {
        panic!("stub: java/util/ConcurrentModificationException.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }
}
