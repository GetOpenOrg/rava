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
    #[binary_name       = "TestArrayList"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestArrayList.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestArrayList;java/lang/Object"]

    pub struct TestArrayList;

    impl TestArrayList {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut names = ArrayList::<Object>::new()?;
            let _t0 = names.add_obj(Object::from_any(String::from("Alice").clone()))?;
            let _t1 = names.add_obj(Object::from_any(String::from("Bob").clone()))?;
            let _t2 = names.add_obj(Object::from_any(String::from("Charlie").clone()))?;
            let _t3 = names.size()?;
            System::out().println_v(_t3)?;
            let _t4 = names.get(0i32)?;
            System::out().println_v(Clone::clone(&(_t4).downcast::<String>()))?;
            let _t5 = names.get(1i32)?;
            System::out().println_v(Clone::clone(&(_t5).downcast::<String>()))?;
            let _t6 = names.get(2i32)?;
            System::out().println_v(Clone::clone(&(_t6).downcast::<String>()))?;
            let mut scores = ArrayList::<Object>::new()?;
            let _t7 = scores.add_obj(100i32.into())?;
            let _t8 = scores.add_obj(95i32.into())?;
            let _t9 = scores.add_obj(87i32.into())?;
            let _t10 = scores.size()?;
            System::out().println_v(_t10)?;
            let _t11 = scores.get(0i32)?;
            let mut first = (_t11).downcast::<i32>();
            System::out().println_v(first)?;
            let _t12 = scores.get(1i32)?;
            System::out().println_v(Clone::clone(&_t12))?;
            let _t13 = names.iterator()?;
            let mut ages = (_t13).downcast::<HashMap<String, i32>>();
            loop {
                let _t14 = ages.hasNext()?;
                if !(_t14) { break; }
                let _t14 = ages.next()?;
                let mut name = (_t14).downcast::<String>();
                System::out().println_v(Clone::clone(&name))?;
            }
            let mut ages = HashMap::<Object, Object>::new()?;
            let _t14 = ages.put(Object::from_any(String::from("Alice").clone()), 30i32.into())?;
            let _t15 = ages.put(Object::from_any(String::from("Bob").clone()), 25i32.into())?;
            let _t16 = ages.put(Object::from_any(String::from("Charlie").clone()), 35i32.into())?;
            let _t17 = ages.size()?;
            System::out().println_v(_t17)?;
            let _t18 = ages.get(Object::from_any(String::from("Alice").clone()))?;
            System::out().println_v(Clone::clone(&_t18))?;
            let _t19 = ages.get(Object::from_any(String::from("Bob").clone()))?;
            System::out().println_v(Clone::clone(&_t19))?;
            let _t20 = ages.containsKey(Object::from_any(String::from("Charlie").clone()))?;
            System::out().println_v(_t20)?;
            let _t21 = ages.containsKey(Object::from_any(String::from("Dave").clone()))?;
            System::out().println_v(_t21)?;
            let mut name = HashSet::<Object>::new()?;
            let _t22 = name.add(Object::from_any(String::from("apple").clone()))?;
            let _t23 = name.add(Object::from_any(String::from("banana").clone()))?;
            let _t24 = name.add(Object::from_any(String::from("apple").clone()))?;
            let _t25 = name.size()?;
            System::out().println_v(_t25)?;
            let _t26 = name.contains(Object::from_any(String::from("apple").clone()))?;
            System::out().println_v(_t26)?;
            let _t27 = name.contains(Object::from_any(String::from("grape").clone()))?;
            System::out().println_v(_t27)?;
            Ok(())
        }
    }
}
