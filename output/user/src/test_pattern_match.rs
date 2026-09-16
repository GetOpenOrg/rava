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
use crate::test_pattern_match_circle::TestPatternMatch_Circle;
use crate::test_pattern_match_rectangle::TestPatternMatch_Rectangle;
use crate::test_pattern_match_triangle::TestPatternMatch_Triangle;
use crate::test_pattern_match_shape::TestPatternMatch_Shape;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestPatternMatch"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestPatternMatch.java"]
    #[inner_classes     = "TestPatternMatch$Circle:TestPatternMatch:Circle:24;TestPatternMatch$Rectangle:TestPatternMatch:Rectangle:24;TestPatternMatch$Triangle:TestPatternMatch:Triangle:24;TestPatternMatch$Shape:TestPatternMatch:Shape:1544;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestPatternMatch;java/lang/Object"]

    pub struct TestPatternMatch;

    impl TestPatternMatch {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "area", descriptor = "(LTestPatternMatch$Shape;)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn area(s: Object) -> Result<f64> {
            panic!("stub: TestPatternMatch.area:(LTestPatternMatch$Shape;)D")
        }

        #[java_method(name = "describe", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn describe(mut obj: Object) -> Result<String> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&obj))?;
            let mut local_1: Object = obj;
            let mut local_2: i32 = 0i32;
            loop {
                /* TODO: invokedynamic 48 */
                match Object::default() {
                    0 => {
                        let mut i = (local_1).downcast::<i32>();
                        local_2 = 1i32;
                        continue;
                    }
                    1 => {
                        let mut i = (local_1).downcast::<i32>();
                    }
                    2 => {
                        let mut s = (local_1).downcast::<String>();
                        let _t1 = s.isEmpty()?;
                        local_2 = 3i32;
                        continue;
                    }
                    3 => {
                        let mut s = (local_1).downcast::<String>();
                    }
                    _ => {
                    }
                }
                if ((panic!("stack underflow") as i32)!=0) { break; }
                local_2 = 3i32;
            }
            Ok(String::from("empty string"))
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            let _t1: f64 = Self::area(Object::from_any(TestPatternMatch_Circle::new(5.0f64)?.clone()))?;
            _arr0.borrow_mut()[0i32 as usize] = _t1.into();
            let _t2 = System::out().printf_str_arr_obj(Clone::clone(&String::from("%.2f%n")), Clone::clone(&_arr0))?;
            let mut _arr3: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            let _t4: f64 = Self::area(Object::from_any(TestPatternMatch_Rectangle::new(4.0f64, 6.0f64)?.clone()))?;
            _arr3.borrow_mut()[0i32 as usize] = _t4.into();
            let _t5 = System::out().printf_str_arr_obj(Clone::clone(&String::from("%.2f%n")), Clone::clone(&_arr3))?;
            let mut _arr6: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            let _t7: f64 = Self::area(Object::from_any(TestPatternMatch_Triangle::new(3.0f64, 8.0f64)?.clone()))?;
            _arr6.borrow_mut()[0i32 as usize] = _t7.into();
            let _t8 = System::out().printf_str_arr_obj(Clone::clone(&String::from("%.2f%n")), Clone::clone(&_arr6))?;
            let _t9: String = Self::describe(42i32.into())?;
            System::out().println_v(Clone::clone(&_t9))?;
            let _t10: String = Self::describe(-1i32.into())?;
            System::out().println_v(Clone::clone(&_t10))?;
            let _t11: String = Self::describe(Object::from_any(String::from("").clone()))?;
            System::out().println_v(Clone::clone(&_t11))?;
            let _t12: String = Self::describe(Object::from_any(String::from("hello").clone()))?;
            System::out().println_v(Clone::clone(&_t12))?;
            Ok(())
        }
    }
}
