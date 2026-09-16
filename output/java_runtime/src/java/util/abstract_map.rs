#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractMap"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Map"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Map<TK;TV;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractMap.java"]
    #[inner_classes     = "java/util/Map$Entry:java/util/Map:Entry:1545;java/util/AbstractMap$1:::0;java/util/AbstractMap$2:::0;java/util/AbstractMap$ViewCollection:java/util/AbstractMap:ViewCollection:1032;java/util/AbstractMap$SimpleImmutableEntry:java/util/AbstractMap:SimpleImmutableEntry:9;java/util/AbstractMap$SimpleEntry:java/util/AbstractMap:SimpleEntry:9;java/util/AbstractMap$2$1:::0;java/util/AbstractMap$1$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/AbstractMap;java/util/Map"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct AbstractMap<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/Set<TK;>;"))]
        pub keySet: Object,
        #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/Collection<TV;>;"))]
        pub values: Object,
    }

    impl<K, V> AbstractMap<K, V> {
        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.entrySet()?;
            let _vdispatch1: i32 = if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.size()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.size()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.size()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.size()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.size()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashSet<Object>>() { _d.size()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            Ok(_vdispatch1)
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.size()?;
            Ok((_t0==0))
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractMap.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsKey(&self, mut key: Object) -> Result<bool> {
            let this = self;
            let _t0 = this.entrySet()?;
            let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut i: Object = _vdispatch1;
            let mut e: Object = Default::default();
            loop {
                let _vdispatch2: bool = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch2) { break; }
                let _vdispatch2: Object = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                e = _vdispatch2;
                let _vdispatch3: Object = if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<Object>() { _d.getKey()? } else if let Some(__f) = e.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                if _is_jnull(&_vdispatch3) {
                    return Ok((1i32 != 0i32));
                }
            }
            loop {
                let _vdispatch2: bool = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch2) { break; }
                let _vdispatch2: Object = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                e = _vdispatch2;
                let _vdispatch3: Object = if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<Object>() { _d.getKey()? } else if let Some(__f) = e.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _vdispatch4: bool = if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&_vdispatch3))? } else { Default::default() };
                if _vdispatch4 {
                    return Ok((1i32 != 0i32));
                }
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn get(&self, mut key: Object) -> Result<V> {
            let this = self;
            let _t0 = this.entrySet()?;
            let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut i: Object = _vdispatch1;
            let mut e: Object = Default::default();
            loop {
                let _vdispatch2: bool = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch2) { break; }
                let _vdispatch2: Object = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                e = _vdispatch2;
                let _vdispatch3: Object = if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<Object>() { _d.getKey()? } else if let Some(__f) = e.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                if _is_jnull(&_vdispatch3) {
                    let _vdispatch4: Object = if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<Object>() { _d.getValue()? } else if let Some(__f) = e.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                    return Ok(panic!("null"));
                }
            }
            loop {
                let _vdispatch2: bool = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch2) { break; }
                let _vdispatch2: Object = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                e = _vdispatch2;
                let _vdispatch3: Object = if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getKey()? } else if let Some(_d) = e.0.as_any().downcast_ref::<Object>() { _d.getKey()? } else if let Some(__f) = e.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _vdispatch4: bool = if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&_vdispatch3))? } else { Default::default() };
                if _vdispatch4 {
                    let _vdispatch5: Object = if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_TreeNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ForwardingNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_ReservationNode<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_TreeBin<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<LinkedHashMap_Entry<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<ConcurrentHashMap_Node<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<TreeMap_Entry<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<HashMap_Node<Object, Object>>() { _d.getValue()? } else if let Some(_d) = e.0.as_any().downcast_ref::<Object>() { _d.getValue()? } else if let Some(__f) = e.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                    return Ok(panic!("null"));
                }
            }
            Ok(panic!("null"))
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn put(&self, mut key: K, mut value: V) -> Result<V> {
            let this = self;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn remove_obj(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn putAll(&self, m: Object) -> Result<()> {
            panic!("stub: java/util/AbstractMap.putAll:(Ljava/util/Map;)V")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/AbstractMap.clear:()V")
        }

        #[java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<TK;>;")]
        pub fn keySet(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.keySet:()Ljava/util/Set;")
        }

        #[java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;")]
        pub fn values(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.values:()Ljava/util/Collection;")
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.entrySet:()Ljava/util/Set;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractMap.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/CloneNotSupportedException")]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "eq", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn eq(o1: Object, o2: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractMap.eq:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;TV;)TV;")]
        pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TK;-TV;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/AbstractMap.forEach:(Ljava/util/function/BiConsumer;)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)V")]
        pub fn replaceAll(&self, function: Object) -> Result<()> {
            panic!("stub: java/util/AbstractMap.replaceAll:(Ljava/util/function/BiFunction;)V")
        }

        #[java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putIfAbsent(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove_obj_obj(&self, key: Object, value: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractMap.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;TV;)Z")]
        pub fn replace_obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
            panic!("stub: java/util/AbstractMap.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn replace_obj_obj(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/Function<-TK;+TV;>;)TV;")]
        pub fn computeIfAbsent(&self, mut key: K, mut mappingFunction: Object) -> Result<V> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&mappingFunction))?;
            let _t1 = this.get(Clone::clone(&key))?;
            let mut v: V = _t1;
            let _vdispatch2: Object = if let Some(__f) = mappingFunction.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&key))? } else { Default::default() };
            let mut newValue: V = _vdispatch2;
            if !_is_jnull(&newValue) {
                let _t3 = this.put(Clone::clone(&key), Clone::clone(&newValue))?;
                return Ok(newValue);
            }
            Ok(v)
        }

        #[java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn compute(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;")]
        pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/AbstractMap.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }
    }
}
