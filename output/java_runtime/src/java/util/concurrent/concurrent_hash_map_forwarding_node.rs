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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<ConcurrentHashMap_ForwardingNode<K, V>> for ConcurrentHashMap_Node<K, V> {
    fn from(v: ConcurrentHashMap_ForwardingNode<K, V>) -> ConcurrentHashMap_Node<K, V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/ConcurrentHashMap$ForwardingNode"]
    #[super_class       = "java/util/concurrent/ConcurrentHashMap$Node"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ConcurrentHashMap.java"]
    #[inner_classes     = "java/util/concurrent/ConcurrentHashMap$Node:java/util/concurrent/ConcurrentHashMap:Node:8;java/util/concurrent/ConcurrentHashMap$ForwardingNode:java/util/concurrent/ConcurrentHashMap:ForwardingNode:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ConcurrentHashMap_Node<K, V>"]
    #[superclass_fields(hash: i32, key: K, val: V, next: ConcurrentHashMap_Node<K, V>)]
    #[all_supertypes    = "java/lang/Object;java/util/Map$Entry;java/util/concurrent/ConcurrentHashMap$ForwardingNode;java/util/concurrent/ConcurrentHashMap$Node"]

    pub struct ConcurrentHashMap_ForwardingNode<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "nextTable", descriptor = "[Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "package", modifiers = "final", is_static = false, generic_signature = "[Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;"))]
        pub nextTable: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>,
    }

    impl<K, V> ConcurrentHashMap_ForwardingNode<K, V> {
        #[java_method(name = "<init>", descriptor = "([Ljava/util/concurrent/ConcurrentHashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;)V")]
        pub fn new(mut tab: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ConcurrentHashMap_Node::new_i_obj_obj(-1i32, Default::default(), Default::default())?);
            this.__set_nextTable(Clone::clone(&tab));
            Ok(this)
        }

        #[java_method(name = "find", descriptor = "(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;")]
        pub fn find(&self, h: i32, k: Object) -> Result<ConcurrentHashMap_Node<Object, Object>> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$ForwardingNode.find:(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;")
        }
    }
}
