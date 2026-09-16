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
    #[binary_name       = "TestRecursion"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestRecursion.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestRecursion;java/lang/Object"]

    pub struct TestRecursion;

    impl TestRecursion {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "fibonacci", descriptor = "(I)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fibonacci(mut n: i32) -> Result<i64> {
            if n <= 1i32 {
                return Ok((n as i64));
            }
            let _t0: i64 = Self::fibonacci((n).wrapping_sub(1i32))?;
            let _t1: i64 = Self::fibonacci((n).wrapping_sub(2i32))?;
            Ok((_t0).wrapping_add(_t1))
        }

        #[java_method(name = "factorial", descriptor = "(I)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn factorial(mut n: i32) -> Result<i64> {
            if n <= 1i32 {
                return Ok(1i64);
            }
            let _t0: i64 = Self::factorial((n).wrapping_sub(1i32))?;
            Ok(((n as i64)).wrapping_mul(_t0))
        }

        #[java_method(name = "gcd", descriptor = "(II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn gcd(mut a: i32, mut b: i32) -> Result<i32> {
            if (b==0) {
                return Ok(a);
            }
            let _t0: i32 = Self::gcd(b, (a%b))?;
            Ok(_t0)
        }

        #[java_method(name = "power", descriptor = "(II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn power(mut base: i32, mut exp: i32) -> Result<i32> {
            if (exp==0) {
                return Ok(1i32);
            }
            if ((exp%2i32)==0) {
                let _t0: i32 = Self::power(base, (exp/2i32))?;
                let mut half: i32 = _t0;
                return Ok((half).wrapping_mul(half));
            }
            let _t0: i32 = Self::power(base, (exp).wrapping_sub(1i32))?;
            Ok((base).wrapping_mul(_t0))
        }

        #[java_method(name = "sumDigits", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sumDigits(mut n: i32) -> Result<i32> {
            if n < 10i32 {
                return Ok(n);
            }
            let _t0: i32 = Self::sumDigits((n/10i32))?;
            Ok(((n%10i32)).wrapping_add(_t0))
        }

        #[java_method(name = "reverse", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reverse(mut s: String) -> Result<String> {
            let _t0 = s.isEmpty()?;
            if _t0 {
                return Ok(s);
            }
            let _t1 = s.substring_i(1i32)?;
            let _t2: String = Self::reverse(Clone::clone(&_t1))?;
            let _t3 = s.charAt(0i32)?;
            Ok(String::from_owned(format!("{}{}", _t2, char::from_u32(_t3 as u32).unwrap_or('?'))))
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut i: i32 = 0i32;
            loop {
                if i > 10i32 { break; }
                let _t0: i64 = Self::fibonacci(i)?;
                System::out().print_l(_t0)?;
                if i < 10i32 {
                    System::out().print_str(Clone::clone(&String::from(" ")))?;
                }
                i = i.wrapping_add(1i32);
            }
            System::out().println()?;
            let _t0: i64 = Self::factorial(0i32)?;
            System::out().println_v(_t0)?;
            let _t1: i64 = Self::factorial(5i32)?;
            System::out().println_v(_t1)?;
            let _t2: i64 = Self::factorial(10i32)?;
            System::out().println_v(_t2)?;
            let _t3: i32 = Self::gcd(48i32, 18i32)?;
            System::out().println_v(_t3)?;
            let _t4: i32 = Self::gcd(100i32, 75i32)?;
            System::out().println_v(_t4)?;
            let _t5: i32 = Self::gcd(7i32, 13i32)?;
            System::out().println_v(_t5)?;
            let _t6: i32 = Self::power(2i32, 10i32)?;
            System::out().println_v(_t6)?;
            let _t7: i32 = Self::power(3i32, 5i32)?;
            System::out().println_v(_t7)?;
            let _t8: i32 = Self::power(5i32, 0i32)?;
            System::out().println_v(_t8)?;
            let _t9: i32 = Self::sumDigits(12345i32)?;
            System::out().println_v(_t9)?;
            let _t10: i32 = Self::sumDigits(999i32)?;
            System::out().println_v(_t10)?;
            let _t11: String = Self::reverse(Clone::clone(&String::from("hello")))?;
            System::out().println_v(Clone::clone(&_t11))?;
            let _t12: String = Self::reverse(Clone::clone(&String::from("abcde")))?;
            System::out().println_v(Clone::clone(&_t12))?;
            Ok(())
        }
    }
}
