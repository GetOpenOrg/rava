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
    #[binary_name       = "TestMethodRef"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestMethodRef.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestMethodRef;java/lang/Object"]

    pub struct TestMethodRef;

    impl TestMethodRef {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "doubleIt", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleIt(mut x: i32) -> Result<i32> {
            Ok((x).wrapping_mul(2i32))
        }

        #[java_method(name = "isPositive", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPositive(mut x: i32) -> Result<bool> {
            Ok((x>0))
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("hello"));
            _arr0.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("world"));
            _arr0.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("java"));
            let _t1: Object = Arrays::asList(Default::default())?;
            let mut words: Object = _t1;
            let _t2: Object = Objects::requireNonNull_obj(Object::from_any(System::out().clone()))?;
            let __lam_cap33_0 = System::out();
            let __lam_33: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<()> { PrintStream::println(__lam_cap33_0.clone(), _la0) });
            if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Object>() { _d.forEach(Clone::clone(&Object::from_any(__lam_33)))?; } else if let Some(__f) = words.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&Object::from_any(__lam_33)))?; }
            let __lam_43: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TestMethodRef::doubleIt(_la0) });
            let mut dbl = Object::from_any(__lam_43);
            let _vdispatch4: Object = if let Some(__f) = dbl.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(5i32.into())? } else { Default::default() };
            System::out().println_v(Clone::clone(&_vdispatch4))?;
            let mut prefix: String = String::from(">>>>");
            let _t5: Object = Objects::requireNonNull_obj(Object::from_any(prefix.clone()))?;
            let __lam_cap65_0 = prefix;
            let __lam_65: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { String::concat(__lam_cap65_0.clone(), _la0) });
            let mut addPrefix = Object::from_any(__lam_65);
            let _vdispatch6: Object = if let Some(__f) = addPrefix.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(String::from("test").clone()))? } else { Default::default() };
            System::out().println_v(Clone::clone(&(_vdispatch6).downcast::<String>()))?;
            let __lam_73: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { StringBuilder::<init>(_la0) });
            let mut sbMaker = Object::from_any(__lam_73);
            let _vdispatch7: Object = if let Some(__f) = sbMaker.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(String::from("initial").clone()))? } else { Default::default() };
            let mut sb = (_vdispatch7).downcast::<StringBuilder>();
            let _t8 = sb.toString()?;
            System::out().println_v(Clone::clone(&_t8))?;
            Ok(())
        }
    }
}
