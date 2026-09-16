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
use crate::test_functional_interface_transformer::TestFunctionalInterface_Transformer;
use crate::test_functional_interface_combiner::TestFunctionalInterface_Combiner;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestFunctionalInterface"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestFunctionalInterface.java"]
    #[inner_classes     = "TestFunctionalInterface$Transformer:TestFunctionalInterface:Transformer:1544;TestFunctionalInterface$Combiner:TestFunctionalInterface:Combiner:1544;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestFunctionalInterface;java/lang/Object"]

    pub struct TestFunctionalInterface;

    impl TestFunctionalInterface {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let __lam_7: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TestFunctionalInterface::lambda_main_0(_la0) });
            let mut upper = Object::from_any(__lam_7);
            let __lam_11: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TestFunctionalInterface::lambda_main_1(_la0) });
            let mut doubled = Object::from_any(__lam_11);
            let _vdispatch0: Object = if let Some(__f) = upper.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(String::from("hello").clone()))? } else { Default::default() };
            System::out().println_v(Clone::clone(&(_vdispatch0).downcast::<String>()))?;
            let _vdispatch1: Object = if let Some(__f) = doubled.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(21i32.into())? } else { Default::default() };
            System::out().println_v(Clone::clone(&_vdispatch1))?;
            let __lam_42: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { TestFunctionalInterface::lambda_main_2(_la0, _la1) });
            let mut add = Object::from_any(__lam_42);
            let __lam_46: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { TestFunctionalInterface::lambda_main_3(_la0, _la1) });
            let mut repeat = Object::from_any(__lam_46);
            let _vdispatch2: Object = if let Some(__f) = add.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(3i32.into(), 4i32.into())? } else { Default::default() };
            System::out().println_v(Clone::clone(&_vdispatch2))?;
            let _vdispatch3: Object = if let Some(__f) = repeat.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(String::from("ab").clone()), 3i32.into())? } else { Default::default() };
            System::out().println_v(Clone::clone(&(_vdispatch3).downcast::<String>()))?;
            let __lam_54: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { String::length(_la0) });
            let mut length = Object::from_any(__lam_54);
            let _vdispatch4: Object = if let Some(__f) = length.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(String::from("hello").clone()))? } else { Default::default() };
            System::out().println_v(Clone::clone(&_vdispatch4))?;
            let __lam_62: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TestFunctionalInterface::lambda_main_4(_la0) });
            let mut sq = Object::from_any(__lam_62);
            let __lam_63: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { Object::toString(_la0) });
            let _vdispatch5: Object = if let Some(__f) = sq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&Object::from_any(__lam_63)))? } else { Default::default() };
            let mut sqStr: Object = _vdispatch5;
            let _vdispatch6: Object = if let Some(__f) = sqStr.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(5i32.into())? } else { Default::default() };
            System::out().println_v(Clone::clone(&(_vdispatch6).downcast::<String>()))?;
            let __lam_68: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<bool> { String::isEmpty(_la0) });
            let mut isEmpty = Object::from_any(__lam_68);
            let _vdispatch7: Object = if let Some(__f) = isEmpty.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut notEmpty: Object = _vdispatch7;
            let _vdispatch8: bool = if let Some(__f) = isEmpty.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(String::from("").clone()))? } else { Default::default() };
            System::out().println_v(_vdispatch8)?;
            let _vdispatch9: bool = if let Some(__f) = notEmpty.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(String::from("hello").clone()))? } else { Default::default() };
            System::out().println_v(_vdispatch9)?;
            let _vdispatch10: Object = if let Some(__f) = isEmpty.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&notEmpty))? } else { Default::default() };
            let _vdispatch11: bool = if let Some(__f) = _vdispatch10.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(String::from("x").clone()))? } else { Default::default() };
            System::out().println_v(_vdispatch11)?;
            let _t12: Object = Objects::requireNonNull_obj(Object::from_any(System::out().clone()))?;
            let __lam_cap96_0 = System::out();
            let __lam_96: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<()> { PrintStream::println(__lam_cap96_0.clone(), _la0) });
            let mut print = Object::from_any(__lam_96);
            if let Some(_d) = print.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef_Dropping<Object>>() { _d.accept(Object::from_any(String::from("consumer works").clone()))?; } else if let Some(_d) = print.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef_Taking<Object>>() { _d.accept(Object::from_any(String::from("consumer works").clone()))?; } else if let Some(_d) = print.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef<Object>>() { _d.accept(Object::from_any(String::from("consumer works").clone()))?; } else if let Some(_d) = print.0.as_any().downcast_ref::<Object>() { _d.accept(Object::from_any(String::from("consumer works").clone()))?; } else if let Some(__f) = print.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Object::from_any(String::from("consumer works").clone()))?; }
            let __lam_106: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { TestFunctionalInterface::lambda_main_5() });
            let mut greeting = Object::from_any(__lam_106);
            let _vdispatch14: Object = if let Some(__f) = greeting.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            System::out().println_v(Clone::clone(&(_vdispatch14).downcast::<String>()))?;
            let __lam_115: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { TestFunctionalInterface::lambda_main_6(_la0, _la1) });
            let mut multiply = Object::from_any(__lam_115);
            let _vdispatch15: Object = if let Some(_d) = multiply.0.as_any().downcast_ref::<Object>() { _d.apply(6i32.into(), 7i32.into())? } else if let Some(_d) = multiply.0.as_any().downcast_ref::<Object>() { _d.apply(6i32.into(), 7i32.into())? } else if let Some(__f) = multiply.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(6i32.into(), 7i32.into())? } else { Default::default() };
            System::out().println_v(Clone::clone(&_vdispatch15))?;
            Ok(())
        }
    }
}
