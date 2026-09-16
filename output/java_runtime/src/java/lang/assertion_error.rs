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
    binary_name       = "java/lang/AssertionError",
    super_class       = "java/lang/Error",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AssertionError.java",
    all_supertypes    = "java/io/Serializable;java/lang/AssertionError;java/lang/Error;java/lang/Object;java/lang/Throwable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AssertionError {
    pub _super: Error,
}

impl AssertionError {
    pub fn as_error(&self) -> &Error { &self._super }
    pub fn into_error(self) -> Error { self._super }
    pub fn as_throwable(&self) -> &Throwable { &self._super._super }
    pub fn into_throwable(self) -> Throwable { self._super._super }
}

impl From<AssertionError> for Error {
    fn from(v: AssertionError) -> Error { v._super }
}

impl From<AssertionError> for Throwable {
    fn from(v: AssertionError) -> Throwable { v._super._super }
}

impl AssertionError {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-5013299493970297370"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        -5013299493970297370i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = Error::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new_str(mut detailMessage: String) -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = Error::new_str(Clone::clone(&detailMessage))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/Object;)V
    pub fn new_obj(mut detailMessage: Object) -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this = AssertionError::new_str(Clone::clone(&String::from_owned(format!("{}", detailMessage))))?;
        if (detailMessage.is_instance_of("java/lang/Throwable")) {
            let _t0 = this._super._super.initCause(Clone::clone(&(detailMessage).downcast::<Throwable>()))?;
        }
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_z(detailMessage: bool) -> Result<Self> {
        panic!("stub: java/lang/AssertionError.<init>:(Z)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_c(detailMessage: u16) -> Result<Self> {
        panic!("stub: java/lang/AssertionError.<init>:(C)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_i(detailMessage: i32) -> Result<Self> {
        panic!("stub: java/lang/AssertionError.<init>:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_l(detailMessage: i64) -> Result<Self> {
        panic!("stub: java/lang/AssertionError.<init>:(J)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(F)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_f(detailMessage: f32) -> Result<Self> {
        panic!("stub: java/lang/AssertionError.<init>:(F)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_d(detailMessage: f64) -> Result<Self> {
        panic!("stub: java/lang/AssertionError.<init>:(D)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str_throwa(message: String, cause: Throwable) -> Result<Self> {
        panic!("stub: java/lang/AssertionError.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }
}
