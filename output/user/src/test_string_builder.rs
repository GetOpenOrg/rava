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
    #[binary_name       = "TestStringBuilder"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestStringBuilder.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestStringBuilder;java/lang/Object"]

    pub struct TestStringBuilder;

    impl TestStringBuilder {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut first: String = String::from("Hello");
            let mut last: String = String::from("World");
            let mut full: String = String::from_owned(format!("{}, {}!", first, last));
            System::out().println_v(Clone::clone(&full))?;
            let _t0 = full.length()?;
            System::out().println_v(_t0)?;
            let _t1 = full.charAt(0i32)?;
            System::out().println_v(_t1)?;
            let _t2 = String::from("hello").toUpperCase()?;
            System::out().println_v(Clone::clone(&_t2))?;
            let _t3 = String::from("HELLO").toLowerCase()?;
            System::out().println_v(Clone::clone(&_t3))?;
            let _t4 = String::from("  hello  ").trim()?;
            System::out().println_v(Clone::clone(&_t4))?;
            let _t5 = String::from("hello").contains(Object::from_any(String::from("ell").clone()))?;
            System::out().println_v(_t5)?;
            let _t6 = String::from("hello world").replace_seq_seq(Object::from_any(String::from("world").clone()), Object::from_any(String::from("Java").clone()))?;
            System::out().println_v(Clone::clone(&_t6))?;
            let _t7 = String::from("hello").startsWith_str(Clone::clone(&String::from("he")))?;
            System::out().println_v(_t7)?;
            let _t8 = String::from("hello").endsWith(Clone::clone(&String::from("lo")))?;
            System::out().println_v(_t8)?;
            let _t9 = String::from("hello world").substring_i(6i32)?;
            System::out().println_v(Clone::clone(&_t9))?;
            let _t10 = String::from("a,b,c").split_str(Clone::clone(&String::from(",")))?;
            System::out().println_v((_t10.borrow().len() as i32))?;
            System::out().println_v(Clone::clone(&String::from_owned(format!("{}", 42i32))))?;
            System::out().println_v(Clone::clone(&String::from_owned(format!("{}", 1i32))))?;
            let _t11 = String::from("hello").equals(Object::from_any(String::from("hello").clone()))?;
            System::out().println_v(_t11)?;
            let _t12 = String::from("hello").equals(Object::from_any(String::from("world").clone()))?;
            System::out().println_v(_t12)?;
            let _t13 = String::from("hello").indexOf_str(Clone::clone(&String::from("ll")))?;
            System::out().println_v(_t13)?;
            let mut sb = StringBuilder::new()?;
            let _t14 = sb.append_str(Clone::clone(&String::from("foo")))?;
            let _t15 = sb.append_str(Clone::clone(&String::from("bar")))?;
            let _t16 = sb.append_i(42i32)?;
            let _t17 = sb.toString()?;
            System::out().println_v(Clone::clone(&_t17))?;
            let _t18 = sb.__super().length()?;
            System::out().println_v(_t18)?;
            let _t19 = StringBuilder::new()?.append_str(Clone::clone(&String::from("a")))?;
            let _t20 = _t19.append_str(Clone::clone(&String::from("b")))?;
            let _t21 = _t20.append_str(Clone::clone(&String::from("c")))?;
            let _t22 = _t21.toString()?;
            let mut chained: String = _t22;
            System::out().println_v(Clone::clone(&chained))?;
            let mut age: i32 = 25i32;
            System::out().println_v(Clone::clone(&String::from_owned(format!("age={}", age))))?;
            let mut pi: f64 = 3.14f64;
            System::out().println_v(Clone::clone(&String::from_owned(format!("pi={}", java_fmt_f64(pi)))))?;
            Ok(())
        }
    }
}
