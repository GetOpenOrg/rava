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
    #[binary_name       = "TestMultiArray"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestMultiArray.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestMultiArray;java/lang/Object"]

    pub struct TestMultiArray;

    impl TestMultiArray {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 3i32 as usize]));
            _arr1.borrow_mut()[0i32 as usize] = 1i32;
            _arr1.borrow_mut()[1i32 as usize] = 2i32;
            _arr1.borrow_mut()[2i32 as usize] = 3i32;
            _arr0.borrow_mut()[0i32 as usize] = Object::from_any(_arr1.clone());
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 3i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = 4i32;
            _arr2.borrow_mut()[1i32 as usize] = 5i32;
            _arr2.borrow_mut()[2i32 as usize] = 6i32;
            _arr0.borrow_mut()[1i32 as usize] = Object::from_any(_arr2.clone());
            let mut _arr3: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 3i32 as usize]));
            _arr3.borrow_mut()[0i32 as usize] = 7i32;
            _arr3.borrow_mut()[1i32 as usize] = 8i32;
            _arr3.borrow_mut()[2i32 as usize] = 9i32;
            _arr0.borrow_mut()[2i32 as usize] = Object::from_any(_arr3.clone());
            let mut matrix: Rc<RefCell<Vec<Object>>> = _arr0;
            let mut transposed: Rc<RefCell<Vec<Object>>> = matrix;
            let mut i = (transposed.borrow().len() as i32);
            let mut j: i32 = 0i32;
            let mut local_8: i32 = Default::default();
        let mut v = Default::default();
            loop {
                if j >= i { break; }
                let mut row = Clone::clone(&transposed.borrow()[j as usize]);
                let mut row: Object = row;
                let mut row = (row.borrow().len() as i32);
                local_8 = 0i32;
                v = Default::default();
                loop {
                    if local_8 >= row { break; }
                    v = row.borrow()[local_8 as usize];
                    System::out().print_str(Clone::clone(&String::from_owned(format!("{} ", v))))?;
                    local_8 = local_8.wrapping_add(1i32);
                }
                System::out().println()?;
                j = j.wrapping_add(1i32);
            }
            let mut _arr4: Vec<Vec<i32>> = vec![vec![0i32; 3i32 as usize]; 3i32 as usize];
            let mut transposed = _arr4;
            i = 0i32;
            loop {
                if i >= 3i32 { break; }
                j = 0i32;
                loop {
                    if j >= 3i32 { break; }
                    Clone::clone(&transposed.borrow()[j as usize]).borrow_mut()[i as usize] = Clone::clone(&matrix.borrow()[i as usize]).borrow()[j as usize];
                    j = j.wrapping_add(1i32);
                }
                i = i.wrapping_add(1i32);
            }
            System::out().println_v(Clone::clone(&String::from("---")))?;
            let mut i = transposed;
            j = (i.borrow().len() as i32);
            let mut row: i32 = 0i32;
            loop {
                if row >= j { break; }
                let mut row = Clone::clone(&i.borrow()[row as usize]);
                let mut row: Vec<i32> = row;
                local_8 = (row.borrow().len() as i32);
                v = 0i32;
                loop {
                    if v >= local_8 { break; }
                    v = row.borrow()[v as usize];
                    System::out().print_str(Clone::clone(&String::from_owned(format!("{} ", v))))?;
                    v = v.wrapping_add(1i32);
                }
                System::out().println()?;
                row = row.wrapping_add(1i32);
            }
            let mut _arr5: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            let mut i: Rc<RefCell<Vec<Object>>> = _arr5;
            j = 0i32;
            loop {
                if j >= 3i32 { break; }
                let mut _arr6: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (j).wrapping_add(1i32) as usize]));
                i.borrow_mut()[j as usize] = Object::from_any(_arr6.clone());
                let mut row = 0i32;
                loop {
                    if row > j { break; }
                    Clone::clone(&i.borrow()[j as usize]).borrow_mut()[row as usize] = (j).wrapping_add(row);
                    row = row.wrapping_add(1i32);
                }
                j = j.wrapping_add(1i32);
            }
            let mut j: Rc<RefCell<Vec<Object>>> = i;
            let mut row = (j.borrow().len() as i32);
            let mut row: i32 = 0i32;
            loop {
                if row >= row { break; }
                let mut row = Clone::clone(&j.borrow()[row as usize]);
                System::out().println_v((row.borrow().len() as i32))?;
                row = row.wrapping_add(1i32);
            }
            Ok(())
        }
    }
}
