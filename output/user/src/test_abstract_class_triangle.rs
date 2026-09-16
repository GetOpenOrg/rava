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
use crate::test_abstract_class_shape::TestAbstractClass_Shape;
use crate::test_abstract_class::TestAbstractClass;
use crate::test_abstract_class_square::TestAbstractClass_Square;
use crate::test_abstract_class_rectangle::TestAbstractClass_Rectangle;

impl From<TestAbstractClass_Triangle> for TestAbstractClass_Shape {
    fn from(v: TestAbstractClass_Triangle) -> TestAbstractClass_Shape { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestAbstractClass$Triangle"]
    #[super_class       = "TestAbstractClass$Shape"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestAbstractClass.java"]
    #[inner_classes     = "TestAbstractClass$Shape:TestAbstractClass:Shape:1032;TestAbstractClass$Triangle:TestAbstractClass:Triangle:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "TestAbstractClass_Shape"]
    #[superclass_fields(color: String)]
    #[all_supertypes    = "TestAbstractClass$Shape;TestAbstractClass$Triangle;java/lang/Object"]

    pub struct TestAbstractClass_Triangle {
        #[cfg_attr(any(), java_field(name = "base", descriptor = "D", is_static = false))]
        pub base: f64,
        #[cfg_attr(any(), java_field(name = "height", descriptor = "D", is_static = false))]
        pub height: f64,
    }

    impl TestAbstractClass_Triangle {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;DD)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut color: String, mut base: f64, mut height: f64) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(TestAbstractClass_Shape::new(Clone::clone(&color))?);
            this.__set_base(base);
            this.__set_height(height);
            Ok(this)
        }

        #[java_method(name = "area", descriptor = "()D", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn area(&self) -> Result<f64> {
            let this = self;
            Ok(((0.5f64*this.__get_base())*this.__get_height()))
        }
    }
}
