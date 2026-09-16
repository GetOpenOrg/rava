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
use crate::test_inheritance_animal::TestInheritance_Animal;
use crate::test_inheritance_dog::TestInheritance_Dog;
use crate::test_inheritance_cat::TestInheritance_Cat;
use crate::test_inheritance_guide_dog::TestInheritance_GuideDog;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestInheritance"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestInheritance.java"]
    #[inner_classes     = "TestInheritance$Animal:TestInheritance:Animal:8;TestInheritance$Dog:TestInheritance:Dog:8;TestInheritance$Cat:TestInheritance:Cat:8;TestInheritance$GuideDog:TestInheritance:GuideDog:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestInheritance;java/lang/Object"]

    pub struct TestInheritance;

    impl TestInheritance {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut a = TestInheritance_Animal::new(Clone::clone(&String::from("Animal")))?;
            let mut d = TestInheritance_Dog::new(Clone::clone(&String::from("Rex")))?;
            let mut c = TestInheritance_Cat::new(Clone::clone(&String::from("Whiskers")))?;
            let mut g = TestInheritance_GuideDog::new(Clone::clone(&String::from("Buddy")))?;
            let _t0 = a.speak()?;
            System::out().println_v(Clone::clone(&_t0))?;
            let _t1 = d.speak()?;
            System::out().println_v(Clone::clone(&_t1))?;
            let _t2 = c.speak()?;
            System::out().println_v(Clone::clone(&_t2))?;
            let _t3 = g.speak()?;
            System::out().println_v(Clone::clone(&_t3))?;
            let _t4 = d.__super().getName()?;
            System::out().println_v(Clone::clone(&_t4))?;
            let mut _arr5: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 4i32 as usize]));
            _arr5.borrow_mut()[0i32 as usize] = Object::from_any(a.clone());
            _arr5.borrow_mut()[1i32 as usize] = Object::from_any(d.clone());
            _arr5.borrow_mut()[2i32 as usize] = Object::from_any(c.clone());
            _arr5.borrow_mut()[3i32 as usize] = Object::from_any(g.clone());
            let mut animals: Rc<RefCell<Vec<Object>>> = _arr5;
            let mut local_6: Rc<RefCell<Vec<Object>>> = animals;
            let mut local_7 = (local_6.borrow().len() as i32);
            let mut local_8: i32 = 0i32;
            loop {
                if local_8 >= local_7 { break; }
                let mut animal = Clone::clone(&local_6.borrow()[local_8 as usize]);
                let _vdispatch6: String = if let Some(_d) = animal.0.as_any().downcast_ref::<TestInheritance_GuideDog>() { _d.speak()? } else if let Some(_d) = animal.0.as_any().downcast_ref::<TestInheritance_Dog>() { _d.speak()? } else if let Some(_d) = animal.0.as_any().downcast_ref::<TestInheritance_Cat>() { _d.speak()? } else if let Some(_d) = animal.0.as_any().downcast_ref::<TestInheritance_Animal>() { _d.speak()? } else if let Some(__f) = animal.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                System::out().println_v(Clone::clone(&_vdispatch6))?;
                local_8 = local_8.wrapping_add(1i32);
            }
            System::out().println_v(true)?;
            System::out().println_v(true)?;
            System::out().println_v(false)?;
            Ok(())
        }
    }
}
