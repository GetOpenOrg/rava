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
    #[binary_name       = "TestIterator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestIterator.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestIterator;java/lang/Object"]

    pub struct TestIterator;

    impl TestIterator {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut list = ArrayList::<Object>::new()?;
            let _t0 = list.add_obj(Object::from_any(String::from("alpha").clone()))?;
            let _t1 = list.add_obj(Object::from_any(String::from("beta").clone()))?;
            let _t2 = list.add_obj(Object::from_any(String::from("gamma").clone()))?;
            let _t3 = list.iterator()?;
            let mut it: Object = _t3;
            loop {
                let _vdispatch4: bool = if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch4) { break; }
                let _vdispatch4: Object = if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                System::out().println_v(Clone::clone(&(_vdispatch4).downcast::<String>()))?;
            }
            let mut nums = ArrayList::<Object>::new()?;
            let mut i: i32 = 0i32;
            loop {
                if i >= 6i32 { break; }
                let _t4 = nums.add_obj(i.into())?;
                i = i.wrapping_add(1i32);
            }
            let _t4 = nums.iterator()?;
            let mut i: Object = _t4;
            loop {
                let _vdispatch5: bool = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch5) { break; }
                let _vdispatch5: Object = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut n = (_vdispatch5).downcast::<i32>();
                if ((n%2i32)==0) {
                    if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.remove()?; } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.remove()?; } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.remove()?; } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.remove()?; } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.remove()?; } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.remove()?; } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<()>>>() { (__f)()?; }
                }
            }
            System::out().println_v(Object::from_any(nums.clone()))?;
            Ok(())
        }
    }
}
