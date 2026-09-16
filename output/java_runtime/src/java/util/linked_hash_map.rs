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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<LinkedHashMap<K, V>> for HashMap<K, V> {
    fn from(v: LinkedHashMap<K, V>) -> HashMap<K, V> { v.__into_super() }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<LinkedHashMap<K, V>> for AbstractMap<K, V> {
    fn from(v: LinkedHashMap<K, V>) -> AbstractMap<K, V> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/LinkedHashMap"]
    #[super_class       = "java/util/HashMap"]
    #[interfaces        = "java/util/SequencedMap"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/HashMap<TK;TV;>;Ljava/util/SequencedMap<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LinkedHashMap.java"]
    #[inner_classes     = "java/util/LinkedHashMap$Entry:java/util/LinkedHashMap:Entry:8;java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/HashMap$TreeNode:java/util/HashMap:TreeNode:24;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/LinkedHashMap$LinkedKeySet:java/util/LinkedHashMap:LinkedKeySet:16;java/util/LinkedHashMap$LinkedValues:java/util/LinkedHashMap:LinkedValues:16;java/util/LinkedHashMap$LinkedEntrySet:java/util/LinkedHashMap:LinkedEntrySet:16;java/util/LinkedHashMap$ReversedLinkedHashMapView:java/util/LinkedHashMap:ReversedLinkedHashMapView:8;java/util/LinkedHashMap$LinkedEntryIterator:java/util/LinkedHashMap:LinkedEntryIterator:16;java/util/LinkedHashMap$LinkedValueIterator:java/util/LinkedHashMap:LinkedValueIterator:16;java/util/LinkedHashMap$LinkedKeyIterator:java/util/LinkedHashMap:LinkedKeyIterator:16;java/util/LinkedHashMap$LinkedHashIterator:java/util/LinkedHashMap:LinkedHashIterator:1024"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "HashMap<K, V>"]
    #[superclass_fields(keySet: Object, values: Object, table: Rc<RefCell<Vec<HashMap_Node<K, V>>>>, entrySet: Object, size: i32, modCount: i32, threshold: i32, loadFactor: f32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/AbstractMap;java/util/HashMap;java/util/LinkedHashMap;java/util/Map;java/util/SequencedMap"]

    pub struct LinkedHashMap<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "head", descriptor = "Ljava/util/LinkedHashMap$Entry;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/LinkedHashMap$Entry<TK;TV;>;"))]
        pub head: LinkedHashMap_Entry<K, V>,
        #[cfg_attr(any(), java_field(name = "tail", descriptor = "Ljava/util/LinkedHashMap$Entry;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/LinkedHashMap$Entry<TK;TV;>;"))]
        pub tail: LinkedHashMap_Entry<K, V>,
        #[cfg_attr(any(), java_field(name = "accessOrder", descriptor = "Z", access = "package", modifiers = "final", is_static = false))]
        pub accessOrder: bool,
        #[cfg_attr(any(), java_field(name = "putMode", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub putMode: i32,
    }

    impl<K, V> LinkedHashMap<K, V> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "3801124242820219131"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            3801124242820219131i64
        }

        #[cfg_attr(any(), java_field(name = "PUT_NORM", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: PUT_NORM:I
        pub fn PUT_NORM() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "PUT_FIRST", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: PUT_FIRST:I
        pub fn PUT_FIRST() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "PUT_LAST", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: PUT_LAST:I
        pub fn PUT_LAST() -> i32 {
            2
        }

        #[java_method(name = "linkNodeAtEnd", descriptor = "(Ljava/util/LinkedHashMap$Entry;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/LinkedHashMap$Entry<TK;TV;>;)V")]
        pub fn linkNodeAtEnd(&self, p: LinkedHashMap_Entry<K, V>) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.linkNodeAtEnd:(Ljava/util/LinkedHashMap$Entry;)V")
        }

        #[java_method(name = "transferLinks", descriptor = "(Ljava/util/LinkedHashMap$Entry;Ljava/util/LinkedHashMap$Entry;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/LinkedHashMap$Entry<TK;TV;>;Ljava/util/LinkedHashMap$Entry<TK;TV;>;)V")]
        pub fn transferLinks(&self, src: LinkedHashMap_Entry<K, V>, dst: LinkedHashMap_Entry<K, V>) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.transferLinks:(Ljava/util/LinkedHashMap$Entry;Ljava/util/LinkedHashMap$Entry;)V")
        }

        #[java_method(name = "reinitialize", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reinitialize(&self) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.reinitialize:()V")
        }

        #[java_method(name = "newNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$Node<TK;TV;>;")]
        pub fn newNode(&self, hash: i32, key: K, value: V, e: HashMap_Node<K, V>) -> Result<HashMap_Node<Object, Object>> {
            panic!("stub: java/util/LinkedHashMap.newNode:(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;")
        }

        #[java_method(name = "replacementNode", descriptor = "(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$Node<TK;TV;>;")]
        pub fn replacementNode(&self, p: HashMap_Node<K, V>, next: HashMap_Node<K, V>) -> Result<HashMap_Node<Object, Object>> {
            panic!("stub: java/util/LinkedHashMap.replacementNode:(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;")
        }

        #[java_method(name = "newTreeNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn newTreeNode(&self, hash: i32, key: K, value: V, next: HashMap_Node<K, V>) -> Result<HashMap_TreeNode<Object, Object>> {
            panic!("stub: java/util/LinkedHashMap.newTreeNode:(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;")
        }

        #[java_method(name = "replacementTreeNode", descriptor = "(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn replacementTreeNode(&self, p: HashMap_Node<K, V>, next: HashMap_Node<K, V>) -> Result<HashMap_TreeNode<Object, Object>> {
            panic!("stub: java/util/LinkedHashMap.replacementTreeNode:(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;")
        }

        #[java_method(name = "afterNodeRemoval", descriptor = "(Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;)V")]
        pub fn afterNodeRemoval(&self, e: HashMap_Node<K, V>) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.afterNodeRemoval:(Ljava/util/HashMap$Node;)V")
        }

        #[java_method(name = "afterNodeInsertion", descriptor = "(Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn afterNodeInsertion(&self, evict: bool) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.afterNodeInsertion:(Z)V")
        }

        #[java_method(name = "afterNodeAccess", descriptor = "(Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;)V")]
        pub fn afterNodeAccess(&self, e: HashMap_Node<K, V>) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.afterNodeAccess:(Ljava/util/HashMap$Node;)V")
        }

        #[java_method(name = "putFirst", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putFirst(&self, k: K, v: V) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.putFirst:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "putLast", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putLast(&self, k: K, v: V) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.putLast:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "internalWriteEntries", descriptor = "(Ljava/io/ObjectOutputStream;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn internalWriteEntries(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.internalWriteEntries:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "<init>", descriptor = "(IF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(IF)V
        pub fn new_i_f(mut initialCapacity: i32, mut loadFactor: f32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(HashMap::new_i_f(initialCapacity, loadFactor)?);
            this.__set_putMode(0i32);
            this.__set_accessOrder((0i32 != 0i32));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(initialCapacity: i32) -> Result<Self> {
            panic!("stub: java/util/LinkedHashMap.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(HashMap::new()?);
            this.__set_putMode(0i32);
            this.__set_accessOrder((0i32 != 0i32));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn new_map(m: Object) -> Result<Self> {
            panic!("stub: java/util/LinkedHashMap.<init>:(Ljava/util/Map;)V")
        }

        #[java_method(name = "<init>", descriptor = "(IFZ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_f_z(initialCapacity: i32, loadFactor: f32, accessOrder: bool) -> Result<Self> {
            panic!("stub: java/util/LinkedHashMap.<init>:(IFZ)V")
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/LinkedHashMap.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn get(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.get:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;TV;)TV;")]
        pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.clear:()V")
        }

        #[java_method(name = "removeEldestEntry", descriptor = "(Ljava/util/Map$Entry;)Z", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map$Entry<TK;TV;>;)Z")]
        pub fn removeEldestEntry(&self, eldest: Object) -> Result<bool> {
            panic!("stub: java/util/LinkedHashMap.removeEldestEntry:(Ljava/util/Map$Entry;)Z")
        }

        #[java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<TK;>;")]
        pub fn keySet(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.keySet:()Ljava/util/Set;")
        }

        #[java_method(name = "sequencedKeySet", descriptor = "()Ljava/util/SequencedSet;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/SequencedSet<TK;>;")]
        pub fn sequencedKeySet(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.sequencedKeySet:()Ljava/util/SequencedSet;")
        }

        #[java_method(name = "nsee", descriptor = "(Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K1:Ljava/lang/Object;V1:Ljava/lang/Object;>(Ljava/util/HashMap$Node<TK1;TV1;>;)Ljava/util/HashMap$Node<TK1;TV1;>;")]
        pub fn nsee(node: HashMap_Node<Object, Object>) -> Result<HashMap_Node<Object, Object>> {
            panic!("stub: java/util/LinkedHashMap.nsee:(Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;")
        }

        #[java_method(name = "keysToArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        pub fn keysToArray_arr_obj(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/LinkedHashMap.keysToArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
        }

        #[java_method(name = "keysToArray", descriptor = "([Ljava/lang/Object;Z)[Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Z)[TT;")]
        pub fn keysToArray_arr_obj_z(&self, a: Rc<RefCell<Vec<Object>>>, reversed: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/LinkedHashMap.keysToArray:([Ljava/lang/Object;Z)[Ljava/lang/Object;")
        }

        #[java_method(name = "valuesToArray", descriptor = "([Ljava/lang/Object;Z)[Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Z)[TT;")]
        pub fn valuesToArray(&self, a: Rc<RefCell<Vec<Object>>>, reversed: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/LinkedHashMap.valuesToArray:([Ljava/lang/Object;Z)[Ljava/lang/Object;")
        }

        #[java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;")]
        pub fn values(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.values:()Ljava/util/Collection;")
        }

        #[java_method(name = "sequencedValues", descriptor = "()Ljava/util/SequencedCollection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/SequencedCollection<TV;>;")]
        pub fn sequencedValues(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.sequencedValues:()Ljava/util/SequencedCollection;")
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.entrySet:()Ljava/util/Set;")
        }

        #[java_method(name = "sequencedEntrySet", descriptor = "()Ljava/util/SequencedSet;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/SequencedSet<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn sequencedEntrySet(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.sequencedEntrySet:()Ljava/util/SequencedSet;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TK;-TV;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.forEach:(Ljava/util/function/BiConsumer;)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)V")]
        pub fn replaceAll(&self, function: Object) -> Result<()> {
            panic!("stub: java/util/LinkedHashMap.replaceAll:(Ljava/util/function/BiFunction;)V")
        }

        #[java_method(name = "newLinkedHashMap", descriptor = "(I)Ljava/util/LinkedHashMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(I)Ljava/util/LinkedHashMap<TK;TV;>;")]
        pub fn newLinkedHashMap(numMappings: i32) -> Result<LinkedHashMap<Object, Object>> {
            panic!("stub: java/util/LinkedHashMap.newLinkedHashMap:(I)Ljava/util/LinkedHashMap;")
        }

        #[java_method(name = "reversed", descriptor = "()Ljava/util/SequencedMap;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/SequencedMap<TK;TV;>;")]
        pub fn reversed(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashMap.reversed:()Ljava/util/SequencedMap;")
        }
    }
}
