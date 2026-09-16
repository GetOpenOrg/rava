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
use crate::jdk::internal::misc::VM;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Integer$IntegerCache",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Integer.java",
    inner_classes     = "java/lang/Integer$IntegerCache:java/lang/Integer:IntegerCache:26",
    all_supertypes    = "java/lang/Integer$IntegerCache;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Integer_IntegerCache;

impl Integer_IntegerCache {
    #[cfg_attr(any(), java_field(name = "low", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-128"))]
    // static field: low:I
    pub fn low() -> i32 {
        -128
    }

    #[cfg_attr(any(), java_field(name = "high", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
    // static field: high:I
    pub fn high() -> i32 {
        panic!("stub: java/lang/Integer$IntegerCache.high:I")
    }

    #[cfg_attr(any(), java_field(name = "cache", descriptor = "[Ljava/lang/Integer;", access = "package", modifiers = "static final", is_static = true))]
    // static field: cache:[Ljava/lang/Integer;
    pub fn cache() -> Rc<RefCell<Vec<i32>>> {
        panic!("stub: java/lang/Integer$IntegerCache.cache:[Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_field(name = "archivedCache", descriptor = "[Ljava/lang/Integer;", access = "package", modifiers = "static", is_static = true))]
    // static field: archivedCache:[Ljava/lang/Integer;
    pub fn archivedCache() -> Rc<RefCell<Vec<i32>>> {
        panic!("stub: java/lang/Integer$IntegerCache.archivedCache:[Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/Integer$IntegerCache.<init>:()V")
    }
}
