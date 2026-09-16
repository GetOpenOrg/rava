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
    #[binary_name       = "TestComparator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestComparator.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestComparator;java/lang/Object"]

    pub struct TestComparator;

    impl TestComparator {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 4i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("banana"));
            _arr0.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("apple"));
            _arr0.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("cherry"));
            _arr0.borrow_mut()[3i32 as usize] = Clone::clone(&String::from("date"));
            let _t1: Object = Arrays::asList(Default::default())?;
            let mut words: Object = _t1;
            let _t2: Object = Comparator::<Object>::naturalOrder()?;
            if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Object>() { _d.sort(Clone::clone(&_t2))?; } else if let Some(__f) = words.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&_t2))?; }
            System::out().println_v(Clone::clone(&words))?;
            let __lam_47: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<i32> { String::length(_la0) });
            let _t4: Object = Comparator::<Object>::comparingInt(Clone::clone(&Object::from_any(__lam_47)))?;
            if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Object>() { _d.sort(Clone::clone(&_t4))?; } else if let Some(__f) = words.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&_t4))?; }
            System::out().println_v(Clone::clone(&words))?;
            let __lam_47: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<i32> { String::length(_la0) });
            let _t6: Object = Comparator::<Object>::comparingInt(Clone::clone(&Object::from_any(__lam_47)))?;
            let _t7: Object = Comparator::<Object>::naturalOrder()?;
            let _vdispatch8: Object = if let Some(_d) = _t6.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.thenComparing_compar(Clone::clone(&_t7))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Collections_ReverseComparator>() { _d.thenComparing_compar(Clone::clone(&_t7))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Object>() { _d.thenComparing(Clone::clone(&_t7))? } else if let Some(__f) = _t6.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t7))? } else { Default::default() };
            if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Object>() { _d.sort(Clone::clone(&_vdispatch8))?; } else if let Some(__f) = words.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&_vdispatch8))?; }
            System::out().println_v(Clone::clone(&words))?;
            let _t10: Object = Comparator::<Object>::reverseOrder()?;
            if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(_d) = words.0.as_any().downcast_ref::<Object>() { _d.sort(Clone::clone(&_t10))?; } else if let Some(__f) = words.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&_t10))?; }
            System::out().println_v(Clone::clone(&words))?;
            Ok(())
        }
    }
}
