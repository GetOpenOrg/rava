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
use crate::test_equals_hash_code_point::TestEqualsHashCode_Point;
use crate::test_equals_hash_code_person::TestEqualsHashCode_Person;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestEqualsHashCode"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestEqualsHashCode.java"]
    #[inner_classes     = "TestEqualsHashCode$Point:TestEqualsHashCode:Point:8;TestEqualsHashCode$Person:TestEqualsHashCode:Person:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestEqualsHashCode;java/lang/Object"]

    pub struct TestEqualsHashCode;

    impl TestEqualsHashCode {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut p1 = TestEqualsHashCode_Point::new(1i32, 2i32)?;
            let mut p2 = TestEqualsHashCode_Point::new(1i32, 2i32)?;
            let mut p3 = TestEqualsHashCode_Point::new(3i32, 4i32)?;
            let _t0 = p1.equals(Object::from_any(p2.clone()))?;
            System::out().println_v(_t0)?;
            let _t1 = p1.equals(Object::from_any(p3.clone()))?;
            System::out().println_v(_t1)?;
            let _t2 = p1.equals(Clone::clone(&Object::default()))?;
            System::out().println_v(_t2)?;
            let _t3 = p1.equals(Object::from_any(String::from("str").clone()))?;
            System::out().println_v(_t3)?;
            System::out().println_v(Object::from_any(p1.clone()) == Object::from_any(p2.clone()))?;
            let _t4 = p1.hashCode()?;
            let _t5 = p2.hashCode()?;
            System::out().println_v(_t4 == _t5)?;
            let _t6 = p1.toString()?;
            System::out().println_v(Clone::clone(&_t6))?;
            let _t7 = p3.toString()?;
            System::out().println_v(Clone::clone(&_t7))?;
            let mut alice1 = TestEqualsHashCode_Person::new(Clone::clone(&String::from("Alice")), 30i32)?;
            let mut alice2 = TestEqualsHashCode_Person::new(Clone::clone(&String::from("Alice")), 30i32)?;
            let mut bob = TestEqualsHashCode_Person::new(Clone::clone(&String::from("Bob")), 25i32)?;
            let _t8 = alice1.equals(Object::from_any(alice2.clone()))?;
            System::out().println_v(_t8)?;
            let _t9 = alice1.equals(Object::from_any(bob.clone()))?;
            System::out().println_v(_t9)?;
            let _t10 = alice1.hashCode()?;
            let _t11 = alice2.hashCode()?;
            System::out().println_v(_t10 == _t11)?;
            let _t12 = p1.equals(Object::from_any(p1.clone()))?;
            System::out().println_v(_t12)?;
            let _t13 = alice1.equals(Object::from_any(alice1.clone()))?;
            System::out().println_v(_t13)?;
            let _t14 = p1.equals(Object::from_any(p2.clone()))?;
            let _t15 = p2.equals(Object::from_any(p1.clone()))?;
            System::out().println_v(_t14 == _t15)?;
            Ok(())
        }
    }
}
