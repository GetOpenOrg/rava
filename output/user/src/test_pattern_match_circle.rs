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
use crate::test_pattern_match_shape::TestPatternMatch_Shape;
use crate::test_pattern_match::TestPatternMatch;
use crate::test_pattern_match_rectangle::TestPatternMatch_Rectangle;
use crate::test_pattern_match_triangle::TestPatternMatch_Triangle;

impl From<TestPatternMatch_Circle> for Record {
    fn from(v: TestPatternMatch_Circle) -> Record { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestPatternMatch$Circle"]
    #[super_class       = "java/lang/Record"]
    #[interfaces        = "TestPatternMatch$Shape"]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestPatternMatch.java"]
    #[inner_classes     = "TestPatternMatch$Circle:TestPatternMatch:Circle:24;TestPatternMatch$Shape:TestPatternMatch:Shape:1544;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Record"]
    #[all_supertypes    = "TestPatternMatch$Circle;TestPatternMatch$Shape;java/lang/Object;java/lang/Record"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct TestPatternMatch_Circle {
        #[cfg_attr(any(), java_field(name = "radius", descriptor = "D", access = "private", modifiers = "final", is_static = false))]
        pub radius: f64,
    }

    impl TestPatternMatch_Circle {
        #[java_method(name = "<init>", descriptor = "(D)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = "radius:0")]
        pub fn new(mut radius: f64) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Record::new()?);
            this.__set_radius(radius);
            Ok(this)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
                Ok(String::from(format!("Circle[radius={}]", self.radius.get()).as_str()))
            }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
                Ok(0)
            }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut o: Object) -> Result<bool> {
                if let Some(other) = o.0.downcast_ref::<Self>() {
                    Ok(self.__get_radius() == other.__get_radius())
                } else {
                    Ok(false)
                }
            }

        #[java_method(name = "radius", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn radius(&self) -> Result<f64> {
            let this = self;
            Ok(this.__get_radius())
        }
    }
}
