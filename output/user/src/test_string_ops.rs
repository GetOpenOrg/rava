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
    #[binary_name       = "TestStringOps"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestStringOps.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestStringOps;java/lang/Object"]

    pub struct TestStringOps;

    impl TestStringOps {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut s: String = String::from("  Hello, World!  ");
            let _t0 = s.trim()?;
            System::out().println_v(Clone::clone(&_t0))?;
            let _t1 = s.strip()?;
            System::out().println_v(Clone::clone(&_t1))?;
            let _t2 = s.trim()?;
            let _t3 = _t2.toLowerCase()?;
            System::out().println_v(Clone::clone(&_t3))?;
            let _t4 = s.trim()?;
            let _t5 = _t4.toUpperCase()?;
            System::out().println_v(Clone::clone(&_t5))?;
            let _t6 = s.trim()?;
            let _t7 = _t6.replace_seq_seq(Object::from_any(String::from("World").clone()), Object::from_any(String::from("Java").clone()))?;
            System::out().println_v(Clone::clone(&_t7))?;
            let _t8 = s.trim()?;
            let _t9 = _t8.contains(Object::from_any(String::from("World").clone()))?;
            System::out().println_v(_t9)?;
            let _t10 = s.trim()?;
            let _t11 = _t10.startsWith_str(Clone::clone(&String::from("Hello")))?;
            System::out().println_v(_t11)?;
            let _t12 = s.trim()?;
            let _t13 = _t12.endsWith(Clone::clone(&String::from("!")))?;
            System::out().println_v(_t13)?;
            let _t14 = s.trim()?;
            let _t15 = _t14.indexOf_str(Clone::clone(&String::from("o")))?;
            System::out().println_v(_t15)?;
            let _t16 = s.trim()?;
            let _t17 = _t16.substring_i(7i32)?;
            System::out().println_v(Clone::clone(&_t17))?;
            let _t18 = String::from("a,b,c,d").split_str(Clone::clone(&String::from(",")))?;
            let mut parts: Rc<RefCell<Vec<String>>> = _t18;
            System::out().println_v((parts.borrow().len() as i32))?;
            let _t19: String = String::join_seq_arr_seq(Object::from_any(String::from("-").clone()), Default::default())?;
            System::out().println_v(Clone::clone(&_t19))?;
            let _t20 = String::from("abc").repeat(3i32)?;
            System::out().println_v(Clone::clone(&_t20))?;
            let _t21 = String::from("  ").isBlank()?;
            System::out().println_v(_t21)?;
            let _t22 = String::from("hello").isBlank()?;
            System::out().println_v(_t22)?;
            Ok(())
        }
    }
}
