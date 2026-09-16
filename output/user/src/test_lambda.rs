#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use java_runtime::prelude::*;
use java_runtime::java::io::*;
use java_runtime::java::lang::*;
use java_runtime::java::lang::reflect::*;
use java_runtime::java::security::*;
use java_runtime::java::util::*;
use java_runtime::java::util::function::*;
use java_runtime::sun::nio::ch::*;
use java_runtime::sun::nio::cs::*;
use java_runtime::sun::security::util::*;
use crate::test_lambda_transformer::TestLambda_Transformer;

#[java_rta_macros::java_class(
    binary_name       = "TestLambda",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "TestLambda.java",
    inner_classes     = "TestLambda$Transformer:TestLambda:Transformer:1544;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "TestLambda;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct TestLambda;

impl TestLambda {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "apply", descriptor = "(ILTestLambda$Transformer;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn apply(mut value: i32, mut t: Object) -> Result<i32> {
        let _vdispatch0: i32 = if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<i32>>>() { (__f)(value)? } else { Default::default() };
        Ok(_vdispatch0)
    }

    #[cfg_attr(any(), java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn main() -> Result<()> {
        let __lam_13: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<i32>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<i32> { TestLambda::lambda_main_0(_la0) });
        let mut doubler = Object::from_any(__lam_13);
        let _t0: i32 = Self::apply(5i32, Clone::clone(&doubler))?;
        System::out().println_v(_t0)?;
        let __lam_34: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<i32>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<i32> { TestLambda::lambda_main_1(_la0) });
        let _t1: i32 = Self::apply(3i32, Clone::clone(&Object::from_any(__lam_34)))?;
        System::out().println_v(_t1)?;
        let mut names = ArrayList::<Object>::new()?;
        let _t2 = names.add_obj(Object::from_any(String::from("Alice").clone()))?;
        let _t3 = names.add_obj(Object::from_any(String::from("Bob").clone()))?;
        let _t4 = names.add_obj(Object::from_any(String::from("Charlie").clone()))?;
        let __lam_48: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<()> { TestLambda::lambda_main_2(_la0) });
        names.forEach(Clone::clone(&Object::from_any(__lam_48)))?;
        let __lam_56: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<bool> { TestLambda::lambda_main_3(_la0) });
        let mut isLong = Object::from_any(__lam_56);
        let _vdispatch5: bool = if let Some(__f) = isLong.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(String::from("Alice").clone()))? } else { Default::default() };
        System::out().println_v(_vdispatch5)?;
        let _vdispatch6: bool = if let Some(__f) = isLong.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(String::from("Bob").clone()))? } else { Default::default() };
        System::out().println_v(_vdispatch6)?;
        let __lam_67: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TestLambda::lambda_main_4(_la0) });
        let mut len = Object::from_any(__lam_67);
        let _vdispatch7: Object = if let Some(__f) = len.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(String::from("Hello").clone()))? } else { Default::default() };
        System::out().println_v(Clone::clone(&_vdispatch7))?;
        let mut _arr8: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![Default::default(); 8i32 as usize]));
        _arr8.borrow_mut()[0i32 as usize] = 3i32;
        _arr8.borrow_mut()[1i32 as usize] = 1i32;
        _arr8.borrow_mut()[2i32 as usize] = 4i32;
        _arr8.borrow_mut()[3i32 as usize] = 1i32;
        _arr8.borrow_mut()[4i32 as usize] = 5i32;
        _arr8.borrow_mut()[5i32 as usize] = 9i32;
        _arr8.borrow_mut()[6i32 as usize] = 2i32;
        _arr8.borrow_mut()[7i32 as usize] = 6i32;
        let _t9: Object = Arrays::asList(Default::default())?;
        let mut nums = ArrayList::<Object>::new_coll(Clone::clone(&_t9))?;
        let __lam_95: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<i32>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<i32> { TestLambda::lambda_main_5(_la0, _la1) });
        Collections::sort_list_compar(Default::default(), Clone::clone(&Object::from_any(__lam_95)))?;
        let _t10 = nums.get(0i32)?;
        System::out().println_v(Clone::clone(&_t10))?;
        let _t11 = nums.size()?;
        let _t12 = nums.get((_t11).wrapping_sub(1i32))?;
        System::out().println_v(Clone::clone(&_t12))?;
        let _t13: Object = Objects::requireNonNull_obj(Object::from_any(System::out().clone()))?;
        let __lam_cap118_0 = System::out();
        let __lam_118: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<()> { PrintStream::println(__lam_cap118_0.clone(), _la0) });
        names.forEach(Clone::clone(&Object::from_any(__lam_118)))?;
        let __lam_121: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<i32>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<i32> { TestLambda::lambda_main_6(_la0) });
        let mut factorial = Object::from_any(__lam_121);
        let _t14: i32 = Self::apply(5i32, Clone::clone(&factorial))?;
        System::out().println_v(_t14)?;
        Ok(())
    }
}
