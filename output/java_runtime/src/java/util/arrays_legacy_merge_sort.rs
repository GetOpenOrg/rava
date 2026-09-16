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
    binary_name       = "java/util/Arrays$LegacyMergeSort",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Arrays.java",
    inner_classes     = "java/util/Arrays$LegacyMergeSort:java/util/Arrays:LegacyMergeSort:24",
    all_supertypes    = "java/lang/Object;java/util/Arrays$LegacyMergeSort",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Arrays_LegacyMergeSort;

impl Arrays_LegacyMergeSort {
    #[cfg_attr(any(), java_field(name = "userRequested", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
    // static field: userRequested:Z
    pub fn userRequested() -> bool {
        panic!("stub: java/util/Arrays$LegacyMergeSort.userRequested:Z")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/util/Arrays$LegacyMergeSort.<init>:()V")
    }
}
