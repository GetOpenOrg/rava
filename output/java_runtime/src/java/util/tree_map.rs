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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<TreeMap<K, V>> for AbstractMap<K, V> {
    fn from(v: TreeMap<K, V>) -> AbstractMap<K, V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/TreeMap"]
    #[super_class       = "java/util/AbstractMap"]
    #[interfaces        = "java/util/NavigableMap,java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/AbstractMap<TK;TV;>;Ljava/util/NavigableMap<TK;TV;>;Ljava/lang/Cloneable;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TreeMap.java"]
    #[inner_classes     = "java/util/TreeMap$Entry:java/util/TreeMap:Entry:24;java/util/TreeMap$EntrySet:java/util/TreeMap:EntrySet:0;java/util/TreeMap$KeySet:java/util/TreeMap:KeySet:24;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/TreeMap$Values:java/util/TreeMap:Values:0;java/util/TreeMap$DescendingSubMap:java/util/TreeMap:DescendingSubMap:24;java/util/TreeMap$AscendingSubMap:java/util/TreeMap:AscendingSubMap:24;java/util/TreeMap$KeyIterator:java/util/TreeMap:KeyIterator:16;java/util/TreeMap$DescendingKeyIterator:java/util/TreeMap:DescendingKeyIterator:16;java/util/AbstractMap$SimpleImmutableEntry:java/util/AbstractMap:SimpleImmutableEntry:9;java/util/TreeMap$NavigableSubMap:java/util/TreeMap:NavigableSubMap:1032;java/util/TreeMap$KeySpliterator:java/util/TreeMap:KeySpliterator:24;java/util/TreeMap$DescendingKeySpliterator:java/util/TreeMap:DescendingKeySpliterator:24;java/util/TreeMap$EntrySpliterator:java/util/TreeMap:EntrySpliterator:24;java/util/TreeMap$ValueSpliterator:java/util/TreeMap:ValueSpliterator:24;java/util/TreeMap$TreeMapSpliterator:java/util/TreeMap:TreeMapSpliterator:8;java/util/TreeMap$SubMap:java/util/TreeMap:SubMap:2;java/util/TreeMap$ValueIterator:java/util/TreeMap:ValueIterator:16;java/util/TreeMap$EntryIterator:java/util/TreeMap:EntryIterator:16;java/util/TreeMap$PrivateEntryIterator:java/util/TreeMap:PrivateEntryIterator:1024;java/util/TreeMap$DescendingSubMap$DescendingEntrySetView:java/util/TreeMap$DescendingSubMap:DescendingEntrySetView:16;java/util/TreeMap$AscendingSubMap$AscendingEntrySetView:java/util/TreeMap$AscendingSubMap:AscendingEntrySetView:16;java/util/TreeMap$NavigableSubMap$DescendingSubMapKeyIterator:java/util/TreeMap$NavigableSubMap:DescendingSubMapKeyIterator:16;java/util/TreeMap$NavigableSubMap$SubMapKeyIterator:java/util/TreeMap$NavigableSubMap:SubMapKeyIterator:16;java/util/TreeMap$NavigableSubMap$DescendingSubMapEntryIterator:java/util/TreeMap$NavigableSubMap:DescendingSubMapEntryIterator:16;java/util/TreeMap$NavigableSubMap$SubMapEntryIterator:java/util/TreeMap$NavigableSubMap:SubMapEntryIterator:16;java/util/TreeMap$NavigableSubMap$SubMapIterator:java/util/TreeMap$NavigableSubMap:SubMapIterator:1024;java/util/TreeMap$NavigableSubMap$EntrySetView:java/util/TreeMap$NavigableSubMap:EntrySetView:1024"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractMap<K, V>"]
    #[superclass_fields(keySet: Object, values: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/AbstractMap;java/util/Map;java/util/NavigableMap;java/util/TreeMap"]

    pub struct TreeMap<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "comparator", descriptor = "Ljava/util/Comparator;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/Comparator<-TK;>;"))]
        pub comparator: Object,
        #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/TreeMap$Entry;", access = "private", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/TreeMap$Entry<TK;TV;>;"))]
        pub root: TreeMap_Entry<K, V>,
        #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub size: i32,
        #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub modCount: i32,
        #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/TreeMap$EntrySet;", access = "private", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/TreeMap<TK;TV;>.EntrySet;"))]
        pub entrySet: TreeMap<K, V>,
        #[cfg_attr(any(), java_field(name = "navigableKeySet", descriptor = "Ljava/util/TreeMap$KeySet;", access = "private", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/TreeMap$KeySet<TK;>;"))]
        pub navigableKeySet: Object,
        #[cfg_attr(any(), java_field(name = "descendingMap", descriptor = "Ljava/util/NavigableMap;", access = "private", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/NavigableMap<TK;TV;>;"))]
        pub descendingMap: Object,
    }

    impl<K, V> TreeMap<K, V> {
        #[cfg_attr(any(), java_field(name = "UNBOUNDED", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
        // static field: UNBOUNDED:Ljava/lang/Object;
        pub fn UNBOUNDED() -> Object {
            panic!("stub: java/util/TreeMap.UNBOUNDED:Ljava/lang/Object;")
        }

        #[cfg_attr(any(), java_field(name = "RED", descriptor = "Z", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: RED:Z
        pub fn RED() -> bool {
            false
        }

        #[cfg_attr(any(), java_field(name = "BLACK", descriptor = "Z", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: BLACK:Z
        pub fn BLACK() -> bool {
            true
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "919286545866124006"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            919286545866124006i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractMap::new()?);
            this.__set_size(0i32);
            this.__set_modCount(0i32);
            this.__set_comparator(Clone::clone(&Object::default()));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Comparator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TK;>;)V")]
        pub fn new_compar(comparator: Object) -> Result<Self> {
            panic!("stub: java/util/TreeMap.<init>:(Ljava/util/Comparator;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn new_map(m: Object) -> Result<Self> {
            panic!("stub: java/util/TreeMap.<init>:(Ljava/util/Map;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/SortedMap;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/SortedMap<TK;+TV;>;)V")]
        pub fn new_sorted(m: Object) -> Result<Self> {
            panic!("stub: java/util/TreeMap.<init>:(Ljava/util/SortedMap;)V")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/TreeMap.size:()I")
        }

        #[java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsKey(&self, key: Object) -> Result<bool> {
            panic!("stub: java/util/TreeMap.containsKey:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/TreeMap.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn get(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.get:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "comparator", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Comparator<-TK;>;")]
        pub fn comparator(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.comparator:()Ljava/util/Comparator;")
        }

        #[java_method(name = "firstKey", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TK;")]
        pub fn firstKey(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.firstKey:()Ljava/lang/Object;")
        }

        #[java_method(name = "lastKey", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TK;")]
        pub fn lastKey(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.lastKey:()Ljava/lang/Object;")
        }

        #[java_method(name = "putFirst", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putFirst(&self, k: K, v: V) -> Result<Object> {
            panic!("stub: java/util/TreeMap.putFirst:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "putLast", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putLast(&self, k: K, v: V) -> Result<Object> {
            panic!("stub: java/util/TreeMap.putLast:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn putAll(&self, map: Object) -> Result<()> {
            panic!("stub: java/util/TreeMap.putAll:(Ljava/util/Map;)V")
        }

        #[java_method(name = "getEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn getEntry(&self, key: Object) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.getEntry:(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "getEntryUsingComparator", descriptor = "(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn getEntryUsingComparator(&self, key: Object) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.getEntryUsingComparator:(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "getCeilingEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn getCeilingEntry(&self, key: K) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.getCeilingEntry:(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "getFloorEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn getFloorEntry(&self, key: K) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.getFloorEntry:(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "getHigherEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn getHigherEntry(&self, key: K) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.getHigherEntry:(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "getLowerEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn getLowerEntry(&self, key: K) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.getLowerEntry:(Ljava/lang/Object;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
        pub fn put_obj_obj(&self, mut key: K, mut value: V) -> Result<V> {
            let this = self;
            let _t0 = this.put_obj_obj_z(Clone::clone(&key), Clone::clone(&value), (1i32 != 0i32))?;
            Ok(panic!("null"))
        }

        #[java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putIfAbsent(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/TreeMap.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/Function<-TK;+TV;>;)TV;")]
        pub fn computeIfAbsent(&self, key: K, mappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.computeIfAbsent:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn compute(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;")]
        pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "callMappingFunctionWithCheck", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/Function<-TK;+TV;>;)TV;")]
        pub fn callMappingFunctionWithCheck(&self, key: K, mappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.callMappingFunctionWithCheck:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "callRemappingFunctionWithCheck", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn callRemappingFunctionWithCheck(&self, key: K, oldValue: V, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.callRemappingFunctionWithCheck:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "addEntry", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/TreeMap$Entry<TK;TV;>;Z)V")]
        pub fn addEntry(&self, mut key: K, mut value: V, mut parent: TreeMap_Entry<K, V>, mut addToLeft: bool) -> Result<()> {
            let this = self;
            let mut e = TreeMap_Entry::<Object, Object>::new(Clone::clone(&key), Clone::clone(&value), Clone::clone(&parent))?;
            if addToLeft {
                parent.__set_left(Clone::clone(&e));
            } else {
                parent.__set_right(Clone::clone(&e));
            }
            this.fixAfterInsertion(Clone::clone(&e))?;
            this.__set_size((this.__get_size()).wrapping_add(1i32));
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(())
        }

        #[java_method(name = "addEntryToEmptyMap", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)V")]
        pub fn addEntryToEmptyMap(&self, mut key: K, mut value: V) -> Result<()> {
            let this = self;
            let _t0 = this.compare(Clone::clone(&key), Clone::clone(&key))?;
            this.__set_root(TreeMap_Entry::<Object, Object>::new(Clone::clone(&key), Clone::clone(&value), Default::default())?);
            this.__set_size(1i32);
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(())
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Z)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Z)TV;")]
        // java: put(Ljava/lang/Object;Ljava/lang/Object;Z)Ljava/lang/Object;
        pub fn put_obj_obj_z(&self, mut key: K, mut value: V, mut replaceOld: bool) -> Result<V> {
            let this = self;
            let mut t = this.__get_root();
            if _is_jnull(&t) {
                this.addEntryToEmptyMap(Clone::clone(&key), Clone::clone(&value))?;
                return Ok(panic!("null"));
            }
            let mut cpr = this.__get_comparator();
        let mut cmp: i32 = Default::default();
        let mut parent: TreeMap_Entry<Object, Object> = Default::default();
            if !_is_jnull(&cpr) {
                loop {
                    parent = t;
                    let _vdispatch0: i32 = if let Some(_d) = cpr.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compare(Clone::clone(&key), Clone::clone(&t.__get_key()))? } else if let Some(_d) = cpr.0.as_any().downcast_ref::<Collections_ReverseComparator>() { _d.compare(Clone::clone(&key), Clone::clone(&t.__get_key()))? } else if let Some(_d) = cpr.0.as_any().downcast_ref::<Object>() { _d.compare(Clone::clone(&key), Clone::clone(&t.__get_key()))? } else if let Some(__f) = cpr.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&key), Clone::clone(&t.__get_key()))? } else { Default::default() };
                    cmp = _vdispatch0;
                    if (cmp<0) {
                        t = t.__get_left();
                    } else {
                        if (cmp>0) {
                            t = t.__get_right();
                        } else {
                            let mut oldValue = t.__get_value();
                            if _is_jnull(&oldValue) {
                                t.__set_value(Clone::clone(&value));
                            }
                            return Ok(oldValue);
                        }
                    }
                    if _is_jnull(&t) { break; }
                }
            } else {
                let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&key))?;
                let mut oldValue: V = key;
                loop {
                    parent = t;
                    let _t1 = oldValue.compareTo(Clone::clone(&t.__get_key()))?;
                    cmp = _t1;
                    if (cmp<0) {
                        t = t.__get_left();
                    } else {
                        if (cmp>0) {
                            t = t.__get_right();
                        } else {
                            let mut oldValue = t.__get_value();
                            if _is_jnull(&oldValue) {
                                t.__set_value(Clone::clone(&value));
                            }
                            return Ok(oldValue);
                        }
                    }
                    if _is_jnull(&t) { break; }
                }
            }
            this.addEntry(Clone::clone(&key), Clone::clone(&value), Clone::clone(&parent), (cmp<0))?;
            Ok(panic!("null"))
        }

        #[java_method(name = "remapValue", descriptor = "(Ljava/util/TreeMap$Entry;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/TreeMap$Entry<TK;TV;>;TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn remapValue(&self, t: TreeMap_Entry<K, V>, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.remapValue:(Ljava/util/TreeMap$Entry;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "mergeValue", descriptor = "(Ljava/util/TreeMap$Entry;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/TreeMap$Entry<TK;TV;>;TV;Ljava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;")]
        pub fn mergeValue(&self, t: TreeMap_Entry<K, V>, value: V, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.mergeValue:(Ljava/util/TreeMap$Entry;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn remove(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/TreeMap.clear:()V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "firstEntry", descriptor = "()Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn firstEntry(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.firstEntry:()Ljava/util/Map$Entry;")
        }

        #[java_method(name = "lastEntry", descriptor = "()Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn lastEntry(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.lastEntry:()Ljava/util/Map$Entry;")
        }

        #[java_method(name = "pollFirstEntry", descriptor = "()Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn pollFirstEntry(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.pollFirstEntry:()Ljava/util/Map$Entry;")
        }

        #[java_method(name = "pollLastEntry", descriptor = "()Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn pollLastEntry(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.pollLastEntry:()Ljava/util/Map$Entry;")
        }

        #[java_method(name = "lowerEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn lowerEntry(&self, key: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.lowerEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
        }

        #[java_method(name = "lowerKey", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)TK;")]
        pub fn lowerKey(&self, key: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.lowerKey:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "floorEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn floorEntry(&self, key: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.floorEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
        }

        #[java_method(name = "floorKey", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)TK;")]
        pub fn floorKey(&self, key: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.floorKey:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "ceilingEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn ceilingEntry(&self, key: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.ceilingEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
        }

        #[java_method(name = "ceilingKey", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)TK;")]
        pub fn ceilingKey(&self, key: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.ceilingKey:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "higherEntry", descriptor = "(Ljava/lang/Object;)Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn higherEntry(&self, key: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.higherEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
        }

        #[java_method(name = "higherKey", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)TK;")]
        pub fn higherKey(&self, key: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.higherKey:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<TK;>;")]
        pub fn keySet(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.keySet:()Ljava/util/Set;")
        }

        #[java_method(name = "navigableKeySet", descriptor = "()Ljava/util/NavigableSet;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/NavigableSet<TK;>;")]
        pub fn navigableKeySet(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.navigableKeySet:()Ljava/util/NavigableSet;")
        }

        #[java_method(name = "descendingKeySet", descriptor = "()Ljava/util/NavigableSet;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/NavigableSet<TK;>;")]
        pub fn descendingKeySet(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.descendingKeySet:()Ljava/util/NavigableSet;")
        }

        #[java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;")]
        pub fn values(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.values:()Ljava/util/Collection;")
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            let this = self;
            let mut es = this.__get_entrySet();
            let mut _merged0: TreeMap_EntrySet;
            if !_is_jnull(&es) {
                _merged0 = es;
            } else {
                this.__set_entrySet(TreeMap_EntrySet::new(Clone::clone(this))?);
                _merged0 = TreeMap_EntrySet::new(Clone::clone(this))?;
            }
            Ok(Object::from_any(_merged0.clone()))
        }

        #[java_method(name = "descendingMap", descriptor = "()Ljava/util/NavigableMap;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/NavigableMap<TK;TV;>;")]
        pub fn descendingMap(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.descendingMap:()Ljava/util/NavigableMap;")
        }

        #[java_method(name = "subMap", descriptor = "(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;ZTK;Z)Ljava/util/NavigableMap<TK;TV;>;")]
        pub fn subMap_obj_z_obj_z(&self, fromKey: K, fromInclusive: bool, toKey: K, toInclusive: bool) -> Result<Object> {
            panic!("stub: java/util/TreeMap.subMap:(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;")
        }

        #[java_method(name = "headMap", descriptor = "(Ljava/lang/Object;Z)Ljava/util/NavigableMap;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Z)Ljava/util/NavigableMap<TK;TV;>;")]
        pub fn headMap_obj_z(&self, toKey: K, inclusive: bool) -> Result<Object> {
            panic!("stub: java/util/TreeMap.headMap:(Ljava/lang/Object;Z)Ljava/util/NavigableMap;")
        }

        #[java_method(name = "tailMap", descriptor = "(Ljava/lang/Object;Z)Ljava/util/NavigableMap;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Z)Ljava/util/NavigableMap<TK;TV;>;")]
        pub fn tailMap_obj_z(&self, fromKey: K, inclusive: bool) -> Result<Object> {
            panic!("stub: java/util/TreeMap.tailMap:(Ljava/lang/Object;Z)Ljava/util/NavigableMap;")
        }

        #[java_method(name = "subMap", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TK;)Ljava/util/SortedMap<TK;TV;>;")]
        pub fn subMap_obj_obj(&self, fromKey: K, toKey: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.subMap:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;")
        }

        #[java_method(name = "headMap", descriptor = "(Ljava/lang/Object;)Ljava/util/SortedMap;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/SortedMap<TK;TV;>;")]
        pub fn headMap_obj(&self, toKey: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.headMap:(Ljava/lang/Object;)Ljava/util/SortedMap;")
        }

        #[java_method(name = "tailMap", descriptor = "(Ljava/lang/Object;)Ljava/util/SortedMap;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)Ljava/util/SortedMap<TK;TV;>;")]
        pub fn tailMap_obj(&self, fromKey: K) -> Result<Object> {
            panic!("stub: java/util/TreeMap.tailMap:(Ljava/lang/Object;)Ljava/util/SortedMap;")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;TV;)Z")]
        pub fn replace_obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
            panic!("stub: java/util/TreeMap.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn replace_obj_obj(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/TreeMap.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TK;-TV;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/TreeMap.forEach:(Ljava/util/function/BiConsumer;)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)V")]
        pub fn replaceAll(&self, function: Object) -> Result<()> {
            panic!("stub: java/util/TreeMap.replaceAll:(Ljava/util/function/BiFunction;)V")
        }

        #[java_method(name = "keyIterator", descriptor = "()Ljava/util/Iterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TK;>;")]
        pub fn keyIterator(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.keyIterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "descendingKeyIterator", descriptor = "()Ljava/util/Iterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TK;>;")]
        pub fn descendingKeyIterator(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.descendingKeyIterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "compare", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(&self, mut k1: Object, mut k2: Object) -> Result<i32> {
            let this = self;
            let mut _merged1: i32;
            if _is_jnull(&this.__get_comparator()) {
                let _vdispatch0: i32 = if let Some(_d) = k1.0.as_any().downcast_ref::<BuddhistCalendar>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<UTF_8>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<GregorianCalendar>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<JapaneseImperialCalendar>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<HeapByteBuffer>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Unicode>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<US_ASCII>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<ISO_8859_1>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Collector_Characteristics>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Locale_Category>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Normalizer_Form>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<StreamShape>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<MatchOps_MatchKind>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Pattern_Qtype>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Formatter_BigDecimalLayoutForm>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<DayOfWeek>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<RoundingMode>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<ZoneOffsetTransitionRule_TimeDefinition>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Month>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<ChronoField>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Character_UnicodeScript>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<StreamOpFlag>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<TestSwitchEnum_Season>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<TestEnumMethods_Planet>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<TestEnumBasic_Day>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<LocalTime>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<ZoneOffsetTransition>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Instant>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<ZoneOffset>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<BigDecimal>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<BigInteger>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Short>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Byte>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Date>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Calendar>() { _d.compareTo_obj(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<CharBuffer>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<ByteBuffer>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Charset>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Float>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<i64>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<bool>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<f64>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<TestComparable_Version>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<TestComparable_Student>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(_d) = k1.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&k2))? } else if let Some(__f) = k1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&k2))? } else { Default::default() };
                _merged1 = _vdispatch0;
            } else {
                let _vdispatch0: i32 = if let Some(_d) = this.__get_comparator().0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compare(Clone::clone(&k1), Clone::clone(&k2))? } else if let Some(_d) = this.__get_comparator().0.as_any().downcast_ref::<Collections_ReverseComparator>() { _d.compare(Clone::clone(&k1), Clone::clone(&k2))? } else if let Some(_d) = this.__get_comparator().0.as_any().downcast_ref::<Object>() { _d.compare(Clone::clone(&k1), Clone::clone(&k2))? } else if let Some(__f) = this.__get_comparator().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&k1), Clone::clone(&k2))? } else { Default::default() };
                _merged1 = _vdispatch0;
            }
            Ok(_merged1)
        }

        #[java_method(name = "valEquals", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valEquals(o1: Object, o2: Object) -> Result<bool> {
            panic!("stub: java/util/TreeMap.valEquals:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "exportEntry", descriptor = "(Ljava/util/TreeMap$Entry;)Ljava/util/Map$Entry;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;)Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn exportEntry(e: TreeMap_Entry<K, V>) -> Result<Object> {
            panic!("stub: java/util/TreeMap.exportEntry:(Ljava/util/TreeMap$Entry;)Ljava/util/Map$Entry;")
        }

        #[java_method(name = "keyOrNull", descriptor = "(Ljava/util/TreeMap$Entry;)Ljava/lang/Object;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;)TK;")]
        pub fn keyOrNull(e: TreeMap_Entry<K, V>) -> Result<Object> {
            panic!("stub: java/util/TreeMap.keyOrNull:(Ljava/util/TreeMap$Entry;)Ljava/lang/Object;")
        }

        #[java_method(name = "key", descriptor = "(Ljava/util/TreeMap$Entry;)Ljava/lang/Object;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;*>;)TK;")]
        pub fn key(e: TreeMap_Entry<K, Object>) -> Result<Object> {
            panic!("stub: java/util/TreeMap.key:(Ljava/util/TreeMap$Entry;)Ljava/lang/Object;")
        }

        #[java_method(name = "getFirstEntry", descriptor = "()Ljava/util/TreeMap$Entry;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn getFirstEntry(&self) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.getFirstEntry:()Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "getLastEntry", descriptor = "()Ljava/util/TreeMap$Entry;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn getLastEntry(&self) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.getLastEntry:()Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "successor", descriptor = "(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn successor(t: TreeMap_Entry<K, V>) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.successor:(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "predecessor", descriptor = "(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn predecessor(t: TreeMap_Entry<K, V>) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.predecessor:(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "colorOf", descriptor = "(Ljava/util/TreeMap$Entry;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;)Z")]
        pub fn colorOf(mut p: TreeMap_Entry<K, V>) -> Result<bool> {
            Ok((if _is_jnull(&p) { (1i32 != 0) } else { p.__get_color() }))
        }

        #[java_method(name = "parentOf", descriptor = "(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn parentOf(mut p: TreeMap_Entry<K, V>) -> Result<TreeMap_Entry<K, V>> {
            Ok(Default::default())
        }

        #[java_method(name = "setColor", descriptor = "(Ljava/util/TreeMap$Entry;Z)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;Z)V")]
        pub fn setColor(mut p: TreeMap_Entry<K, V>, mut c: bool) -> Result<()> {
            if !_is_jnull(&p) {
                p.__set_color(c);
            }
            Ok(())
        }

        #[java_method(name = "leftOf", descriptor = "(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn leftOf(mut p: TreeMap_Entry<K, V>) -> Result<TreeMap_Entry<K, V>> {
            Ok(Default::default())
        }

        #[java_method(name = "rightOf", descriptor = "(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/TreeMap$Entry<TK;TV;>;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn rightOf(mut p: TreeMap_Entry<K, V>) -> Result<TreeMap_Entry<K, V>> {
            Ok(Default::default())
        }

        #[java_method(name = "rotateLeft", descriptor = "(Ljava/util/TreeMap$Entry;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/TreeMap$Entry<TK;TV;>;)V")]
        pub fn rotateLeft(&self, mut p: TreeMap_Entry<K, V>) -> Result<()> {
            let this = self;
            let mut r = p.__get_right();
            p.__set_right(Clone::clone(&r.__get_left()));
            if !_is_jnull(&r.__get_left()) {
                r.__get_left().__set_parent(Clone::clone(&p));
            }
            r.__set_parent(Clone::clone(&p.__get_parent()));
            if _is_jnull(&p.__get_parent()) {
                this.__set_root(Clone::clone(&r));
            } else {
                if Object::from_any(p.__get_parent().__get_left().clone()) == Object::from_any(p.clone()) {
                    p.__get_parent().__set_left(Clone::clone(&r));
                } else {
                    p.__get_parent().__set_right(Clone::clone(&r));
                }
            }
            r.__set_left(Clone::clone(&p));
            p.__set_parent(Clone::clone(&r));
            Ok(())
        }

        #[java_method(name = "rotateRight", descriptor = "(Ljava/util/TreeMap$Entry;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/TreeMap$Entry<TK;TV;>;)V")]
        pub fn rotateRight(&self, mut p: TreeMap_Entry<K, V>) -> Result<()> {
            let this = self;
            let mut l = p.__get_left();
            p.__set_left(Clone::clone(&l.__get_right()));
            if !_is_jnull(&l.__get_right()) {
                l.__get_right().__set_parent(Clone::clone(&p));
            }
            l.__set_parent(Clone::clone(&p.__get_parent()));
            if _is_jnull(&p.__get_parent()) {
                this.__set_root(Clone::clone(&l));
            } else {
                if Object::from_any(p.__get_parent().__get_right().clone()) == Object::from_any(p.clone()) {
                    p.__get_parent().__set_right(Clone::clone(&l));
                } else {
                    p.__get_parent().__set_left(Clone::clone(&l));
                }
            }
            l.__set_right(Clone::clone(&p));
            p.__set_parent(Clone::clone(&l));
            Ok(())
        }

        #[java_method(name = "fixAfterInsertion", descriptor = "(Ljava/util/TreeMap$Entry;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/TreeMap$Entry<TK;TV;>;)V")]
        pub fn fixAfterInsertion(&self, mut x: TreeMap_Entry<K, V>) -> Result<()> {
            let this = self;
            x.__set_color((0i32 != 0i32));
            loop {
                if _is_jnull(&x) { break; }
                let _t0: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                let _t1: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                let _t2: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t1))?;
                let _t3: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::leftOf(Clone::clone(&_t2))?;
                let _t4: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                let _t5: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t4))?;
                let _t6: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::rightOf(Clone::clone(&_t5))?;
                let mut y: TreeMap_Entry<Object, Object> = _t6;
                let _t7: bool = TreeMap::<Object, Object>::colorOf(Clone::clone(&y))?;
                if !(_t7) {
                    let _t8: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&_t8), (1i32 != 0i32))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&y), (1i32 != 0i32))?;
                    let _t9: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t10: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t9))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&_t10), (0i32 != 0i32))?;
                    let _t11: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t12: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t11))?;
                    let mut x: TreeMap_Entry<Object, Object> = _t12;
                } else {
                    let _t8: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t9: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::rightOf(Clone::clone(&_t8))?;
                    if Object::from_any(x.clone()) == Object::from_any(_t9.clone()) {
                        let _t10: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                        let mut x: TreeMap_Entry<Object, Object> = _t10;
                        this.rotateLeft(Clone::clone(&x))?;
                    }
                    let _t10: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&_t10), (1i32 != 0i32))?;
                    let _t11: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t12: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t11))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&_t12), (0i32 != 0i32))?;
                    let _t13: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t14: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t13))?;
                    this.rotateRight(Clone::clone(&_t14))?;
                }
                continue;
                let _t8: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                let _t9: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t8))?;
                let _t10: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::leftOf(Clone::clone(&_t9))?;
                y = _t10;
                let _t11: bool = TreeMap::<Object, Object>::colorOf(Clone::clone(&y))?;
                if !(_t11) {
                    let _t12: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&_t12), (1i32 != 0i32))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&y), (1i32 != 0i32))?;
                    let _t13: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t14: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t13))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&_t14), (0i32 != 0i32))?;
                    let _t15: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t16: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t15))?;
                    x = _t16;
                } else {
                    let _t12: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t13: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::leftOf(Clone::clone(&_t12))?;
                    if Object::from_any(x.clone()) == Object::from_any(_t13.clone()) {
                        let _t14: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                        x = _t14;
                        this.rotateRight(Clone::clone(&x))?;
                    }
                    let _t14: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&_t14), (1i32 != 0i32))?;
                    let _t15: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t16: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t15))?;
                    TreeMap::<Object, Object>::setColor(Clone::clone(&_t16), (0i32 != 0i32))?;
                    let _t17: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&x))?;
                    let _t18: TreeMap_Entry<Object, Object> = TreeMap::<Object, Object>::parentOf(Clone::clone(&_t17))?;
                    this.rotateLeft(Clone::clone(&_t18))?;
                }
            }
            this.__get_root().__set_color((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "deleteEntry", descriptor = "(Ljava/util/TreeMap$Entry;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/TreeMap$Entry<TK;TV;>;)V")]
        pub fn deleteEntry(&self, p: TreeMap_Entry<K, V>) -> Result<()> {
            panic!("stub: java/util/TreeMap.deleteEntry:(Ljava/util/TreeMap$Entry;)V")
        }

        #[java_method(name = "fixAfterDeletion", descriptor = "(Ljava/util/TreeMap$Entry;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/TreeMap$Entry<TK;TV;>;)V")]
        pub fn fixAfterDeletion(&self, x: TreeMap_Entry<K, V>) -> Result<()> {
            panic!("stub: java/util/TreeMap.fixAfterDeletion:(Ljava/util/TreeMap$Entry;)V")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/TreeMap.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/TreeMap.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "readTreeSet", descriptor = "(ILjava/io/ObjectInputStream;Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException", generic_signature = "(ILjava/io/ObjectInputStream;TV;)V")]
        pub fn readTreeSet(&self, size: i32, s: Object, defaultVal: V) -> Result<()> {
            panic!("stub: java/util/TreeMap.readTreeSet:(ILjava/io/ObjectInputStream;Ljava/lang/Object;)V")
        }

        #[java_method(name = "addAllForTreeSet", descriptor = "(Ljava/util/SortedSet;Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/SortedSet<+TK;>;TV;)V")]
        pub fn addAllForTreeSet(&self, set: Object, defaultVal: V) -> Result<()> {
            panic!("stub: java/util/TreeMap.addAllForTreeSet:(Ljava/util/SortedSet;Ljava/lang/Object;)V")
        }

        #[java_method(name = "buildFromSorted", descriptor = "(ILjava/util/Iterator;Ljava/io/ObjectInputStream;Ljava/lang/Object;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException", generic_signature = "(ILjava/util/Iterator<*>;Ljava/io/ObjectInputStream;TV;)V")]
        pub fn buildFromSorted_i_iterat_object_obj(&self, size: i32, it: Object, str: Object, defaultVal: V) -> Result<()> {
            panic!("stub: java/util/TreeMap.buildFromSorted:(ILjava/util/Iterator;Ljava/io/ObjectInputStream;Ljava/lang/Object;)V")
        }

        #[java_method(name = "buildFromSorted", descriptor = "(IIIILjava/util/Iterator;Ljava/io/ObjectInputStream;Ljava/lang/Object;)Ljava/util/TreeMap$Entry;", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException", generic_signature = "(IIIILjava/util/Iterator<*>;Ljava/io/ObjectInputStream;TV;)Ljava/util/TreeMap$Entry<TK;TV;>;")]
        pub fn buildFromSorted_i_i_i_i_iterat_object_obj(&self, level: i32, lo: i32, hi: i32, redLevel: i32, it: Object, str: Object, defaultVal: V) -> Result<TreeMap_Entry<Object, Object>> {
            panic!("stub: java/util/TreeMap.buildFromSorted:(IIIILjava/util/Iterator;Ljava/io/ObjectInputStream;Ljava/lang/Object;)Ljava/util/TreeMap$Entry;")
        }

        #[java_method(name = "computeRedLevel", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeRedLevel(size: i32) -> Result<i32> {
            panic!("stub: java/util/TreeMap.computeRedLevel:(I)I")
        }

        #[java_method(name = "keySpliteratorFor", descriptor = "(Ljava/util/NavigableMap;)Ljava/util/Spliterator;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;>(Ljava/util/NavigableMap<TK;*>;)Ljava/util/Spliterator<TK;>;")]
        pub fn keySpliteratorFor(m: Object) -> Result<Object> {
            panic!("stub: java/util/TreeMap.keySpliteratorFor:(Ljava/util/NavigableMap;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "keySpliterator", descriptor = "()Ljava/util/Spliterator;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TK;>;")]
        pub fn keySpliterator(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.keySpliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "descendingKeySpliterator", descriptor = "()Ljava/util/Spliterator;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TK;>;")]
        pub fn descendingKeySpliterator(&self) -> Result<Object> {
            panic!("stub: java/util/TreeMap.descendingKeySpliterator:()Ljava/util/Spliterator;")
        }
    }
}
