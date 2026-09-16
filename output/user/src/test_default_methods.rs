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
use crate::test_default_methods_circle::TestDefaultMethods_Circle;
use crate::test_default_methods_formal_person::TestDefaultMethods_FormalPerson;
use crate::test_default_methods_informal_person::TestDefaultMethods_InformalPerson;
use crate::test_default_methods_drawable::TestDefaultMethods_Drawable;
use crate::test_default_methods_formal_greetable::TestDefaultMethods_FormalGreetable;
use crate::test_default_methods_greetable::TestDefaultMethods_Greetable;
use crate::test_default_methods_colorable::TestDefaultMethods_Colorable;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestDefaultMethods"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestDefaultMethods.java"]
    #[inner_classes     = "TestDefaultMethods$Circle:TestDefaultMethods:Circle:8;TestDefaultMethods$FormalPerson:TestDefaultMethods:FormalPerson:8;TestDefaultMethods$InformalPerson:TestDefaultMethods:InformalPerson:8;TestDefaultMethods$Drawable:TestDefaultMethods:Drawable:1544;TestDefaultMethods$FormalGreetable:TestDefaultMethods:FormalGreetable:1544;TestDefaultMethods$Greetable:TestDefaultMethods:Greetable:1544;TestDefaultMethods$Colorable:TestDefaultMethods:Colorable:1544"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestDefaultMethods;java/lang/Object"]

    pub struct TestDefaultMethods;

    impl TestDefaultMethods {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut c = TestDefaultMethods_Circle::new(5.0f64, Clone::clone(&String::from("red")))?;
            c.drawTwice()?;
            let _t0 = c.getType()?;
            System::out().println_v(Clone::clone(&_t0))?;
            let _t1 = c.describe()?;
            System::out().println_v(Clone::clone(&_t1))?;
            let mut fp = TestDefaultMethods_FormalPerson::new()?;
            let mut ip = TestDefaultMethods_InformalPerson::new()?;
            let _t2 = fp.greet(Clone::clone(&String::from("Alice")))?;
            System::out().println_v(Clone::clone(&_t2))?;
            let _t3 = ip.greet(Clone::clone(&String::from("Bob")))?;
            System::out().println_v(Clone::clone(&_t3))?;
            let mut d: TestDefaultMethods_Circle = c;
            d.draw()?;
            d.drawTwice()?;
            Ok(())
        }
    }
}
