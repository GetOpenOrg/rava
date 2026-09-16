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
    #[binary_name       = "TestMultiCatch"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestMultiCatch.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestMultiCatch;java/lang/Object"]

    pub struct TestMultiCatch;

    impl TestMultiCatch {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "divide", descriptor = "(II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide(mut a: i32, mut b: i32) -> Result<i32> {
            Ok((a/b))
        }

        #[java_method(name = "parseAndDivide", descriptor = "(Ljava/lang/String;Ljava/lang/String;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseAndDivide(mut a: String, mut b: String) -> Result<i32> {
            let _t0: i32 = Integer::parseInt_str(Clone::clone(&a))?;
            let _t1: i32 = Integer::parseInt_str(Clone::clone(&b))?;
            Ok((_t0/_t1))
        }

        #[java_method(name = "accessArray", descriptor = "([II)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn accessArray(mut arr: Rc<RefCell<Vec<i32>>>, mut idx: i32) -> Result<String> {
            Ok(String::from_owned(format!("{}", arr.borrow()[idx as usize])))
        }

        #[java_method(name = "tryCatch", descriptor = "(II)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryCatch(mut a: i32, mut b: i32) -> Result<()> {
            let _t0: i32 = Self::divide(a, b)?;
            let mut result: i32 = _t0;
            System::out().println_v(Clone::clone(&String::from_owned(format!("result: {}", result))))?;
            Ok(())
        }

        #[java_method(name = "multiCatch", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiCatch(mut a: String, mut b: String) -> Result<()> {
            let _t0: i32 = Self::parseAndDivide(Clone::clone(&a), Clone::clone(&b))?;
            let mut result: i32 = _t0;
            System::out().println_v(Clone::clone(&String::from_owned(format!("result: {}", result))))?;
            Ok(())
        }

        #[java_method(name = "nestedTry", descriptor = "([Ljava/lang/String;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nestedTry(mut data: Rc<RefCell<Vec<String>>>) -> Result<()> {
            let mut i: i32 = 0i32;
            loop {
                if i >= (data.borrow().len() as i32) { break; }
                let _t0: i32 = Integer::parseInt_str(Clone::clone(&Clone::clone(&data.borrow()[i as usize])))?;
                let mut val: i32 = _t0;
                System::out().println_v(Clone::clone(&String::from_owned(format!("parsed: {}", val))))?;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            Self::tryCatch(10i32, 2i32)?;
            Self::tryCatch(10i32, 0i32)?;
            Self::multiCatch(Clone::clone(&String::from("10")), Clone::clone(&String::from("2")))?;
            Self::multiCatch(Clone::clone(&String::from("10")), Clone::clone(&String::from("0")))?;
            Self::multiCatch(Clone::clone(&String::from("abc")), Clone::clone(&String::from("2")))?;
            let mut _arr0: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 5i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("1"));
            _arr0.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("abc"));
            _arr0.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("3"));
            _arr0.borrow_mut()[3i32 as usize] = Clone::clone(&String::from("bad"));
            _arr0.borrow_mut()[4i32 as usize] = Clone::clone(&String::from("5"));
            Self::nestedTry(Clone::clone(&_arr0))?;
            let mut i: i32 = 0i32;
            loop {
                if i >= 3i32 { break; }
                if i == 1i32 {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                System::out().println_v(Clone::clone(&String::from_owned(format!("try: {}", i))))?;
                System::out().println_v(Clone::clone(&String::from_owned(format!("finally: {}", i))))?;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }
    }
}
