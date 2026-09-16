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
    binary_name       = "java/lang/VirtualMachineError",
    super_class       = "java/lang/Error",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "VirtualMachineError.java",
    all_supertypes    = "java/io/Serializable;java/lang/Error;java/lang/Object;java/lang/Throwable;java/lang/VirtualMachineError",
)]
#[derive(Clone, Default, PartialEq)]
pub struct VirtualMachineError {
    pub _super: Error,
}

impl VirtualMachineError {
    pub fn as_error(&self) -> &Error { &self._super }
    pub fn into_error(self) -> Error { self._super }
    pub fn as_throwable(&self) -> &Throwable { &self._super._super }
    pub fn into_throwable(self) -> Throwable { self._super._super }
}

impl From<VirtualMachineError> for Error {
    fn from(v: VirtualMachineError) -> Error { v._super }
}

impl From<VirtualMachineError> for Throwable {
    fn from(v: VirtualMachineError) -> Throwable { v._super._super }
}

impl VirtualMachineError {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4161983926571568670"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        4161983926571568670i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = Error::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new_str(mut message: String) -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = Error::new_str(Clone::clone(&message))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str_throwa(message: String, cause: Throwable) -> Result<Self> {
        panic!("stub: java/lang/VirtualMachineError.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_throwa(cause: Throwable) -> Result<Self> {
        panic!("stub: java/lang/VirtualMachineError.<init>:(Ljava/lang/Throwable;)V")
    }
}
