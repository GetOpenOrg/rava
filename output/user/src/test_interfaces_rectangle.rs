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
use crate::test_interfaces_drawable::TestInterfaces_Drawable;
use crate::test_interfaces::TestInterfaces;
use crate::test_interfaces_circle::TestInterfaces_Circle;
use crate::test_interfaces_resizable::TestInterfaces_Resizable;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestInterfaces$Rectangle"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "TestInterfaces$Drawable"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestInterfaces.java"]
    #[inner_classes     = "TestInterfaces$Rectangle:TestInterfaces:Rectangle:8;TestInterfaces$Drawable:TestInterfaces:Drawable:1544;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestInterfaces$Drawable;TestInterfaces$Rectangle;java/lang/Object"]

    pub struct TestInterfaces_Rectangle {
        #[cfg_attr(any(), java_field(name = "width", descriptor = "D", is_static = false))]
        pub width: f64,
        #[cfg_attr(any(), java_field(name = "height", descriptor = "D", is_static = false))]
        pub height: f64,
    }

    impl TestInterfaces_Rectangle {
        #[java_method(name = "<init>", descriptor = "(DD)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut width: f64, mut height: f64) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_width(width);
            this.__set_height(height);
            Ok(this)
        }

        #[java_method(name = "draw", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn draw(&self) -> Result<()> {
            let this = self;
            System::out().println_v(Clone::clone(&String::from_owned(format!("Rectangle {}x{}", java_fmt_f64(this.__get_width()), java_fmt_f64(this.__get_height())))))?;
            Ok(())
        }

        #[java_method(name = "description", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn description(&self) -> Result<String> {
            let this = self;
            Ok(String::from("a drawable object"))
        }
    }
}
