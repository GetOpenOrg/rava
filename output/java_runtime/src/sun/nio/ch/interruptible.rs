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
    binary_name       = "sun/nio/ch/Interruptible",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Interruptible.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Interruptible;

impl Interruptible {
    #[cfg_attr(any(), java_method(name = "interrupt", descriptor = "(Ljava/lang/Thread;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn interrupt(&self, arg0: Thread) -> Result<()> {
        panic!("stub: sun/nio/ch/Interruptible.interrupt:(Ljava/lang/Thread;)V")
    }
}
