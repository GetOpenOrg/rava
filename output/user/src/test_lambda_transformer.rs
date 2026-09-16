#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use java_runtime::prelude::*;
use java_runtime::java::io::*;
use java_runtime::java::lang::*;
use java_runtime::java::lang::reflect::*;
use java_runtime::java::security::*;
use java_runtime::java::util::*;
use java_runtime::java::util::function::*;
use java_runtime::sun::nio::ch::*;
use java_runtime::sun::nio::cs::*;
use java_runtime::sun::security::util::*;
use crate::test_lambda::TestLambda;

#[java_rta_macros::java_class(
    binary_name       = "TestLambda$Transformer",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "abstract interface",
    generic_signature = "",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "TestLambda.java",
    inner_classes     = "TestLambda$Transformer:TestLambda:Transformer:1544",
)]
#[derive(Clone, Default, PartialEq)]
pub struct TestLambda_Transformer;

impl TestLambda_Transformer {
    #[cfg_attr(any(), java_method(name = "transform", descriptor = "(I)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn transform(&self, arg0: i32) -> Result<i32> {
        panic!("stub: TestLambda$Transformer.transform:(I)I")
    }
}
