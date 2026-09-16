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
    #[binary_name       = "TestStringRegex"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestStringRegex.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestStringRegex;java/lang/Object"]

    pub struct TestStringRegex;

    impl TestStringRegex {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let _t0 = String::from("hello123").matches(Clone::clone(&String::from("[a-z]+\\d+")))?;
            System::out().println_v(_t0)?;
            let _t1 = String::from("hello").matches(Clone::clone(&String::from("[a-z]+\\d+")))?;
            System::out().println_v(_t1)?;
            let _t2 = String::from("123").matches(Clone::clone(&String::from("\\d+")))?;
            System::out().println_v(_t2)?;
            let mut s: String = String::from("foo bar  baz   qux");
            let _t3 = s.replaceAll(Clone::clone(&String::from("\\s+")), Clone::clone(&String::from(" ")))?;
            let _t4 = _t3.trim()?;
            System::out().println_v(Clone::clone(&_t4))?;
            let mut digits: String = String::from("a1b2c3d4");
            let _t5 = digits.replaceAll(Clone::clone(&String::from("[a-z]")), Clone::clone(&String::from("")))?;
            System::out().println_v(Clone::clone(&_t5))?;
            let _t6 = digits.replaceAll(Clone::clone(&String::from("\\d")), Clone::clone(&String::from("")))?;
            System::out().println_v(Clone::clone(&_t6))?;
            let mut csv: String = String::from("one,two,three,four");
            let _t7 = csv.split_str(Clone::clone(&String::from(",")))?;
            let mut parts: Rc<RefCell<Vec<String>>> = _t7;
            let mut limited: Rc<RefCell<Vec<String>>> = parts;
            let mut email = (limited.borrow().len() as i32);
            let mut m: i32 = 0i32;
            loop {
                if m >= email { break; }
                let mut p = Clone::clone(&limited.borrow()[m as usize]);
                System::out().println_v(Clone::clone(&p))?;
                m = m.wrapping_add(1i32);
            }
            let _t8 = csv.split_str_i(Clone::clone(&String::from(",")), 2i32)?;
            limited = _t8;
            System::out().println_v((limited.borrow().len() as i32))?;
            System::out().println_v(Clone::clone(&Clone::clone(&limited.borrow()[0i32 as usize])))?;
            System::out().println_v(Clone::clone(&Clone::clone(&limited.borrow()[1i32 as usize])))?;
            let _t9: Pattern = Pattern::compile_str(Clone::clone(&String::from("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}")))?;
            let mut email: Pattern = _t9;
            let _t10 = email.matcher(Object::from_any(String::from("contact: user@example.com and info@test.org").clone()))?;
            let mut m: Matcher = _t10;
            let mut p: i32 = 0i32;
            loop {
                let _t11 = m.find()?;
                if !(_t11) { break; }
                let _t11 = m.group()?;
                System::out().println_v(Clone::clone(&_t11))?;
                p = p.wrapping_add(1i32);
            }
            System::out().println_v(Clone::clone(&String::from_owned(format!("found: {}", p))))?;
            let _t11: Pattern = Pattern::compile_str(Clone::clone(&String::from("(\\d{4})-(\\d{2})-(\\d{2})")))?;
            let mut datePattern: Pattern = _t11;
            let _t12 = datePattern.matcher(Object::from_any(String::from("Today is 2026-09-15.").clone()))?;
            let mut dm: Matcher = _t12;
            let _t13 = dm.find()?;
            if _t13 {
                let _t14 = dm.group_i(1i32)?;
                System::out().println_v(Clone::clone(&_t14))?;
                let _t15 = dm.group_i(2i32)?;
                System::out().println_v(Clone::clone(&_t15))?;
                let _t16 = dm.group_i(3i32)?;
                System::out().println_v(Clone::clone(&_t16))?;
            }
            let mut str: String = String::from("apple banana apple cherry");
            let _t14 = str.replaceFirst(Clone::clone(&String::from("apple")), Clone::clone(&String::from("mango")))?;
            System::out().println_v(Clone::clone(&_t14))?;
            Ok(())
        }
    }
}
