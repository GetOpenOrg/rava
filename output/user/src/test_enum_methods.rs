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
use crate::test_enum_methods_planet::TestEnumMethods_Planet;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestEnumMethods"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestEnumMethods.java"]
    #[inner_classes     = "TestEnumMethods$Planet:TestEnumMethods:Planet:16408"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestEnumMethods;java/lang/Object"]

    pub struct TestEnumMethods;

    impl TestEnumMethods {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut earthWeight: f64 = 75.0f64;
            let _t0 = TestEnumMethods_Planet::EARTH().surfaceGravity()?;
            let mut mass = (earthWeight/_t0);
            let _t1: Rc<RefCell<Vec<TestEnumMethods_Planet>>> = TestEnumMethods_Planet::values()?;
            let mut local_5: Rc<RefCell<Vec<TestEnumMethods_Planet>>> = _t1;
            let mut local_6 = (local_5.borrow().len() as i32);
            let mut local_7: i32 = 0i32;
            loop {
                if local_7 >= local_6 { break; }
                let mut p = Clone::clone(&local_5.borrow()[local_7 as usize]);
                let mut _arr2: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
                _arr2.borrow_mut()[0i32 as usize] = Object::from_any(p.clone());
                let _t3 = p.surfaceWeight(mass)?;
                _arr2.borrow_mut()[1i32 as usize] = _t3.into();
                let _t4 = System::out().printf_str_arr_obj(Clone::clone(&String::from("Weight on %s is %6.2f%n")), Clone::clone(&_arr2))?;
                local_7 = local_7.wrapping_add(1i32);
            }
            Ok(())
        }
    }
}
