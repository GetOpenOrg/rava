#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/NullPointerException",
    super_class       = "java/lang/RuntimeException",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "NullPointerException.java",
    all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/NullPointerException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct NullPointerException {
    pub _super: RuntimeException,
    #[cfg_attr(any(), java_field(name = "extendedMessageState", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
    pub extendedMessageState: JField<i32>,
    #[cfg_attr(any(), java_field(name = "extendedMessage", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
    pub extendedMessage: JField<String>,
}

impl NullPointerException {
    pub fn as_runtime_exception(&self) -> &RuntimeException { &self._super }
    pub fn into_runtime_exception(self) -> RuntimeException { self._super }
    pub fn as_exception(&self) -> &Exception { &self._super._super }
    pub fn into_exception(self) -> Exception { self._super._super }
    pub fn as_throwable(&self) -> &Throwable { &self._super._super._super }
    pub fn into_throwable(self) -> Throwable { self._super._super._super }
}

impl From<NullPointerException> for RuntimeException {
    fn from(v: NullPointerException) -> RuntimeException { v._super }
}

impl From<NullPointerException> for Exception {
    fn from(v: NullPointerException) -> Exception { v._super._super }
}

impl From<NullPointerException> for Throwable {
    fn from(v: NullPointerException) -> Throwable { v._super._super._super }
}

impl NullPointerException {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "5162710183389028792"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        5162710183389028792i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), extendedMessageState: JField::new(0), extendedMessage: JField::new(String::default()), ..Default::default() };
        this._super = RuntimeException::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new_str(mut s: String) -> Result<Self> {
        let mut this = Self { _super: Default::default(), extendedMessageState: JField::new(0), extendedMessage: JField::new(String::default()), ..Default::default() };
        this._super = RuntimeException::new_str(Clone::clone(&s))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "fillInStackTrace", descriptor = "()Ljava/lang/Throwable;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn fillInStackTrace(&self) -> Result<Throwable> {
        panic!("stub: java/lang/NullPointerException.fillInStackTrace:()Ljava/lang/Throwable;")
    }

    #[cfg_attr(any(), java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getMessage(&self) -> Result<String> {
        panic!("stub: java/lang/NullPointerException.getMessage:()Ljava/lang/String;")
    }
}
