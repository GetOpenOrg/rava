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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<Hashtable<K, V>> for Dictionary<K, V> {
    fn from(v: Hashtable<K, V>) -> Dictionary<K, V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Hashtable"]
    #[super_class       = "java/util/Dictionary"]
    #[interfaces        = "java/util/Map,java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/Dictionary<TK;TV;>;Ljava/util/Map<TK;TV;>;Ljava/lang/Cloneable;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Hashtable.java"]
    #[inner_classes     = "java/util/Hashtable$Entry:java/util/Hashtable:Entry:10;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/Hashtable$Enumerator:java/util/Hashtable:Enumerator:2;java/util/Hashtable$KeySet:java/util/Hashtable:KeySet:2;java/util/Hashtable$EntrySet:java/util/Hashtable:EntrySet:2;java/util/Hashtable$ValueCollection:java/util/Hashtable:ValueCollection:2;java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/util/Hashtable$UnsafeHolder:java/util/Hashtable:UnsafeHolder:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Dictionary<K, V>"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/Dictionary;java/util/Hashtable;java/util/Map"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Hashtable<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/util/Hashtable$Entry;", access = "private", modifiers = "transient", is_static = false, generic_signature = "[Ljava/util/Hashtable$Entry<**>;"))]
        pub table: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "count", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub count: i32,
        #[cfg_attr(any(), java_field(name = "threshold", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub threshold: i32,
        #[cfg_attr(any(), java_field(name = "loadFactor", descriptor = "F", access = "private", modifiers = "", is_static = false))]
        pub loadFactor: f32,
        #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub modCount: i32,
        #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/util/Set<TK;>;"))]
        pub keySet: Object,
        #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;"))]
        pub entrySet: Object,
        #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/util/Collection<TV;>;"))]
        pub values: Object,
    }

    impl<K, V> Hashtable<K, V> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "1421746759512286392"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            1421746759512286392i64
        }

        #[cfg_attr(any(), java_field(name = "MAX_ARRAY_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2147483639"))]
        // static field: MAX_ARRAY_SIZE:I
        pub fn MAX_ARRAY_SIZE() -> i32 {
            2147483639
        }

        #[cfg_attr(any(), java_field(name = "KEYS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: KEYS:I
        pub fn KEYS() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "VALUES", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: VALUES:I
        pub fn VALUES() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "ENTRIES", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: ENTRIES:I
        pub fn ENTRIES() -> i32 {
            2
        }

        #[java_method(name = "<init>", descriptor = "(IF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_f(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
            panic!("stub: java/util/Hashtable.<init>:(IF)V")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(initialCapacity: i32) -> Result<Self> {
            panic!("stub: java/util/Hashtable.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Hashtable.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn new_map(t: Object) -> Result<Self> {
            panic!("stub: java/util/Hashtable.<init>:(Ljava/util/Map;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Void;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_void(dummy: Object) -> Result<Self> {
            panic!("stub: java/util/Hashtable.<init>:(Ljava/lang/Void;)V")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/Hashtable.size:()I")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/Hashtable.isEmpty:()Z")
        }

        #[java_method(name = "keys", descriptor = "()Ljava/util/Enumeration;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Enumeration<TK;>;")]
        pub fn keys(&self) -> Result<Object> {
            panic!("stub: java/util/Hashtable.keys:()Ljava/util/Enumeration;")
        }

        #[java_method(name = "elements", descriptor = "()Ljava/util/Enumeration;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Enumeration<TV;>;")]
        pub fn elements(&self) -> Result<Object> {
            panic!("stub: java/util/Hashtable.elements:()Ljava/util/Enumeration;")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/Hashtable.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/Hashtable.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsKey(&self, key: Object) -> Result<bool> {
            panic!("stub: java/util/Hashtable.containsKey:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn get(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/Hashtable.get:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "rehash", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rehash(&self) -> Result<()> {
            panic!("stub: java/util/Hashtable.rehash:()V")
        }

        #[java_method(name = "addEntry", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;I)V")]
        pub fn addEntry(&self, hash: i32, key: K, value: V, index: i32) -> Result<()> {
            panic!("stub: java/util/Hashtable.addEntry:(ILjava/lang/Object;Ljava/lang/Object;I)V")
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn put(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/Hashtable.put:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn remove_obj(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/Hashtable.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn putAll(&self, t: Object) -> Result<()> {
            panic!("stub: java/util/Hashtable.putAll:(Ljava/util/Map;)V")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/Hashtable.clear:()V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/Hashtable.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "cloneHashtable", descriptor = "()Ljava/util/Hashtable;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Hashtable<**>;")]
        pub fn cloneHashtable(&self) -> Result<Hashtable<Object, Object>> {
            panic!("stub: java/util/Hashtable.cloneHashtable:()Ljava/util/Hashtable;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "getEnumeration", descriptor = "(I)Ljava/util/Enumeration;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(I)Ljava/util/Enumeration<TT;>;")]
        pub fn getEnumeration(&self, type_: i32) -> Result<Object> {
            panic!("stub: java/util/Hashtable.getEnumeration:(I)Ljava/util/Enumeration;")
        }

        #[java_method(name = "getIterator", descriptor = "(I)Ljava/util/Iterator;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(I)Ljava/util/Iterator<TT;>;")]
        pub fn getIterator(&self, type_: i32) -> Result<Object> {
            panic!("stub: java/util/Hashtable.getIterator:(I)Ljava/util/Iterator;")
        }

        #[java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<TK;>;")]
        pub fn keySet(&self) -> Result<Object> {
            panic!("stub: java/util/Hashtable.keySet:()Ljava/util/Set;")
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            panic!("stub: java/util/Hashtable.entrySet:()Ljava/util/Set;")
        }

        #[java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;")]
        pub fn values(&self) -> Result<Object> {
            panic!("stub: java/util/Hashtable.values:()Ljava/util/Collection;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/Hashtable.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;TV;)TV;")]
        pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<Object> {
            panic!("stub: java/util/Hashtable.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TK;-TV;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/Hashtable.forEach:(Ljava/util/function/BiConsumer;)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)V")]
        pub fn replaceAll(&self, function: Object) -> Result<()> {
            panic!("stub: java/util/Hashtable.replaceAll:(Ljava/util/function/BiFunction;)V")
        }

        #[java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putIfAbsent(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/Hashtable.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove_obj_obj(&self, key: Object, value: Object) -> Result<bool> {
            panic!("stub: java/util/Hashtable.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;TV;)Z")]
        pub fn replace_obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
            panic!("stub: java/util/Hashtable.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn replace_obj_obj(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/Hashtable.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/Function<-TK;+TV;>;)TV;")]
        pub fn computeIfAbsent(&self, key: K, mappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/Hashtable.computeIfAbsent:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/Hashtable.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn compute(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/Hashtable.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;")]
        pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/Hashtable.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Hashtable.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "writeHashtable", descriptor = "(Ljava/io/ObjectOutputStream;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeHashtable(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Hashtable.writeHashtable:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "defaultWriteHashtable", descriptor = "(Ljava/io/ObjectOutputStream;IF)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn defaultWriteHashtable(&self, s: Object, length: i32, loadFactor: f32) -> Result<()> {
            panic!("stub: java/util/Hashtable.defaultWriteHashtable:(Ljava/io/ObjectOutputStream;IF)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Hashtable.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "readHashtable", descriptor = "(Ljava/io/ObjectInputStream;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readHashtable(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Hashtable.readHashtable:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "reconstitutionPut", descriptor = "([Ljava/util/Hashtable$Entry;Ljava/lang/Object;Ljava/lang/Object;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/StreamCorruptedException", generic_signature = "([Ljava/util/Hashtable$Entry<**>;TK;TV;)V")]
        pub fn reconstitutionPut(&self, tab: Rc<RefCell<Vec<Object>>>, key: K, value: V) -> Result<()> {
            panic!("stub: java/util/Hashtable.reconstitutionPut:([Ljava/util/Hashtable$Entry;Ljava/lang/Object;Ljava/lang/Object;)V")
        }
    }
}
