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
    #[binary_name       = "TestSorting"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestSorting.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestSorting;java/lang/Object"]

    pub struct TestSorting;

    impl TestSorting {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "bubbleSort", descriptor = "([I)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bubbleSort(mut arr: Rc<RefCell<Vec<i32>>>) -> Result<()> {
            let mut n = (arr.borrow().len() as i32);
            let mut i: i32 = 0i32;
            loop {
                if i >= (n).wrapping_sub(1i32) { break; }
                let mut j: i32 = 0i32;
                loop {
                    if j >= ((n).wrapping_sub(i)).wrapping_sub(1i32) { break; }
                    if arr.borrow()[j as usize] > arr.borrow()[(j).wrapping_add(1i32) as usize] {
                        let mut tmp = arr.borrow()[j as usize];
                        let _tmp_val0 = arr.borrow()[(j).wrapping_add(1i32) as usize];
                        arr.borrow_mut()[j as usize] = _tmp_val0;
                        arr.borrow_mut()[(j).wrapping_add(1i32) as usize] = tmp;
                    }
                    j = j.wrapping_add(1i32);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "selectionSort", descriptor = "([I)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn selectionSort(mut arr: Rc<RefCell<Vec<i32>>>) -> Result<()> {
            let mut n = (arr.borrow().len() as i32);
            let mut i: i32 = 0i32;
            loop {
                if i >= (n).wrapping_sub(1i32) { break; }
                let mut minIdx: i32 = i;
                let mut j = (i).wrapping_add(1i32);
                loop {
                    if j >= n { break; }
                    if arr.borrow()[j as usize] < arr.borrow()[minIdx as usize] {
                        minIdx = j;
                    }
                    j = j.wrapping_add(1i32);
                }
                j = arr.borrow()[minIdx as usize];
                let _tmp_val0 = arr.borrow()[i as usize];
                arr.borrow_mut()[minIdx as usize] = _tmp_val0;
                arr.borrow_mut()[i as usize] = j;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "insertionSort", descriptor = "([I)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insertionSort(mut arr: Rc<RefCell<Vec<i32>>>) -> Result<()> {
            let mut n = (arr.borrow().len() as i32);
            let mut i: i32 = 1i32;
            loop {
                if i >= n { break; }
                let mut key = arr.borrow()[i as usize];
                let mut j = (i).wrapping_sub(1i32);
                loop {
                    if (j<0) { break; }
                    if arr.borrow()[j as usize] > key {
                        let _tmp_val0 = arr.borrow()[j as usize];
                        arr.borrow_mut()[(j).wrapping_add(1i32) as usize] = _tmp_val0;
                        j = j.wrapping_sub(1i32);
                        continue;
                    }
                    break;
                }
                arr.borrow_mut()[(j).wrapping_add(1i32) as usize] = key;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "binarySearch", descriptor = "([II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch(mut arr: Rc<RefCell<Vec<i32>>>, mut target: i32) -> Result<i32> {
            let mut lo: i32 = 0i32;
            let mut hi = ((arr.borrow().len() as i32)).wrapping_sub(1i32);
            loop {
                if lo > hi { break; }
                let mut mid = (lo).wrapping_add(((hi).wrapping_sub(lo)/2i32));
                if arr.borrow()[mid as usize] == target {
                    return Ok(mid);
                }
                if arr.borrow()[mid as usize] < target {
                    lo = (mid).wrapping_add(1i32);
                } else {
                    hi = (mid).wrapping_sub(1i32);
                }
            }
            Ok(-1i32)
        }

        #[java_method(name = "printArray", descriptor = "([I)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn printArray(mut arr: Rc<RefCell<Vec<i32>>>) -> Result<()> {
            let mut i: i32 = 0i32;
            loop {
                if i >= (arr.borrow().len() as i32) { break; }
                if (i>0) {
                    System::out().print_str(Clone::clone(&String::from(" ")))?;
                }
                System::out().print_i(arr.borrow()[i as usize])?;
                i = i.wrapping_add(1i32);
            }
            System::out().println()?;
            Ok(())
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 6i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = 5i32;
            _arr0.borrow_mut()[1i32 as usize] = 2i32;
            _arr0.borrow_mut()[2i32 as usize] = 8i32;
            _arr0.borrow_mut()[3i32 as usize] = 1i32;
            _arr0.borrow_mut()[4i32 as usize] = 9i32;
            _arr0.borrow_mut()[5i32 as usize] = 3i32;
            let mut a: Rc<RefCell<Vec<i32>>> = _arr0;
            Self::bubbleSort(Clone::clone(&a))?;
            Self::printArray(Clone::clone(&a))?;
            let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 7i32 as usize]));
            _arr1.borrow_mut()[0i32 as usize] = 64i32;
            _arr1.borrow_mut()[1i32 as usize] = 34i32;
            _arr1.borrow_mut()[2i32 as usize] = 25i32;
            _arr1.borrow_mut()[3i32 as usize] = 12i32;
            _arr1.borrow_mut()[4i32 as usize] = 22i32;
            _arr1.borrow_mut()[5i32 as usize] = 11i32;
            _arr1.borrow_mut()[6i32 as usize] = 90i32;
            let mut b: Rc<RefCell<Vec<i32>>> = _arr1;
            Self::selectionSort(Clone::clone(&b))?;
            Self::printArray(Clone::clone(&b))?;
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 5i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = 12i32;
            _arr2.borrow_mut()[1i32 as usize] = 11i32;
            _arr2.borrow_mut()[2i32 as usize] = 13i32;
            _arr2.borrow_mut()[3i32 as usize] = 5i32;
            _arr2.borrow_mut()[4i32 as usize] = 6i32;
            let mut c: Rc<RefCell<Vec<i32>>> = _arr2;
            Self::insertionSort(Clone::clone(&c))?;
            Self::printArray(Clone::clone(&c))?;
            let _t3: i32 = Self::binarySearch(Clone::clone(&a), 5i32)?;
            System::out().println_v(_t3)?;
            let _t4: i32 = Self::binarySearch(Clone::clone(&a), 7i32)?;
            System::out().println_v(_t4)?;
            let _t5: i32 = Self::binarySearch(Clone::clone(&c), 11i32)?;
            System::out().println_v(_t5)?;
            let mut _arr6: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 5i32 as usize]));
            _arr6.borrow_mut()[0i32 as usize] = 1i32;
            _arr6.borrow_mut()[1i32 as usize] = 2i32;
            _arr6.borrow_mut()[2i32 as usize] = 3i32;
            _arr6.borrow_mut()[3i32 as usize] = 4i32;
            _arr6.borrow_mut()[4i32 as usize] = 5i32;
            let mut d: Rc<RefCell<Vec<i32>>> = _arr6;
            Self::bubbleSort(Clone::clone(&d))?;
            Self::printArray(Clone::clone(&d))?;
            let mut _arr7: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 5i32 as usize]));
            _arr7.borrow_mut()[0i32 as usize] = 5i32;
            _arr7.borrow_mut()[1i32 as usize] = 4i32;
            _arr7.borrow_mut()[2i32 as usize] = 3i32;
            _arr7.borrow_mut()[3i32 as usize] = 2i32;
            _arr7.borrow_mut()[4i32 as usize] = 1i32;
            let mut e: Rc<RefCell<Vec<i32>>> = _arr7;
            Self::insertionSort(Clone::clone(&e))?;
            Self::printArray(Clone::clone(&e))?;
            Ok(())
        }
    }
}
