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
use crate::test_nested_generic_pair::TestNestedGeneric_Pair;
use crate::test_nested_generic_box::TestNestedGeneric_Box;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestNestedGeneric"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestNestedGeneric.java"]
    #[inner_classes     = "java/util/Map$Entry:java/util/Map:Entry:1545;TestNestedGeneric$Pair:TestNestedGeneric:Pair:8;TestNestedGeneric$Box:TestNestedGeneric:Box:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestNestedGeneric;java/lang/Object"]

    pub struct TestNestedGeneric;

    impl TestNestedGeneric {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "fromMap", descriptor = "(Ljava/util/Map;)Ljava/util/List;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/Map<TK;TV;>;)Ljava/util/List<LTestNestedGeneric$Pair<TK;TV;>;>;")]
        pub fn fromMap(mut map: Object) -> Result<Object> {
            let mut result = ArrayList::<Object>::new()?;
            let _vdispatch0: Object = if let Some(_d) = map.0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<Properties>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<Object>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.entrySet()? } else if let Some(_d) = map.0.as_any().downcast_ref::<Object>() { _d.entrySet()? } else if let Some(__f) = map.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let _vdispatch1: Object = if let Some(_d) = _vdispatch0.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _vdispatch0.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.iterator()? } else if let Some(_d) = _vdispatch0.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _vdispatch0.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _vdispatch0.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.iterator()? } else if let Some(_d) = _vdispatch0.0.as_any().downcast_ref::<HashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _vdispatch0.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = _vdispatch0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut local_2: Object = _vdispatch1;
            loop {
                let _vdispatch2: bool = if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = local_2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch2) { break; }
                let _vdispatch2: Object = if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = local_2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut e: Object = _vdispatch2;
                let _vdispatch3: Object = if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<Object>() { _d.getKey()? } else if let Some(__f) = e.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _vdispatch4: Object = if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<Object>() { _d.getValue()? } else if let Some(__f) = e.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _t5 = result.add_obj(Object::from_any(TestNestedGeneric_Pair::new(Clone::clone(&_vdispatch3), Clone::clone(&_vdispatch4))?.clone()))?;
            }
            Ok(Object::from_any(result.clone()))
        }

        #[java_method(name = "chunk", descriptor = "(Ljava/util/List;I)Ljava/util/List;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<TT;>;I)Ljava/util/List<Ljava/util/List<TT;>;>;")]
        pub fn chunk(mut list: Object, mut size: i32) -> Result<Object> {
            let mut result = ArrayList::<Object>::new()?;
            let mut i: i32 = 0i32;
            loop {
                let _vdispatch0: i32 = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                if i >= _vdispatch0 { break; }
                let _vdispatch0: i32 = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                let _t1: i32 = Math::min_i_i((i).wrapping_add(size), _vdispatch0)?;
                let mut end: i32 = _t1;
                let _vdispatch2: Object = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.subList(i, end)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.subList(i, end)? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.subList(i, end)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.subList(i, end)? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.subList(i, end)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.subList(i, end)? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.subList(i, end)? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.subList(i, end)? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.subList(i, end)? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(i, end)? } else { Default::default() };
                let _t3 = result.add_obj(Object::from_any(ArrayList::<Object>::new_coll(Clone::clone(&_vdispatch2))?.clone()))?;
                i = (i).wrapping_add(size);
            }
            Ok(Object::from_any(result.clone()))
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut p1 = TestNestedGeneric_Pair::new(Object::from_any(String::from("hello").clone()), 42i32.into())?;
            System::out().println_v(Object::from_any(p1.clone()))?;
            let _t0 = p1.swap()?;
            System::out().println_v(Object::from_any(_t0.clone()))?;
            let mut nested = TestNestedGeneric_Pair::new(Object::from_any(ArrayList::<Object>::new()?.clone()), Object::from_any(String::from("empty").clone()))?;
            let _vdispatch1: bool = if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.add(1i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.add(1i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.add(1i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.add(1i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.add(1i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.add(1i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.add(1i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.add(1i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<Object>() { _d.add(1i32.into())? } else if let Some(__f) = nested.__get_first().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(1i32.into())? } else { Default::default() };
            let _vdispatch2: bool = if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.add(2i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.add(2i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.add(2i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.add(2i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.add(2i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.add(2i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.add(2i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.add(2i32.into())? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<Object>() { _d.add(2i32.into())? } else if let Some(__f) = nested.__get_first().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(2i32.into())? } else { Default::default() };
            let _vdispatch3: i32 = if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = nested.__get_first().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = nested.__get_first().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            System::out().println_v(_vdispatch3)?;
            System::out().println_v(Clone::clone(&(nested.__get_second()).downcast::<String>()))?;
            let mut box_ = TestNestedGeneric_Box::new(21i32.into())?;
            let __lam_113: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TestNestedGeneric::lambda_main_0(_la0) });
            let _t4 = box_.map(Clone::clone(&Object::from_any(__lam_113)))?;
            let mut doubled: TestNestedGeneric_Box<Object> = _t4;
            let __lam_121: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { Object::toString(_la0) });
            let _t5 = doubled.map(Clone::clone(&Object::from_any(__lam_121)))?;
            let mut strBox: TestNestedGeneric_Box<Object> = _t5;
            System::out().println_v(Object::from_any(box_.clone()))?;
            System::out().println_v(Object::from_any(doubled.clone()))?;
            System::out().println_v(Object::from_any(strBox.clone()))?;
            let mut nums = ArrayList::<Object>::new()?;
            let mut i: i32 = 1i32;
            loop {
                if i > 9i32 { break; }
                let _t6 = nums.add_obj(i.into())?;
                i = i.wrapping_add(1i32);
            }
            let _t6: Object = Self::chunk(Object::from_any(nums.clone()), 3i32)?;
            let mut i: Object = _t6;
            let _vdispatch7: i32 = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = i.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            System::out().println_v(_vdispatch7)?;
            let _vdispatch8: Object = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.iterator()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.iterator()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.iterator()? } else if let Some(_d) = i.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.iterator()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut coord = (_vdispatch8).downcast::<TestNestedGeneric_Pair<TestNestedGeneric_Pair<i32, i32>, String>>();
            loop {
                let _t9 = coord.hasNext()?;
                if !(_t9) { break; }
                let _t9 = coord.next()?;
                let mut c: Object = _t9;
                System::out().println_v(Clone::clone(&c))?;
            }
            let mut coord = TestNestedGeneric_Pair::new(Object::from_any(TestNestedGeneric_Pair::new(3i32.into(), 4i32.into())?.clone()), Object::from_any(String::from("origin").clone()))?;
            System::out().println_v(Clone::clone(&coord.__get_first()))?;
            System::out().println_v(Clone::clone(&(coord.__get_second()).downcast::<String>()))?;
            Ok(())
        }
    }
}
