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
use crate::test_abstract_class_square::TestAbstractClass_Square;
use crate::test_abstract_class_rectangle::TestAbstractClass_Rectangle;
use crate::test_abstract_class_triangle::TestAbstractClass_Triangle;
use crate::test_abstract_class_shape::TestAbstractClass_Shape;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestAbstractClass"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestAbstractClass.java"]
    #[inner_classes     = "TestAbstractClass$Square:TestAbstractClass:Square:8;TestAbstractClass$Rectangle:TestAbstractClass:Rectangle:8;TestAbstractClass$Triangle:TestAbstractClass:Triangle:8;TestAbstractClass$Shape:TestAbstractClass:Shape:1032"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestAbstractClass;java/lang/Object"]

    pub struct TestAbstractClass;

    impl TestAbstractClass {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut s = TestAbstractClass_Square::new(Clone::clone(&String::from("red")), 3.0f64)?;
            let mut r = TestAbstractClass_Rectangle::new(Clone::clone(&String::from("blue")), 4.0f64, 5.0f64)?;
            let mut t = TestAbstractClass_Triangle::new(Clone::clone(&String::from("green")), 6.0f64, 4.0f64)?;
            let _t0 = s.area()?;
            System::out().println_v(_t0)?;
            let _t1 = r.area()?;
            System::out().println_v(_t1)?;
            let _t2 = t.area()?;
            System::out().println_v(_t2)?;
            let _t3 = s.__super().describe()?;
            System::out().println_v(Clone::clone(&_t3))?;
            let _t4 = r.__super().describe()?;
            System::out().println_v(Clone::clone(&_t4))?;
            let _t5 = t.__super().describe()?;
            System::out().println_v(Clone::clone(&_t5))?;
            let mut _arr6: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 4i32 as usize]));
            _arr6.borrow_mut()[0i32 as usize] = Object::from_any(s.clone());
            _arr6.borrow_mut()[1i32 as usize] = Object::from_any(r.clone());
            _arr6.borrow_mut()[2i32 as usize] = Object::from_any(t.clone());
            _arr6.borrow_mut()[3i32 as usize] = Object::from_any(TestAbstractClass_Square::new(Clone::clone(&String::from("black")), 5.0f64)?.clone());
            let mut shapes: Rc<RefCell<Vec<Object>>> = _arr6;
            let mut anyShape: Rc<RefCell<Vec<Object>>> = shapes;
            let mut local_6 = (anyShape.borrow().len() as i32);
            let mut local_7: i32 = 0i32;
            loop {
                if local_7 >= local_6 { break; }
                let mut shape = Clone::clone(&anyShape.borrow()[local_7 as usize]);
                let _vdispatch7: f64 = if let Some(_d) = shape.0.as_any().downcast_ref::<TestAbstractClass_Triangle>() { _d.area()? } else if let Some(_d) = shape.0.as_any().downcast_ref::<TestAbstractClass_Square>() { _d.area()? } else if let Some(_d) = shape.0.as_any().downcast_ref::<TestAbstractClass_Rectangle>() { _d.area()? } else if let Some(_d) = shape.0.as_any().downcast_ref::<TestAbstractClass_Shape>() { _d.area()? } else if let Some(__f) = shape.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<f64>>>() { (__f)()? } else { Default::default() };
                System::out().println_v(_vdispatch7)?;
                local_7 = local_7.wrapping_add(1i32);
            }
            let mut anyShape: TestAbstractClass_Square = s;
            System::out().println_v(true)?;
            System::out().println_v(true)?;
            let mut anyShape: TestAbstractClass_Rectangle = r;
            System::out().println_v(false)?;
            Ok(())
        }
    }
}
