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
    binary_name       = "java/lang/Record",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Record.java",
    all_supertypes    = "java/lang/Object;java/lang/Record",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Record;

impl Record {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/lang/Record.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Record.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Record.toString:()Ljava/lang/String;")
    }
}
