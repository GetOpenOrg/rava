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
    binary_name       = "java/lang/ArrayIndexOutOfBoundsException",
    super_class       = "java/lang/IndexOutOfBoundsException",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ArrayIndexOutOfBoundsException.java",
    all_supertypes    = "java/io/Serializable;java/lang/ArrayIndexOutOfBoundsException;java/lang/Exception;java/lang/IndexOutOfBoundsException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct ArrayIndexOutOfBoundsException {
    pub _super: IndexOutOfBoundsException,
}

impl ArrayIndexOutOfBoundsException {
    pub fn as_index_out_of_bounds_exception(&self) -> &IndexOutOfBoundsException { &self._super }
    pub fn into_index_out_of_bounds_exception(self) -> IndexOutOfBoundsException { self._super }
    pub fn as_runtime_exception(&self) -> &RuntimeException { &self._super._super }
    pub fn into_runtime_exception(self) -> RuntimeException { self._super._super }
    pub fn as_exception(&self) -> &Exception { &self._super._super._super }
    pub fn into_exception(self) -> Exception { self._super._super._super }
    pub fn as_throwable(&self) -> &Throwable { &self._super._super._super._super }
    pub fn into_throwable(self) -> Throwable { self._super._super._super._super }
}

impl From<ArrayIndexOutOfBoundsException> for IndexOutOfBoundsException {
    fn from(v: ArrayIndexOutOfBoundsException) -> IndexOutOfBoundsException { v._super }
}

impl From<ArrayIndexOutOfBoundsException> for RuntimeException {
    fn from(v: ArrayIndexOutOfBoundsException) -> RuntimeException { v._super._super }
}

impl From<ArrayIndexOutOfBoundsException> for Exception {
    fn from(v: ArrayIndexOutOfBoundsException) -> Exception { v._super._super._super }
}

impl From<ArrayIndexOutOfBoundsException> for Throwable {
    fn from(v: ArrayIndexOutOfBoundsException) -> Throwable { v._super._super._super._super }
}

impl ArrayIndexOutOfBoundsException {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-5116101128118950844"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        -5116101128118950844i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = IndexOutOfBoundsException::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str(s: String) -> Result<Self> {
        panic!("stub: java/lang/ArrayIndexOutOfBoundsException.<init>:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(I)V
    pub fn new_i(mut index: i32) -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Array index out of range: ")))?;
        let _t1 = _t0.append_i(index)?;
        let _t2 = _t1.toString()?;
        this._super = IndexOutOfBoundsException::new_str(Clone::clone(&_t2))?;
        Ok(this)
    }
}
