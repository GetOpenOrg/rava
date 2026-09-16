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
    #[binary_name       = "TestArraysUtil"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestArraysUtil.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestArraysUtil;java/lang/Object"]

    pub struct TestArraysUtil;

    impl TestArraysUtil {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 5i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = 5i32;
            _arr0.borrow_mut()[1i32 as usize] = 3i32;
            _arr0.borrow_mut()[2i32 as usize] = 1i32;
            _arr0.borrow_mut()[3i32 as usize] = 4i32;
            _arr0.borrow_mut()[4i32 as usize] = 2i32;
            let mut arr: Rc<RefCell<Vec<i32>>> = _arr0;
            Arrays::sort_arr_i(Clone::clone(&arr))?;
            let _t1: String = Arrays::toString_arr_i(Clone::clone(&arr))?;
            System::out().println_v(Clone::clone(&_t1))?;
            let _t2: i32 = Arrays::binarySearch_arr_i_i(Clone::clone(&arr), 3i32)?;
            let mut idx: i32 = _t2;
            System::out().println_v(idx)?;
            let _t3: Rc<RefCell<Vec<i32>>> = Arrays::copyOf_arr_i_i(Clone::clone(&arr), 3i32)?;
            let mut copy: Rc<RefCell<Vec<i32>>> = _t3;
            let _t4: String = Arrays::toString_arr_i(Clone::clone(&copy))?;
            System::out().println_v(Clone::clone(&_t4))?;
            let _t5: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&arr), 1i32, 4i32)?;
            let mut range: Rc<RefCell<Vec<i32>>> = _t5;
            let _t6: String = Arrays::toString_arr_i(Clone::clone(&range))?;
            System::out().println_v(Clone::clone(&_t6))?;
            let mut _arr7: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 5i32 as usize]));
            let mut filled: Rc<RefCell<Vec<i32>>> = _arr7;
            Arrays::fill_arr_i_i(Clone::clone(&filled), 7i32)?;
            let _t8: String = Arrays::toString_arr_i(Clone::clone(&filled))?;
            System::out().println_v(Clone::clone(&_t8))?;
            let mut _arr9: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 5i32 as usize]));
            _arr9.borrow_mut()[0i32 as usize] = 1i32;
            _arr9.borrow_mut()[1i32 as usize] = 2i32;
            _arr9.borrow_mut()[2i32 as usize] = 3i32;
            _arr9.borrow_mut()[3i32 as usize] = 4i32;
            _arr9.borrow_mut()[4i32 as usize] = 5i32;
            let _t10: bool = Arrays::equals_arr_i_arr_i(Clone::clone(&arr), Clone::clone(&_arr9))?;
            System::out().println_v(_t10)?;
            let mut _arr11: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            _arr11.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("banana"));
            _arr11.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("apple"));
            _arr11.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("cherry"));
            let mut words: Rc<RefCell<Vec<String>>> = _arr11;
            Arrays::sort_arr_obj(Default::default())?;
            let _t12: String = Arrays::toString_arr_obj(Default::default())?;
            System::out().println_v(Clone::clone(&_t12))?;
            Ok(())
        }
    }
}
