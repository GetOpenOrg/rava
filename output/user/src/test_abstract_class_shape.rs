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
use crate::test_abstract_class::TestAbstractClass;
use crate::test_abstract_class_square::TestAbstractClass_Square;
use crate::test_abstract_class_rectangle::TestAbstractClass_Rectangle;
use crate::test_abstract_class_triangle::TestAbstractClass_Triangle;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestAbstractClass$Shape"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestAbstractClass.java"]
    #[inner_classes     = "TestAbstractClass$Shape:TestAbstractClass:Shape:1032;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestAbstractClass$Shape;java/lang/Object"]

    pub struct TestAbstractClass_Shape {
        #[cfg_attr(any(), java_field(name = "color", descriptor = "Ljava/lang/String;", is_static = false))]
        pub color: String,
    }

    impl TestAbstractClass_Shape {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut color: String) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_color(Clone::clone(&color));
            Ok(this)
        }

        #[java_method(name = "area", descriptor = "()D", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn area(&self) -> Result<f64> {
            panic!("stub: TestAbstractClass$Shape.area:()D")
        }

        #[java_method(name = "describe", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn describe(&self) -> Result<String> {
            let this = self;
            let _t0 = this.area()?;
            Ok(String::from_owned(format!("{} shape, area={}", this.__get_color(), java_fmt_f64(_t0))))
        }
    }
}
