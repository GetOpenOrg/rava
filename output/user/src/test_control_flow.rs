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
    #[binary_name       = "TestControlFlow"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestControlFlow.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestControlFlow;java/lang/Object"]

    pub struct TestControlFlow;

    impl TestControlFlow {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut x: i32 = 10i32;
            if x > 5i32 {
                System::out().println_v(Clone::clone(&String::from("greater")))?;
            } else {
                System::out().println_v(Clone::clone(&String::from("smaller")))?;
            }
            let mut i: i32 = 0i32;
            loop {
                if i >= 3i32 { break; }
                System::out().println_v(i)?;
                i = i.wrapping_add(1i32);
            }
            let mut j: i32 = 0i32;
            loop {
                if j >= 10i32 { break; }
                if j == 5i32 {
                    break;
                }
                System::out().println_v(j)?;
                j = j.wrapping_add(1i32);
            }
            j = 0i32;
            loop {
                if j >= 5i32 { break; }
                if j == 2i32 {
                } else {
                    System::out().println_v(j)?;
                }
                j = j.wrapping_add(1i32);
            }
            j = 3i32;
            match j {
                1 => {
                    System::out().println_v(Clone::clone(&String::from("Monday")))?;
                }
                2 => {
                    System::out().println_v(Clone::clone(&String::from("Tuesday")))?;
                }
                3 => {
                    System::out().println_v(Clone::clone(&String::from("Wednesday")))?;
                }
                _ => {
                    System::out().println_v(Clone::clone(&String::from("Other")))?;
                }
            }
            let mut result = (if x > 5i32 { String::from("yes") } else { String::from("no") });
            System::out().println_v(Clone::clone(&result))?;
            let mut n: i32 = 0i32;
            loop {
                System::out().println_v(Clone::clone(&String::from_owned(format!("do {}", n))))?;
                n = n.wrapping_add(1i32);
                if n >= 2i32 { break; }
            }
            let mut a: i32 = 0i32;
            loop {
                if a >= 2i32 { break; }
                let mut b: i32 = 0i32;
                loop {
                    if b >= 2i32 { break; }
                    System::out().println_v(Clone::clone(&String::from_owned(format!("{},{}", a, b))))?;
                    b = b.wrapping_add(1i32);
                }
                a = a.wrapping_add(1i32);
            }
            Ok(())
        }
    }
}
