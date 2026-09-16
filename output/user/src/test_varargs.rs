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
    #[binary_name       = "TestVarargs"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestVarargs.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestVarargs;java/lang/Object"]

    pub struct TestVarargs;

    impl TestVarargs {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "sum", descriptor = "([I)I", access = "package", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sum(mut nums: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            let mut total: i32 = 0i32;
            let mut local_2: Rc<RefCell<Vec<i32>>> = nums;
            let mut local_3 = (local_2.borrow().len() as i32);
            let mut local_4: i32 = 0i32;
            loop {
                if local_4 >= local_3 { break; }
                let mut n = local_2.borrow()[local_4 as usize];
                total = (total).wrapping_add(n);
                local_4 = local_4.wrapping_add(1i32);
            }
            Ok(total)
        }

        #[java_method(name = "join", descriptor = "(Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/String;", access = "package", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn join(mut sep: String, mut parts: Rc<RefCell<Vec<String>>>) -> Result<String> {
            let mut sb = StringBuilder::new()?;
            let mut i: i32 = 0i32;
            loop {
                if i >= (parts.borrow().len() as i32) { break; }
                if (i>0) {
                    let _t0 = sb.append_str(Clone::clone(&sep))?;
                }
                let _t0 = sb.append_str(Clone::clone(&Clone::clone(&parts.borrow()[i as usize])))?;
                i = i.wrapping_add(1i32);
            }
            let _t0 = sb.toString()?;
            Ok(_t0)
        }

        #[java_method(name = "max", descriptor = "(D[D)D", access = "package", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max(mut first: f64, mut rest: Rc<RefCell<Vec<f64>>>) -> Result<f64> {
            let mut m: f64 = first;
            let mut local_5: Rc<RefCell<Vec<f64>>> = rest;
            let mut local_6 = (local_5.borrow().len() as i32);
            let mut local_7: i32 = 0i32;
            loop {
                if local_7 >= local_6 { break; }
                let mut v = local_5.borrow()[local_7 as usize];
                if (((v>(m)) as i32-((v)<(m)) as i32)>0) {
                    m = v;
                }
                local_7 = local_7.wrapping_add(1i32);
            }
            Ok(m)
        }

        #[java_method(name = "count", descriptor = "([Ljava/lang/Object;)I", access = "package", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn count(mut items: Rc<RefCell<Vec<Object>>>) -> Result<i32> {
            Ok((items.borrow().len() as i32))
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 0i32 as usize]));
            let _t1: i32 = Self::sum(Clone::clone(&_arr0))?;
            System::out().println_v(_t1)?;
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 1i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = 1i32;
            let _t3: i32 = Self::sum(Clone::clone(&_arr2))?;
            System::out().println_v(_t3)?;
            let mut _arr4: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 3i32 as usize]));
            _arr4.borrow_mut()[0i32 as usize] = 1i32;
            _arr4.borrow_mut()[1i32 as usize] = 2i32;
            _arr4.borrow_mut()[2i32 as usize] = 3i32;
            let _t5: i32 = Self::sum(Clone::clone(&_arr4))?;
            System::out().println_v(_t5)?;
            let mut _arr6: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 4i32 as usize]));
            _arr6.borrow_mut()[0i32 as usize] = 10i32;
            _arr6.borrow_mut()[1i32 as usize] = 20i32;
            _arr6.borrow_mut()[2i32 as usize] = 30i32;
            _arr6.borrow_mut()[3i32 as usize] = 40i32;
            let _t7: i32 = Self::sum(Clone::clone(&_arr6))?;
            System::out().println_v(_t7)?;
            let mut _arr8: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            _arr8.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("a"));
            _arr8.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("b"));
            _arr8.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("c"));
            let _t9: String = Self::join(Clone::clone(&String::from(", ")), Clone::clone(&_arr8))?;
            System::out().println_v(Clone::clone(&_t9))?;
            let mut _arr10: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            _arr10.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("x"));
            let _t11: String = Self::join(Clone::clone(&String::from("-")), Clone::clone(&_arr10))?;
            System::out().println_v(Clone::clone(&_t11))?;
            let mut _arr12: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 0i32 as usize]));
            let _t13: String = Self::join(Clone::clone(&String::from("|")), Clone::clone(&_arr12))?;
            System::out().println_v(Clone::clone(&_t13))?;
            let mut _arr14: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(vec![0f64; 0i32 as usize]));
            let _t15: f64 = Self::max(3.0f64, Clone::clone(&_arr14))?;
            System::out().println_v(_t15)?;
            let mut _arr16: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(vec![0f64; 2i32 as usize]));
            _arr16.borrow_mut()[0i32 as usize] = 5.0f64;
            _arr16.borrow_mut()[1i32 as usize] = 2.0f64;
            let _t17: f64 = Self::max(1f64, Clone::clone(&_arr16))?;
            System::out().println_v(_t17)?;
            let mut _arr18: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            _arr18.borrow_mut()[0i32 as usize] = Object::from_any(String::from("hello").clone());
            _arr18.borrow_mut()[1i32 as usize] = 42i32.into();
            _arr18.borrow_mut()[2i32 as usize] = 1i32.into();
            let _t19: i32 = Self::count(Clone::clone(&_arr18))?;
            System::out().println_v(_t19)?;
            let mut _arr20: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 3i32 as usize]));
            _arr20.borrow_mut()[0i32 as usize] = 4i32;
            _arr20.borrow_mut()[1i32 as usize] = 5i32;
            _arr20.borrow_mut()[2i32 as usize] = 6i32;
            let mut arr: Rc<RefCell<Vec<i32>>> = _arr20;
            let _t21: i32 = Self::sum(Clone::clone(&arr))?;
            System::out().println_v(_t21)?;
            Ok(())
        }
    }
}
