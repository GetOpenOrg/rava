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
use crate::test_comparable_student::TestComparable_Student;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestComparable$Version"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Comparable"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Ljava/lang/Comparable<LTestComparable$Version;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestComparable.java"]
    #[inner_classes     = "TestComparable$Version:TestComparable:Version:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestComparable$Version;java/lang/Comparable;java/lang/Object"]
    #[has_to_string_method = true]

    pub struct TestComparable_Version {
        #[cfg_attr(any(), java_field(name = "major", descriptor = "I", is_static = false))]
        pub major: i32,
        #[cfg_attr(any(), java_field(name = "minor", descriptor = "I", is_static = false))]
        pub minor: i32,
        #[cfg_attr(any(), java_field(name = "patch", descriptor = "I", is_static = false))]
        pub patch: i32,
    }

    impl TestComparable_Version {
        #[java_method(name = "<init>", descriptor = "(III)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut major: i32, mut minor: i32, mut patch: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_major(major);
            this.__set_minor(minor);
            this.__set_patch(patch);
            Ok(this)
        }

        #[java_method(name = "compareTo", descriptor = "(LTestComparable$Version;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut other: TestComparable_Version) -> Result<i32> {
            let this = self;
            if this.__get_major() != other.__get_major() {
                let _t0: i32 = Integer::compare(this.__get_major(), other.__get_major())?;
                return Ok(_t0);
            }
            if this.__get_minor() != other.__get_minor() {
                let _t0: i32 = Integer::compare(this.__get_minor(), other.__get_minor())?;
                return Ok(_t0);
            }
            let _t0: i32 = Integer::compare(this.__get_patch(), other.__get_patch())?;
            Ok(_t0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            Ok(String::from_owned(format!("{}.{}.{}", this.__get_major(), this.__get_minor(), this.__get_patch())))
        }
    }
}
