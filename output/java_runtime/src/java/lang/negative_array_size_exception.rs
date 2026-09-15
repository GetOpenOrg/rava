#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/NegativeArraySizeException",
    super_class       = "java/lang/RuntimeException",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "NegativeArraySizeException.java",
    all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/NegativeArraySizeException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct NegativeArraySizeException {
    pub _super: RuntimeException,
}

impl NegativeArraySizeException {
    pub fn as_runtime_exception(&self) -> &RuntimeException { &self._super }
    pub fn into_runtime_exception(self) -> RuntimeException { self._super }
    pub fn as_exception(&self) -> &Exception { &self._super._super }
    pub fn into_exception(self) -> Exception { self._super._super }
    pub fn as_throwable(&self) -> &Throwable { &self._super._super._super }
    pub fn into_throwable(self) -> Throwable { self._super._super._super }
}

impl From<NegativeArraySizeException> for RuntimeException {
    fn from(v: NegativeArraySizeException) -> RuntimeException { v._super }
}

impl From<NegativeArraySizeException> for Exception {
    fn from(v: NegativeArraySizeException) -> Exception { v._super._super }
}

impl From<NegativeArraySizeException> for Throwable {
    fn from(v: NegativeArraySizeException) -> Throwable { v._super._super._super }
}

impl NegativeArraySizeException {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-8960118058596991861"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        -8960118058596991861i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = RuntimeException::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str(s: String) -> Result<Self> {
        panic!("stub: java/lang/NegativeArraySizeException.<init>:(Ljava/lang/String;)V")
    }
}
