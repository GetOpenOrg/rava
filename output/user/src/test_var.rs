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
    #[binary_name       = "TestVar"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestVar.java"]
    #[inner_classes     = "java/util/Map$Entry:java/util/Map:Entry:1545;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestVar;java/lang/Object"]

    pub struct TestVar;

    impl TestVar {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut x: i32 = 42i32;
            let mut s: String = String::from("hello");
            let mut list = ArrayList::<Object>::new()?;
            let _t0 = list.add_obj(Object::from_any(String::from("a").clone()))?;
            let _t1 = list.add_obj(Object::from_any(String::from("b").clone()))?;
            let _t2 = list.add_obj(Object::from_any(String::from("c").clone()))?;
            System::out().println_v(x)?;
            System::out().println_v(Clone::clone(&s))?;
            let _t3 = list.size()?;
            System::out().println_v(_t3)?;
            let mut map = TreeMap::<Object, Object>::new()?;
            let _t4 = map.put_obj_obj(Object::from_any(String::from("one").clone()), 1i32.into())?;
            let _t5 = map.put_obj_obj(Object::from_any(String::from("two").clone()), 2i32.into())?;
            let _t6 = map.entrySet()?;
            let _vdispatch7: Object = if let Some(_d) = _t6.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.iterator()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<HashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = _t6.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut local_5: Object = _vdispatch7;
            loop {
                let _vdispatch8: bool = if let Some(_d) = local_5.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = local_5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch8) { break; }
                let _vdispatch8: Object = if let Some(_d) = local_5.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = local_5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut entry: Object = _vdispatch8;
                let _vdispatch9: Object = if let Some(_d) = entry.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<Object>() { _d.getKey()? } else if let Some(__f) = entry.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _vdispatch10: Object = if let Some(_d) = entry.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getValue()? } else if let Some(_d) = entry.0.as_any().downcast_ref::<Object>() { _d.getValue()? } else if let Some(__f) = entry.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                System::out().println_v(Clone::clone(&String::from_owned(format!("{}={}", (_vdispatch9).downcast::<String>(), String::from_owned(format!("{}", _vdispatch10))))))?;
            }
            Ok(())
        }
    }
}
