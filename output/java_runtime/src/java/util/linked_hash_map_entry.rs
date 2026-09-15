#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/LinkedHashMap$Entry",
    super_class       = "java/util/HashMap$Node",
    interfaces        = "",
    access            = "package",
    modifiers         = "",
    generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/HashMap$Node<TK;TV;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "LinkedHashMap.java",
    inner_classes     = "java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/LinkedHashMap$Entry:java/util/LinkedHashMap:Entry:8",
    all_supertypes    = "java/lang/Object;java/util/HashMap$Node;java/util/LinkedHashMap$Entry;java/util/Map$Entry",
)]
#[derive(Clone, Default, PartialEq)]
pub struct LinkedHashMap_Entry<K: Clone + Default + 'static, V: Clone + Default + 'static> {
    pub _super: HashMap_Node<K, V>,
    #[cfg_attr(any(), java_field(name = "before", descriptor = "Ljava/util/LinkedHashMap$Entry;", is_static = false, generic_signature = "Ljava/util/LinkedHashMap$Entry<TK;TV;>;"))]
    pub before: JField<LinkedHashMap_Entry<K, V>>,
    #[cfg_attr(any(), java_field(name = "after", descriptor = "Ljava/util/LinkedHashMap$Entry;", is_static = false, generic_signature = "Ljava/util/LinkedHashMap$Entry<TK;TV;>;"))]
    pub after: JField<LinkedHashMap_Entry<K, V>>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> LinkedHashMap_Entry<K, V> {
    pub fn as_hash_map_node(&self) -> &HashMap_Node<K, V> { &self._super }
    pub fn into_hash_map_node(self) -> HashMap_Node<K, V> { self._super }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<LinkedHashMap_Entry<K, V>> for HashMap_Node<K, V> {
    fn from(v: LinkedHashMap_Entry<K, V>) -> HashMap_Node<K, V> { v._super }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> LinkedHashMap_Entry<K, V> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)V"))]
    pub fn new(mut hash: i32, mut key: K, mut value: V, mut next: HashMap_Node<Object, Object>) -> Result<Self> {
        let mut this = Self { _super: Default::default(), before: JField::new(Default::default()), after: JField::new(Default::default()), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = HashMap_Node::new(hash, Clone::clone(&key), Clone::clone(&value), Clone::clone(&next))?;
        Ok(this)
    }
}
