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
    #[binary_name       = "TestArrays"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestArrays.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestArrays;java/lang/Object"]

    pub struct TestArrays;

    impl TestArrays {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "sumArray", descriptor = "([I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sumArray(mut arr: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            let mut sum: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if i >= (arr.borrow().len() as i32) { break; }
                sum = (sum).wrapping_add(arr.borrow()[i as usize]);
                i = i.wrapping_add(1i32);
            }
            Ok(sum)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 5i32 as usize]));
            let mut arr: Rc<RefCell<Vec<i32>>> = _arr0;
            arr.borrow_mut()[0i32 as usize] = 10i32;
            arr.borrow_mut()[1i32 as usize] = 20i32;
            arr.borrow_mut()[2i32 as usize] = 30i32;
            arr.borrow_mut()[3i32 as usize] = 40i32;
            arr.borrow_mut()[4i32 as usize] = 50i32;
            System::out().println_v((arr.borrow().len() as i32))?;
            let _t1: i32 = Self::sumArray(Clone::clone(&arr))?;
            System::out().println_v(_t1)?;
            System::out().println_v(arr.borrow()[2i32 as usize])?;
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 5i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = 1i32;
            _arr2.borrow_mut()[1i32 as usize] = 2i32;
            _arr2.borrow_mut()[2i32 as usize] = 3i32;
            _arr2.borrow_mut()[3i32 as usize] = 4i32;
            _arr2.borrow_mut()[4i32 as usize] = 5i32;
            let mut arr2: Rc<RefCell<Vec<i32>>> = _arr2;
            let _t3: i32 = Self::sumArray(Clone::clone(&arr2))?;
            System::out().println_v(_t3)?;
            Ok(())
        }
    }
}
