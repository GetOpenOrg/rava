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
use crate::test_casting_dog::TestCasting_Dog;
use crate::test_casting_cat::TestCasting_Cat;
use crate::test_casting_animal::TestCasting_Animal;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestCasting"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestCasting.java"]
    #[inner_classes     = "TestCasting$Dog:TestCasting:Dog:8;TestCasting$Cat:TestCasting:Cat:8;TestCasting$Animal:TestCasting:Animal:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestCasting;java/lang/Object"]

    pub struct TestCasting;

    impl TestCasting {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut a1 = TestCasting_Dog::new(Clone::clone(&String::from("Rex")))?;
            let mut a2 = TestCasting_Cat::new(Clone::clone(&String::from("Whiskers")))?;
            let _t0 = a1.speak()?;
            System::out().println_v(Clone::clone(&_t0))?;
            let _t1 = a2.speak()?;
            System::out().println_v(Clone::clone(&_t1))?;
            if true {
                let mut d: TestCasting_Dog = a1;
                d.fetch()?;
            }
            let mut _arr2: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = Object::from_any(TestCasting_Dog::new(Clone::clone(&String::from("Buddy")))?.clone());
            _arr2.borrow_mut()[1i32 as usize] = Object::from_any(TestCasting_Cat::new(Clone::clone(&String::from("Luna")))?.clone());
            _arr2.borrow_mut()[2i32 as usize] = Object::from_any(TestCasting_Dog::new(Clone::clone(&String::from("Max")))?.clone());
            let mut d: Rc<RefCell<Vec<Object>>> = _arr2;
            let mut pi: Rc<RefCell<Vec<Object>>> = d;
            let mut local_5 = (pi.borrow().len() as i32);
            let mut truncated: i32 = 0i32;
            loop {
                if truncated >= local_5 { break; }
                let mut a = Clone::clone(&pi.borrow()[truncated as usize]);
                let _vdispatch3: String = if let Some(_d) = a.0.as_any().downcast_ref::<TestCasting_Dog>() { _d.speak()? } else if let Some(_d) = a.0.as_any().downcast_ref::<TestCasting_Cat>() { _d.speak()? } else if let Some(_d) = a.0.as_any().downcast_ref::<TestCasting_Animal>() { _d.speak()? } else if let Some(__f) = a.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                System::out().println_v(Clone::clone(&_vdispatch3))?;
                if (a.is_instance_of("TestCasting$Dog")) {
                    (a).downcast::<TestCasting_Dog>().fetch()?;
                } else {
                    if (a.is_instance_of("TestCasting$Cat")) {
                        (a).downcast::<TestCasting_Cat>().purr()?;
                    }
                }
                truncated = truncated.wrapping_add(1i32);
            }
            let mut pi: f64 = 3.14159f64;
            truncated = (pi as i32);
            System::out().println_v(truncated)?;
            let mut a: i64 = 100000i64;
            let mut small: i32 = (a as i32);
            System::out().println_v(small)?;
            let mut val: i32 = 65i32;
            let mut ch = ((val) as u16 as i32);
            System::out().println_v(((ch) as u16))?;
            let mut x: i32 = 42i32;
            let mut d: f64 = (x as f64);
            System::out().println_v(d)?;
            System::out().println_v(false)?;
            System::out().println_v(false)?;
            Ok(())
        }
    }
}
