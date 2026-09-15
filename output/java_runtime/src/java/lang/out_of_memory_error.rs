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
    binary_name       = "java/lang/OutOfMemoryError",
    super_class       = "java/lang/VirtualMachineError",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "OutOfMemoryError.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct OutOfMemoryError {
    pub _super: VirtualMachineError,
}

impl OutOfMemoryError {
    pub fn as_virtual_machine_error(&self) -> &VirtualMachineError { &self._super }
    pub fn into_virtual_machine_error(self) -> VirtualMachineError { self._super }
    pub fn as_error(&self) -> &Error { &self._super._super }
    pub fn into_error(self) -> Error { self._super._super }
    pub fn as_throwable(&self) -> &Throwable { &self._super._super._super }
    pub fn into_throwable(self) -> Throwable { self._super._super._super }
}

impl From<OutOfMemoryError> for VirtualMachineError {
    fn from(v: OutOfMemoryError) -> VirtualMachineError { v._super }
}

impl From<OutOfMemoryError> for Error {
    fn from(v: OutOfMemoryError) -> Error { v._super._super }
}

impl From<OutOfMemoryError> for Throwable {
    fn from(v: OutOfMemoryError) -> Throwable { v._super._super._super }
}

impl OutOfMemoryError {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "8228564086184010517"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        8228564086184010517i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = VirtualMachineError::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new_str(mut s: String) -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = VirtualMachineError::new_str(Clone::clone(&s))?;
        Ok(this)
    }
}
