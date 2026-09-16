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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<ConcurrentHashMap_TreeNode<K, V>> for ConcurrentHashMap_Node<K, V> {
    fn from(v: ConcurrentHashMap_TreeNode<K, V>) -> ConcurrentHashMap_Node<K, V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/ConcurrentHashMap$TreeNode"]
    #[super_class       = "java/util/concurrent/ConcurrentHashMap$Node"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ConcurrentHashMap.java"]
    #[inner_classes     = "java/util/concurrent/ConcurrentHashMap$Node:java/util/concurrent/ConcurrentHashMap:Node:8;java/util/concurrent/ConcurrentHashMap$TreeNode:java/util/concurrent/ConcurrentHashMap:TreeNode:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ConcurrentHashMap_Node<K, V>"]
    #[superclass_fields(hash: i32, key: K, val: V, next: ConcurrentHashMap_Node<K, V>)]
    #[all_supertypes    = "java/lang/Object;java/util/Map$Entry;java/util/concurrent/ConcurrentHashMap$Node;java/util/concurrent/ConcurrentHashMap$TreeNode"]

    pub struct ConcurrentHashMap_TreeNode<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;"))]
        pub parent: ConcurrentHashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "left", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;"))]
        pub left: ConcurrentHashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "right", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;"))]
        pub right: ConcurrentHashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "prev", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;"))]
        pub prev: ConcurrentHashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "red", descriptor = "Z", is_static = false))]
        pub red: bool,
    }

    impl<K, V> ConcurrentHashMap_TreeNode<K, V> {
        #[java_method(name = "<init>", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;)V")]
        pub fn new(mut hash: i32, mut key: K, mut val: V, mut next: ConcurrentHashMap_Node<K, V>, mut parent: ConcurrentHashMap_TreeNode<K, V>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ConcurrentHashMap_Node::new_i_obj_obj_concur(hash, Clone::clone(&key), Clone::clone(&val), Clone::clone(&next))?);
            this.__set_parent(Clone::clone(&parent));
            Ok(this)
        }

        #[java_method(name = "find", descriptor = "(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;")]
        pub fn find(&self, h: i32, k: Object) -> Result<ConcurrentHashMap_Node<Object, Object>> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$TreeNode.find:(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;")
        }

        #[java_method(name = "findTreeNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Class;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;Ljava/lang/Class<*>;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;")]
        pub fn findTreeNode(&self, mut h: i32, mut k: Object, mut kc: Class<Object>) -> Result<ConcurrentHashMap_TreeNode<K, V>> {
            let this = self;
            let mut p = this.clone();
            let mut p: ConcurrentHashMap_TreeNode<Object, Object> = Default::default();
            loop {
                let mut pl = p.__get_left();
                let mut pr = p.__get_right();
                let mut ph = p.__get_hash();
                if p.__get_hash() > h {
                    p = pl;
                } else {
                    if ph < h {
                        let mut p: ConcurrentHashMap_TreeNode<Object, Object> = pr;
                    } else {
                        let mut pk = p.__get_key();
                        let _vdispatch0: bool = if let Some(__f) = k.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&pk))? } else { Default::default() };
                        if _vdispatch0 {
                            return Ok(Default::default());
                        }
                        if _is_jnull(&pl) {
                            let mut p: ConcurrentHashMap_TreeNode<Object, Object> = pr;
                        } else {
                            if _is_jnull(&pr) {
                                let mut p: ConcurrentHashMap_TreeNode<Object, Object> = pl;
                            } else {
                                let _t1: Object = ConcurrentHashMap::<Object, Object>::comparableClassFor(Clone::clone(&k))?;
                                kc = _t1;
                                if !_is_jnull(&kc) {
                                    let _t2: i32 = ConcurrentHashMap::<Object, Object>::compareComparables(Clone::clone(&kc), Clone::clone(&k), Clone::clone(&pk))?;
                                    let mut dir: i32 = _t2;
                                    if (dir!=0) {
                                        let mut p = (if (dir<0) { pl } else { pr });
                                    } else {
                                        let _t3 = pr.findTreeNode(h, Clone::clone(&k), Clone::clone(&kc))?;
                                        let mut q: ConcurrentHashMap_TreeNode<Object, Object> = _t3;
                                        if !_is_jnull(&q) {
                                            return Ok(Default::default());
                                        }
                                        let mut p: ConcurrentHashMap_TreeNode<Object, Object> = pl;
                                    }
                                } else {
                                    let _t2 = pr.findTreeNode(h, Clone::clone(&k), Clone::clone(&kc))?;
                                    let mut q: ConcurrentHashMap_TreeNode<Object, Object> = _t2;
                                    if !_is_jnull(&q) {
                                        return Ok(Default::default());
                                    }
                                    let mut p: ConcurrentHashMap_TreeNode<Object, Object> = pl;
                                }
                            }
                        }
                    }
                }
                if _is_jnull(&p) { break; }
            }
            Ok(Default::default())
        }
    }
}
