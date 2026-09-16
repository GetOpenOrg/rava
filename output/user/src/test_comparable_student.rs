#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use java_runtime::prelude::*;
use java_runtime::java::io::*;
use java_runtime::java::lang::*;
use java_runtime::java::lang::r#ref::*;
use java_runtime::java::lang::reflect::*;
use java_runtime::java::math::*;
use java_runtime::java::nio::*;
use java_runtime::java::nio::charset::*;
use java_runtime::java::security::*;
use java_runtime::java::text::*;
use java_runtime::java::text::spi::*;
use java_runtime::java::time::*;
use java_runtime::java::time::chrono::*;
use java_runtime::java::time::temporal::*;
use java_runtime::java::time::zone::*;
use java_runtime::java::util::*;
use java_runtime::java::util::concurrent::*;
use java_runtime::java::util::concurrent::atomic::*;
use java_runtime::java::util::concurrent::locks::*;
use java_runtime::java::util::function::*;
use java_runtime::java::util::regex::*;
use java_runtime::java::util::spi::*;
use java_runtime::java::util::stream::*;
use java_runtime::java::util::zip::*;
use java_runtime::sun::nio::ch::*;
use java_runtime::sun::nio::cs::*;
use java_runtime::sun::reflect::generics::factory::*;
use java_runtime::sun::reflect::generics::repository::*;
use java_runtime::sun::reflect::generics::scope::*;
use java_runtime::sun::reflect::misc::*;
use java_runtime::sun::security::action::*;
use java_runtime::sun::security::util::*;
use java_runtime::sun::text::*;
use java_runtime::sun::util::*;
use java_runtime::sun::util::calendar::*;
use java_runtime::sun::util::locale::*;
use java_runtime::sun::util::locale::provider::*;
use java_runtime::sun::util::spi::*;
use java_runtime::java::text::Normalizer;
use crate::test_comparable::TestComparable;
use crate::test_comparable_version::TestComparable_Version;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestComparable$Student"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Comparable"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Ljava/lang/Comparable<LTestComparable$Student;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestComparable.java"]
    #[inner_classes     = "TestComparable$Student:TestComparable:Student:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestComparable$Student;java/lang/Comparable;java/lang/Object"]
    #[has_to_string_method = true]

    pub struct TestComparable_Student {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "gpa", descriptor = "D", is_static = false))]
        pub gpa: f64,
    }

    impl TestComparable_Student {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;D)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut name: String, mut gpa: f64) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_name(Clone::clone(&name));
            this.__set_gpa(gpa);
            Ok(this)
        }

        #[java_method(name = "compareTo", descriptor = "(LTestComparable$Student;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut other: TestComparable_Student) -> Result<i32> {
            let this = self;
            let _t0: i32 = Double::compare(other.__get_gpa(), this.__get_gpa())?;
            let mut cmp: i32 = _t0;
            if (cmp!=0) {
                return Ok(cmp);
            }
            let _t1 = this.__get_name().compareTo(Clone::clone(&other.__get_name()))?;
            Ok(_t1)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            Ok(String::from_owned(format!("{}:{}", this.__get_name(), java_fmt_f64(this.__get_gpa()))))
        }
    }
}
