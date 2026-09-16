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
    #[binary_name       = "TestStringFormat"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestStringFormat.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestStringFormat;java/lang/Object"]

    pub struct TestStringFormat;

    impl TestStringFormat {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut name: String = String::from("Alice");
            let mut age: i32 = 30i32;
            let mut score: f64 = 98.5f64;
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = Object::from_any(name.clone());
            _arr0.borrow_mut()[1i32 as usize] = age.into();
            let _t1 = System::out().printf_str_arr_obj(Clone::clone(&String::from("Name: %s, Age: %d%n")), Clone::clone(&_arr0))?;
            let mut _arr2: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = score.into();
            let _t3 = System::out().printf_str_arr_obj(Clone::clone(&String::from("Score: %.1f%n")), Clone::clone(&_arr2))?;
            let mut _arr4: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            _arr4.borrow_mut()[0i32 as usize] = 255i32.into();
            let _t5 = System::out().printf_str_arr_obj(Clone::clone(&String::from("Hex: %x%n")), Clone::clone(&_arr4))?;
            let mut _arr6: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            _arr6.borrow_mut()[0i32 as usize] = Object::from_any(String::from("hi").clone());
            let _t7 = System::out().printf_str_arr_obj(Clone::clone(&String::from("Padded: %10s|%n")), Clone::clone(&_arr6))?;
            let mut _arr8: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            _arr8.borrow_mut()[0i32 as usize] = Object::from_any(String::from("hi").clone());
            let _t9 = System::out().printf_str_arr_obj(Clone::clone(&String::from("Left: %-10s|%n")), Clone::clone(&_arr8))?;
            let mut _arr10: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
            _arr10.borrow_mut()[0i32 as usize] = Object::from_any(name.clone());
            _arr10.borrow_mut()[1i32 as usize] = age.into();
            let _t11: String = String::format_str_arr_obj(Clone::clone(&String::from("(%s, %d)")), Clone::clone(&_arr10))?;
            let mut formatted: String = _t11;
            System::out().println_v(Clone::clone(&formatted))?;
            Ok(())
        }
    }
}
