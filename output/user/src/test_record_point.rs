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
use crate::test_record::TestRecord;
use crate::test_record_person::TestRecord_Person;

impl From<TestRecord_Point> for Record {
    fn from(v: TestRecord_Point) -> Record { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestRecord$Point"]
    #[super_class       = "java/lang/Record"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestRecord.java"]
    #[inner_classes     = "TestRecord$Point:TestRecord:Point:24;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Record"]
    #[all_supertypes    = "TestRecord$Point;java/lang/Object;java/lang/Record"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct TestRecord_Point {
        #[cfg_attr(any(), java_field(name = "x", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub x: i32,
        #[cfg_attr(any(), java_field(name = "y", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub y: i32,
    }

    impl TestRecord_Point {
        #[java_method(name = "<init>", descriptor = "(II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = "x:0;y:0")]
        pub fn new(mut x: i32, mut y: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Record::new()?);
            this.__set_x(x);
            this.__set_y(y);
            Ok(this)
        }

        #[java_method(name = "distance", descriptor = "()D", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn distance(&self) -> Result<f64> {
            let this = self;
            let _t0: f64 = Math::sqrt((((this.__get_x()).wrapping_mul(this.__get_x())).wrapping_add((this.__get_y()).wrapping_mul(this.__get_y())) as f64))?;
            Ok(_t0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
                Ok(String::from(format!("Point[x={}, y={}]", self.x.get(), self.y.get()).as_str()))
            }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
                Ok(0)
            }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut o: Object) -> Result<bool> {
                if let Some(other) = o.0.downcast_ref::<Self>() {
                    Ok(self.__get_x() == other.__get_x() && self.__get_y() == other.__get_y())
                } else {
                    Ok(false)
                }
            }

        #[java_method(name = "x", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn x(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_x())
        }

        #[java_method(name = "y", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn y(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_y())
        }
    }
}
