#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use java_runtime::prelude::*;
use java_runtime::java::io::*;
use java_runtime::java::lang::*;
use java_runtime::java::security::*;
use java_runtime::java::util::*;
use java_runtime::java::util::stream::*;
use java_runtime::sun::nio::ch::*;
use java_runtime::sun::nio::cs::*;
use java_runtime::sun::security::util::*;
use crate::test_stream_collectors::TestStreamCollectors;

#[java_rta_macros::java_class(
    binary_name       = "TestStreamCollectors$Person",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "TestStreamCollectors.java",
    inner_classes     = "TestStreamCollectors$Person:TestStreamCollectors:Person:8",
    all_supertypes    = "TestStreamCollectors$Person;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct TestStreamCollectors_Person {
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", is_static = false))]
    pub name: JField<String>,
    #[cfg_attr(any(), java_field(name = "age", descriptor = "I", is_static = false))]
    pub age: JField<i32>,
    #[cfg_attr(any(), java_field(name = "dept", descriptor = "Ljava/lang/String;", is_static = false))]
    pub dept: JField<String>,
}

impl TestStreamCollectors_Person {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;ILjava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(mut name: String, mut age: i32, mut dept: String) -> Result<Self> {
        let mut this = Self { name: JField::new(String::default()), age: JField::new(0), dept: JField::new(String::default()), ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.name.set(Clone::clone(&name));
        this.age.set(age);
        this.dept.set(Clone::clone(&dept));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }
}
