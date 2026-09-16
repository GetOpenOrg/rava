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
use crate::test_default_methods_drawable::TestDefaultMethods_Drawable;
use crate::test_default_methods_colorable::TestDefaultMethods_Colorable;
use crate::test_default_methods::TestDefaultMethods;
use crate::test_default_methods_formal_person::TestDefaultMethods_FormalPerson;
use crate::test_default_methods_informal_person::TestDefaultMethods_InformalPerson;
use crate::test_default_methods_formal_greetable::TestDefaultMethods_FormalGreetable;
use crate::test_default_methods_greetable::TestDefaultMethods_Greetable;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestDefaultMethods$Circle"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "TestDefaultMethods$Drawable,TestDefaultMethods$Colorable"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestDefaultMethods.java"]
    #[inner_classes     = "TestDefaultMethods$Circle:TestDefaultMethods:Circle:8;TestDefaultMethods$Drawable:TestDefaultMethods:Drawable:1544;TestDefaultMethods$Colorable:TestDefaultMethods:Colorable:1544;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestDefaultMethods$Circle;TestDefaultMethods$Colorable;TestDefaultMethods$Drawable;java/lang/Object"]

    pub struct TestDefaultMethods_Circle {
        #[cfg_attr(any(), java_field(name = "radius", descriptor = "D", is_static = false))]
        pub radius: f64,
        #[cfg_attr(any(), java_field(name = "color", descriptor = "Ljava/lang/String;", is_static = false))]
        pub color: String,
    }

    impl TestDefaultMethods_Circle {
        #[java_method(name = "<init>", descriptor = "(DLjava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut radius: f64, mut color: String) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_radius(radius);
            this.__set_color(Clone::clone(&color));
            Ok(this)
        }

        #[java_method(name = "draw", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn draw(&self) -> Result<()> {
            let this = self;
            System::out().println_v(Clone::clone(&String::from_owned(format!("Circle r={}", java_fmt_f64(this.__get_radius())))))?;
            Ok(())
        }

        #[java_method(name = "getColor", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getColor(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_color())
        }

        #[java_method(name = "drawTwice", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn drawTwice(&self) -> Result<()> {
            let this = self;
            this.draw()?;
            this.draw()?;
            Ok(())
        }

        #[java_method(name = "getType", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getType(&self) -> Result<String> {
            let this = self;
            Ok(String::from("Drawable"))
        }

        #[java_method(name = "describe", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn describe(&self) -> Result<String> {
            let this = self;
            let _t0 = this.getColor()?;
            let _t1 = this.getClass()?;
            let _vdispatch2: String = if let Some(__f) = _t1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            Ok(String::from_owned(format!("{} {}", _t0, _vdispatch2)))
        }
    }
}
