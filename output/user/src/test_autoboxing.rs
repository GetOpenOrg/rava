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
    #[binary_name       = "TestAutoboxing"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestAutoboxing.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestAutoboxing;java/lang/Object"]

    pub struct TestAutoboxing;

    impl TestAutoboxing {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut i: i32 = 42i32;
            let mut d: f64 = 3.14f64;
            let mut b: i32 = 1i32;
            let mut l: i64 = 100i64;
            let mut pi: i32 = i;
            let mut pd: f64 = d;
            let _t0: bool = b.booleanValue()?;
            let mut pb = (_t0) as i32;
            let mut pl: i64 = l;
            System::out().println_v(pi)?;
            System::out().println_v(pd)?;
            System::out().println_v((pb != 0i32))?;
            System::out().println_v(pl)?;
            let mut x: i32 = 10i32;
            let mut y: i32 = 20i32;
            let mut sum = (x).wrapping_add(y);
            System::out().println_v(sum)?;
            let mut product = (x).wrapping_mul(y);
            System::out().println_v(product.into())?;
            let mut a: i32 = 100i32;
            let mut a2: i32 = 100i32;
            System::out().println_v(a == a2)?;
            let _t1 = a.equals(a2.into())?;
            System::out().println_v(_t1)?;
            let mut big: i32 = 200i32;
            let mut big2: i32 = 200i32;
            let _t2 = big.equals(big2.into())?;
            System::out().println_v(_t2)?;
            let mut nullable: Object = Object::default();
            System::out().println_v(_is_jnull(&nullable))?;
            let _t3: String = Integer::toString_i(255i32)?;
            System::out().println_v(Clone::clone(&_t3))?;
            let _t4: String = Integer::toBinaryString(10i32)?;
            System::out().println_v(Clone::clone(&_t4))?;
            let _t5: String = Integer::toHexString(255i32)?;
            System::out().println_v(Clone::clone(&_t5))?;
            let _t6: i32 = Integer::parseInt_str(Clone::clone(&String::from("123")))?;
            let mut parsed: i32 = _t6;
            System::out().println_v(parsed)?;
            System::out().println_v(-2147483648i32)?;
            System::out().println_v(2147483647i32)?;
            Ok(())
        }
    }
}
