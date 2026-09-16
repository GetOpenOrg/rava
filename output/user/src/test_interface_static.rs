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
use crate::test_interface_static_math_utils::TestInterfaceStatic_MathUtils;
use crate::test_interface_static_string_utils::TestInterfaceStatic_StringUtils;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestInterfaceStatic"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestInterfaceStatic.java"]
    #[inner_classes     = "TestInterfaceStatic$MathUtils:TestInterfaceStatic:MathUtils:1544;TestInterfaceStatic$StringUtils:TestInterfaceStatic:StringUtils:1544"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestInterfaceStatic;java/lang/Object"]

    pub struct TestInterfaceStatic;

    impl TestInterfaceStatic {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let _t0: i32 = TestInterfaceStatic_MathUtils::square(5i32)?;
            System::out().println_v(_t0)?;
            let _t1: i32 = TestInterfaceStatic_MathUtils::cube(3i32)?;
            System::out().println_v(_t1)?;
            let _t2: bool = TestInterfaceStatic_MathUtils::isPrime(7i32)?;
            System::out().println_v(_t2)?;
            let _t3: bool = TestInterfaceStatic_MathUtils::isPrime(9i32)?;
            System::out().println_v(_t3)?;
            let _t4: bool = TestInterfaceStatic_MathUtils::isPrime(2i32)?;
            System::out().println_v(_t4)?;
            let _t5: i32 = TestInterfaceStatic_MathUtils::clamp(15i32, 0i32, 10i32)?;
            System::out().println_v(_t5)?;
            let _t6: i32 = TestInterfaceStatic_MathUtils::clamp(-5i32, 0i32, 10i32)?;
            System::out().println_v(_t6)?;
            let _t7: i32 = TestInterfaceStatic_MathUtils::clamp(5i32, 0i32, 10i32)?;
            System::out().println_v(_t7)?;
            let _t8: String = TestInterfaceStatic_StringUtils::repeat(Clone::clone(&String::from("ab")), 3i32)?;
            System::out().println_v(Clone::clone(&_t8))?;
            let _t9: bool = TestInterfaceStatic_StringUtils::isPalindrome(Clone::clone(&String::from("racecar")))?;
            System::out().println_v(_t9)?;
            let _t10: bool = TestInterfaceStatic_StringUtils::isPalindrome(Clone::clone(&String::from("hello")))?;
            System::out().println_v(_t10)?;
            let _t11: String = TestInterfaceStatic_StringUtils::capitalize(Clone::clone(&String::from("hELLO")))?;
            System::out().println_v(Clone::clone(&_t11))?;
            let _t12: String = TestInterfaceStatic_StringUtils::capitalize(Clone::clone(&String::from("")))?;
            System::out().println_v(Clone::clone(&_t12))?;
            Ok(())
        }
    }
}
