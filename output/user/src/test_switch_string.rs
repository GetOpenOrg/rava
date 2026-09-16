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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestSwitchString"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestSwitchString.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestSwitchString;java/lang/Object"]

    pub struct TestSwitchString;

    impl TestSwitchString {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "classify", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn classify(mut s: String) -> Result<String> {
            let mut local_1: String = s;
            let mut local_2: i32 = -1i32;
            let _t0 = local_1.hashCode()?;
            match _t0 {
                -734239628 => {
                    let _t1 = local_1.equals(Object::from_any(String::from("yellow").clone()))?;
                    if _t1 {
                        local_2 = 3i32;
                    }
                }
                112785 => {
                    let _t1 = local_1.equals(Object::from_any(String::from("red").clone()))?;
                    local_2 = 0i32;
                }
                3027034 => {
                    let _t1 = local_1.equals(Object::from_any(String::from("blue").clone()))?;
                    local_2 = 1i32;
                }
                98619139 => {
                    let _t1 = local_1.equals(Object::from_any(String::from("green").clone()))?;
                    local_2 = 2i32;
                }
                _ => {
                }
            }
            let _switch_key = local_2;
            return Ok(String::from("primary-ish"));
            return Ok(String::from("bright"));
            Ok(String::from("other"))
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let _t0: String = Self::classify(Clone::clone(&String::from("red")))?;
            System::out().println_v(Clone::clone(&_t0))?;
            let _t1: String = Self::classify(Clone::clone(&String::from("blue")))?;
            System::out().println_v(Clone::clone(&_t1))?;
            let _t2: String = Self::classify(Clone::clone(&String::from("yellow")))?;
            System::out().println_v(Clone::clone(&_t2))?;
            let _t3: String = Self::classify(Clone::clone(&String::from("purple")))?;
            System::out().println_v(Clone::clone(&_t3))?;
            let _t4: String = Self::classify(Clone::clone(&String::from("green")))?;
            System::out().println_v(Clone::clone(&_t4))?;
            Ok(())
        }
    }
}
