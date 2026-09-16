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
use crate::jdk::internal::misc::Unsafe;

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<ConcurrentHashMap<K, V>> for AbstractMap<K, V> {
    fn from(v: ConcurrentHashMap<K, V>) -> AbstractMap<K, V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/ConcurrentHashMap"]
    #[super_class       = "java/util/AbstractMap"]
    #[interfaces        = "java/util/concurrent/ConcurrentMap,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/AbstractMap<TK;TV;>;Ljava/util/concurrent/ConcurrentMap<TK;TV;>;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ConcurrentHashMap.java"]
    #[inner_classes     = "java/util/concurrent/ConcurrentHashMap$Node:java/util/concurrent/ConcurrentHashMap:Node:8;java/util/concurrent/ConcurrentHashMap$Traverser:java/util/concurrent/ConcurrentHashMap:Traverser:8;java/util/concurrent/ConcurrentHashMap$TreeBin:java/util/concurrent/ConcurrentHashMap:TreeBin:24;java/util/concurrent/ConcurrentHashMap$TreeNode:java/util/concurrent/ConcurrentHashMap:TreeNode:24;java/util/concurrent/ConcurrentHashMap$ReservationNode:java/util/concurrent/ConcurrentHashMap:ReservationNode:24;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/concurrent/ConcurrentHashMap$KeySetView:java/util/concurrent/ConcurrentHashMap:KeySetView:25;java/util/concurrent/ConcurrentHashMap$ValuesView:java/util/concurrent/ConcurrentHashMap:ValuesView:24;java/util/concurrent/ConcurrentHashMap$EntrySetView:java/util/concurrent/ConcurrentHashMap:EntrySetView:24;java/util/concurrent/ConcurrentHashMap$Segment:java/util/concurrent/ConcurrentHashMap:Segment:8;java/io/ObjectOutputStream$PutField:java/io/ObjectOutputStream:PutField:1033;java/util/AbstractMap$SimpleImmutableEntry:java/util/AbstractMap:SimpleImmutableEntry:9;java/util/concurrent/ConcurrentHashMap$KeyIterator:java/util/concurrent/ConcurrentHashMap:KeyIterator:24;java/util/concurrent/ConcurrentHashMap$ValueIterator:java/util/concurrent/ConcurrentHashMap:ValueIterator:24;java/util/concurrent/ConcurrentHashMap$CounterCell:java/util/concurrent/ConcurrentHashMap:CounterCell:24;java/util/concurrent/ConcurrentHashMap$ForwardingNode:java/util/concurrent/ConcurrentHashMap:ForwardingNode:24;java/util/concurrent/ConcurrentHashMap$ForEachMappingTask:java/util/concurrent/ConcurrentHashMap:ForEachMappingTask:24;java/util/concurrent/ConcurrentHashMap$BulkTask:java/util/concurrent/ConcurrentHashMap:BulkTask:1032;java/util/concurrent/ConcurrentHashMap$ForEachTransformedMappingTask:java/util/concurrent/ConcurrentHashMap:ForEachTransformedMappingTask:24;java/util/concurrent/ConcurrentHashMap$SearchMappingsTask:java/util/concurrent/ConcurrentHashMap:SearchMappingsTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceMappingsTask:java/util/concurrent/ConcurrentHashMap:MapReduceMappingsTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToDoubleTask:java/util/concurrent/ConcurrentHashMap:MapReduceMappingsToDoubleTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToLongTask:java/util/concurrent/ConcurrentHashMap:MapReduceMappingsToLongTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToIntTask:java/util/concurrent/ConcurrentHashMap:MapReduceMappingsToIntTask:24;java/util/concurrent/ConcurrentHashMap$ForEachKeyTask:java/util/concurrent/ConcurrentHashMap:ForEachKeyTask:24;java/util/concurrent/ConcurrentHashMap$ForEachTransformedKeyTask:java/util/concurrent/ConcurrentHashMap:ForEachTransformedKeyTask:24;java/util/concurrent/ConcurrentHashMap$SearchKeysTask:java/util/concurrent/ConcurrentHashMap:SearchKeysTask:24;java/util/concurrent/ConcurrentHashMap$ReduceKeysTask:java/util/concurrent/ConcurrentHashMap:ReduceKeysTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceKeysTask:java/util/concurrent/ConcurrentHashMap:MapReduceKeysTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceKeysToDoubleTask:java/util/concurrent/ConcurrentHashMap:MapReduceKeysToDoubleTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceKeysToLongTask:java/util/concurrent/ConcurrentHashMap:MapReduceKeysToLongTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceKeysToIntTask:java/util/concurrent/ConcurrentHashMap:MapReduceKeysToIntTask:24;java/util/concurrent/ConcurrentHashMap$ForEachValueTask:java/util/concurrent/ConcurrentHashMap:ForEachValueTask:24;java/util/concurrent/ConcurrentHashMap$ForEachTransformedValueTask:java/util/concurrent/ConcurrentHashMap:ForEachTransformedValueTask:24;java/util/concurrent/ConcurrentHashMap$SearchValuesTask:java/util/concurrent/ConcurrentHashMap:SearchValuesTask:24;java/util/concurrent/ConcurrentHashMap$ReduceValuesTask:java/util/concurrent/ConcurrentHashMap:ReduceValuesTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceValuesTask:java/util/concurrent/ConcurrentHashMap:MapReduceValuesTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceValuesToDoubleTask:java/util/concurrent/ConcurrentHashMap:MapReduceValuesToDoubleTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceValuesToLongTask:java/util/concurrent/ConcurrentHashMap:MapReduceValuesToLongTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceValuesToIntTask:java/util/concurrent/ConcurrentHashMap:MapReduceValuesToIntTask:24;java/util/concurrent/ConcurrentHashMap$ForEachEntryTask:java/util/concurrent/ConcurrentHashMap:ForEachEntryTask:24;java/util/concurrent/ConcurrentHashMap$ForEachTransformedEntryTask:java/util/concurrent/ConcurrentHashMap:ForEachTransformedEntryTask:24;java/util/concurrent/ConcurrentHashMap$SearchEntriesTask:java/util/concurrent/ConcurrentHashMap:SearchEntriesTask:24;java/util/concurrent/ConcurrentHashMap$ReduceEntriesTask:java/util/concurrent/ConcurrentHashMap:ReduceEntriesTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceEntriesTask:java/util/concurrent/ConcurrentHashMap:MapReduceEntriesTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToDoubleTask:java/util/concurrent/ConcurrentHashMap:MapReduceEntriesToDoubleTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToLongTask:java/util/concurrent/ConcurrentHashMap:MapReduceEntriesToLongTask:24;java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToIntTask:java/util/concurrent/ConcurrentHashMap:MapReduceEntriesToIntTask:24;java/util/concurrent/ConcurrentHashMap$CollectionView:java/util/concurrent/ConcurrentHashMap:CollectionView:1032;java/util/concurrent/ConcurrentHashMap$EntrySpliterator:java/util/concurrent/ConcurrentHashMap:EntrySpliterator:24;java/util/concurrent/ConcurrentHashMap$ValueSpliterator:java/util/concurrent/ConcurrentHashMap:ValueSpliterator:24;java/util/concurrent/ConcurrentHashMap$KeySpliterator:java/util/concurrent/ConcurrentHashMap:KeySpliterator:24;java/util/concurrent/ConcurrentHashMap$MapEntry:java/util/concurrent/ConcurrentHashMap:MapEntry:24;java/util/concurrent/ConcurrentHashMap$EntryIterator:java/util/concurrent/ConcurrentHashMap:EntryIterator:24;java/util/concurrent/ConcurrentHashMap$BaseIterator:java/util/concurrent/ConcurrentHashMap:BaseIterator:8;java/util/concurrent/ConcurrentHashMap$TableStack:java/util/concurrent/ConcurrentHashMap:TableStack:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractMap<K, V>"]
    #[superclass_fields(keySet: Object, values: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/AbstractMap;java/util/Map;java/util/concurrent/ConcurrentHashMap;java/util/concurrent/ConcurrentMap"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ConcurrentHashMap<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "package", modifiers = "volatile transient", is_static = false, generic_signature = "[Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;"))]
        pub table: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>,
        #[cfg_attr(any(), java_field(name = "nextTable", descriptor = "[Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "[Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;"))]
        pub nextTable: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>,
        #[cfg_attr(any(), java_field(name = "baseCount", descriptor = "J", access = "private", modifiers = "volatile transient", is_static = false))]
        pub baseCount: i64,
        #[cfg_attr(any(), java_field(name = "sizeCtl", descriptor = "I", access = "private", modifiers = "volatile transient", is_static = false))]
        pub sizeCtl: i32,
        #[cfg_attr(any(), java_field(name = "transferIndex", descriptor = "I", access = "private", modifiers = "volatile transient", is_static = false))]
        pub transferIndex: i32,
        #[cfg_attr(any(), java_field(name = "cellsBusy", descriptor = "I", access = "private", modifiers = "volatile transient", is_static = false))]
        pub cellsBusy: i32,
        #[cfg_attr(any(), java_field(name = "counterCells", descriptor = "[Ljava/util/concurrent/ConcurrentHashMap$CounterCell;", access = "private", modifiers = "volatile transient", is_static = false))]
        pub counterCells: Rc<RefCell<Vec<ConcurrentHashMap_CounterCell>>>,
        #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$KeySetView;", access = "private", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$KeySetView<TK;TV;>;"))]
        pub keySet: Object,
        #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$ValuesView;", access = "private", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$ValuesView<TK;TV;>;"))]
        pub values: Object,
        #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$EntrySetView;", access = "private", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$EntrySetView<TK;TV;>;"))]
        pub entrySet: Object,
    }

    impl<K, V> ConcurrentHashMap<K, V> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "7249069246763182397"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            7249069246763182397i64
        }

        #[cfg_attr(any(), java_field(name = "MAXIMUM_CAPACITY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1073741824"))]
        // static field: MAXIMUM_CAPACITY:I
        pub fn MAXIMUM_CAPACITY() -> i32 {
            1073741824
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_CAPACITY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: DEFAULT_CAPACITY:I
        pub fn DEFAULT_CAPACITY() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "MAX_ARRAY_SIZE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2147483639"))]
        // static field: MAX_ARRAY_SIZE:I
        pub fn MAX_ARRAY_SIZE() -> i32 {
            2147483639
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_CONCURRENCY_LEVEL", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: DEFAULT_CONCURRENCY_LEVEL:I
        pub fn DEFAULT_CONCURRENCY_LEVEL() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "LOAD_FACTOR", descriptor = "F", access = "private", modifiers = "static final", is_static = true, constant_value = "0.75"))]
        // static field: LOAD_FACTOR:F
        pub fn LOAD_FACTOR() -> f32 {
            0.75f32
        }

        #[cfg_attr(any(), java_field(name = "TREEIFY_THRESHOLD", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: TREEIFY_THRESHOLD:I
        pub fn TREEIFY_THRESHOLD() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "UNTREEIFY_THRESHOLD", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: UNTREEIFY_THRESHOLD:I
        pub fn UNTREEIFY_THRESHOLD() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "MIN_TREEIFY_CAPACITY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "64"))]
        // static field: MIN_TREEIFY_CAPACITY:I
        pub fn MIN_TREEIFY_CAPACITY() -> i32 {
            64
        }

        #[cfg_attr(any(), java_field(name = "MIN_TRANSFER_STRIDE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: MIN_TRANSFER_STRIDE:I
        pub fn MIN_TRANSFER_STRIDE() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "RESIZE_STAMP_BITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: RESIZE_STAMP_BITS:I
        pub fn RESIZE_STAMP_BITS() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "MAX_RESIZERS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "65535"))]
        // static field: MAX_RESIZERS:I
        pub fn MAX_RESIZERS() -> i32 {
            65535
        }

        #[cfg_attr(any(), java_field(name = "RESIZE_STAMP_SHIFT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: RESIZE_STAMP_SHIFT:I
        pub fn RESIZE_STAMP_SHIFT() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "MOVED", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-1"))]
        // static field: MOVED:I
        pub fn MOVED() -> i32 {
            -1
        }

        #[cfg_attr(any(), java_field(name = "TREEBIN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-2"))]
        // static field: TREEBIN:I
        pub fn TREEBIN() -> i32 {
            -2
        }

        #[cfg_attr(any(), java_field(name = "RESERVED", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-3"))]
        // static field: RESERVED:I
        pub fn RESERVED() -> i32 {
            -3
        }

        #[cfg_attr(any(), java_field(name = "HASH_BITS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2147483647"))]
        // static field: HASH_BITS:I
        pub fn HASH_BITS() -> i32 {
            2147483647
        }

        #[cfg_attr(any(), java_field(name = "NCPU", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: NCPU:I
        pub fn NCPU() -> i32 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.NCPU:I")
        }

        #[cfg_attr(any(), java_field(name = "serialPersistentFields", descriptor = "[Ljava/io/ObjectStreamField;", access = "private", modifiers = "static final", is_static = true))]
        // static field: serialPersistentFields:[Ljava/io/ObjectStreamField;
        pub fn serialPersistentFields() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.serialPersistentFields:[Ljava/io/ObjectStreamField;")
        }

        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "SIZECTL", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: SIZECTL:J
        pub fn SIZECTL() -> i64 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.SIZECTL:J")
        }

        #[cfg_attr(any(), java_field(name = "TRANSFERINDEX", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: TRANSFERINDEX:J
        pub fn TRANSFERINDEX() -> i64 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.TRANSFERINDEX:J")
        }

        #[cfg_attr(any(), java_field(name = "BASECOUNT", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: BASECOUNT:J
        pub fn BASECOUNT() -> i64 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.BASECOUNT:J")
        }

        #[cfg_attr(any(), java_field(name = "CELLSBUSY", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: CELLSBUSY:J
        pub fn CELLSBUSY() -> i64 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.CELLSBUSY:J")
        }

        #[cfg_attr(any(), java_field(name = "CELLVALUE", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: CELLVALUE:J
        pub fn CELLVALUE() -> i64 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.CELLVALUE:J")
        }

        #[cfg_attr(any(), java_field(name = "ABASE", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: ABASE:I
        pub fn ABASE() -> i32 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.ABASE:I")
        }

        #[cfg_attr(any(), java_field(name = "ASHIFT", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: ASHIFT:I
        pub fn ASHIFT() -> i32 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.ASHIFT:I")
        }

        #[java_method(name = "spread", descriptor = "(I)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spread(mut h: i32) -> Result<i32> {
            Ok(((h^((h as u32>>(16i32&0x1f)) as i32))&2147483647i32))
        }

        #[java_method(name = "tableSizeFor", descriptor = "(I)I", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tableSizeFor(mut c: i32) -> Result<i32> {
            let _t0: i32 = Integer::numberOfLeadingZeros((c).wrapping_sub(1i32))?;
            let mut n = ((-1i32 as u32>>(_t0&0x1f)) as i32);
            Ok((if (n<0) { 1i32 } else { (if n >= 1073741824i32 { 1073741824i32 } else { (n).wrapping_add(1i32) }) }))
        }

        #[java_method(name = "comparableClassFor", descriptor = "(Ljava/lang/Object;)Ljava/lang/Class;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)Ljava/lang/Class<*>;")]
        pub fn comparableClassFor(mut x: Object) -> Result<Class<Object>> {
            let _vdispatch0: Object = if let Some(__f) = x.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut c = (_vdispatch0).downcast::<Class<Object>>();
            if c == Object::default() {
                return Ok(c);
            }
            let _t1 = c.getGenericInterfaces()?;
            let mut ts: Rc<RefCell<Vec<Object>>> = _t1;
            let mut local_5: Rc<RefCell<Vec<Object>>> = ts;
            let mut local_6 = (local_5.borrow().len() as i32);
            let mut local_7: i32 = 0i32;
            loop {
                if local_7 >= local_6 { break; }
                let mut t = Clone::clone(&local_5.borrow()[local_7 as usize]);
                let mut p: Object = t;
                let _vdispatch2: Object = if let Some(__f) = p.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _vdispatch3: Rc<RefCell<Vec<Object>>> = if let Some(__f) = p.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Rc<RefCell<Vec<Object>>>>>>() { (__f)()? } else { Default::default() };
                let mut as_: Rc<RefCell<Vec<Object>>> = _vdispatch3;
                if Clone::clone(&as_.borrow()[0i32 as usize]) == Object::from_any(c.clone()) {
                    return Ok(c);
                }
                local_7 = local_7.wrapping_add(1i32);
            }
            Ok(Default::default())
        }

        #[java_method(name = "compareComparables", descriptor = "(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Object;Ljava/lang/Object;)I")]
        pub fn compareComparables(mut kc: Class<Object>, mut k: Object, mut x: Object) -> Result<i32> {
            let _vdispatch0: Object = if let Some(__f) = x.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut _merged2: i32;
            if _vdispatch0 != Object::from_any(kc.clone()) {
                _merged2 = 0i32;
            } else {
                let _vdispatch1: i32 = if let Some(_d) = k.0.as_any().downcast_ref::<BuddhistCalendar>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<UTF_8>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<GregorianCalendar>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<JapaneseImperialCalendar>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<HeapByteBuffer>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Unicode>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<US_ASCII>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<ISO_8859_1>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Collector_Characteristics>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Locale_Category>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Normalizer_Form>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<StreamShape>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<MatchOps_MatchKind>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Pattern_Qtype>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Formatter_BigDecimalLayoutForm>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<DayOfWeek>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<RoundingMode>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<ZoneOffsetTransitionRule_TimeDefinition>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Month>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<ChronoField>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Character_UnicodeScript>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<StreamOpFlag>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<TestSwitchEnum_Season>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<TestEnumMethods_Planet>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<TestEnumBasic_Day>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<LocalTime>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<ZoneOffsetTransition>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Instant>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<ZoneOffset>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<BigDecimal>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<BigInteger>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Short>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Byte>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Date>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Calendar>() { _d.compareTo_obj(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<CharBuffer>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<ByteBuffer>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Charset>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Float>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<i64>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<bool>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<f64>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<TestComparable_Version>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<TestComparable_Student>() { _d.compareTo(Clone::clone(&x))? } else if let Some(_d) = k.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&x))? } else if let Some(__f) = k.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&x))? } else { Default::default() };
                _merged2 = _vdispatch1;
            }
            Ok(_merged2)
        }

        #[java_method(name = "tabAt", descriptor = "([Ljava/util/concurrent/ConcurrentHashMap$Node;I)Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>([Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;I)Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;")]
        pub fn tabAt(mut tab: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>, mut i: i32) -> Result<ConcurrentHashMap_Node<K, V>> {
            let _t0 = ConcurrentHashMap::<Object, Object>::U().getReferenceAcquire(Object::from_any(tab.clone()), (((i as i64)).wrapping_shl((ConcurrentHashMap::<Object, Object>::ASHIFT()&0x3f) as u32)).wrapping_add((ConcurrentHashMap::<Object, Object>::ABASE() as i64)))?;
            Ok(Default::default())
        }

        #[java_method(name = "casTabAt", descriptor = "([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)Z", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>([Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;ILjava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;)Z")]
        pub fn casTabAt(mut tab: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>, mut i: i32, mut c: ConcurrentHashMap_Node<K, V>, mut v: ConcurrentHashMap_Node<K, V>) -> Result<bool> {
            let _t0 = ConcurrentHashMap::<Object, Object>::U().compareAndSetReference(Object::from_any(tab.clone()), (((i as i64)).wrapping_shl((ConcurrentHashMap::<Object, Object>::ASHIFT()&0x3f) as u32)).wrapping_add((ConcurrentHashMap::<Object, Object>::ABASE() as i64)), Object::from_any(c.clone()), Object::from_any(v.clone()))?;
            Ok(_t0)
        }

        #[java_method(name = "setTabAt", descriptor = "([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;)V", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>([Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;ILjava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;)V")]
        pub fn setTabAt(mut tab: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>, mut i: i32, mut v: ConcurrentHashMap_Node<K, V>) -> Result<()> {
            ConcurrentHashMap::<Object, Object>::U().putReferenceRelease(Object::from_any(tab.clone()), (((i as i64)).wrapping_shl((ConcurrentHashMap::<Object, Object>::ASHIFT()&0x3f) as u32)).wrapping_add((ConcurrentHashMap::<Object, Object>::ABASE() as i64)), Object::from_any(v.clone()))?;
            Ok(())
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(initialCapacity: i32) -> Result<Self> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn new_map(m: Object) -> Result<Self> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.<init>:(Ljava/util/Map;)V")
        }

        #[java_method(name = "<init>", descriptor = "(IF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_f(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.<init>:(IF)V")
        }

        #[java_method(name = "<init>", descriptor = "(IFI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_f_i(initialCapacity: i32, loadFactor: f32, concurrencyLevel: i32) -> Result<Self> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.<init>:(IFI)V")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.size:()I")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.isEmpty:()Z")
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn get(&self, mut key: Object) -> Result<V> {
            let this = self;
            let _vdispatch0: i32 = if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let _t1: i32 = ConcurrentHashMap::<Object, Object>::spread(_vdispatch0)?;
            let mut h: i32 = _t1;
            let mut tab = this.__get_table();
            let mut n = (tab.borrow().len() as i32);
            let _t2: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::tabAt(Default::default(), ((n).wrapping_sub(1i32)&h))?;
            let mut e: ConcurrentHashMap_Node<Object, Object> = _t2;
            let mut eh = e.__get_hash();
            let mut ek = e.__get_key();
            let _vdispatch3: bool = if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&ek))? } else { Default::default() };
            return Ok(panic!("null"));
            let _t4 = e.find(h, Clone::clone(&key))?;
            let mut p: ConcurrentHashMap_Node<Object, Object> = _t4;
            return Ok(panic!("null"));
            loop {
                e = e.__get_next();
                ek = e.__get_key();
                let _vdispatch5: bool = if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&ek))? } else { Default::default() };
                if _vdispatch5 { break; }
            }
            return Ok(panic!("null"));
            Ok(panic!("null"))
        }

        #[java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsKey(&self, key: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.containsKey:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn put(&self, mut key: K, mut value: V) -> Result<V> {
            let this = self;
            let _t0 = this.putVal(Clone::clone(&key), Clone::clone(&value), (0i32 != 0i32))?;
            Ok(panic!("null"))
        }

        #[java_method(name = "putVal", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Z)Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Z)TV;")]
        pub fn putVal(&self, mut key: K, mut value: V, mut onlyIfAbsent: bool) -> Result<V> {
            let this = self;
            if _is_jnull(&value) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = key.hashCode()?;
            let _t1: i32 = ConcurrentHashMap::<Object, Object>::spread(_t0)?;
            let mut hash: i32 = _t1;
            let mut binCount: i32 = 0i32;
            let mut tab = this.__get_table();
            let mut oldVal: V = Default::default();
            loop {
                let mut n = (tab.borrow().len() as i32);
                if ((tab.borrow().len() as i32)==0) {
                    let _t2 = this.initTable()?;
                    tab = _t2;
                } else {
                    let mut i = ((n).wrapping_sub(1i32)&hash);
                    let _t2: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::tabAt(Default::default(), ((n).wrapping_sub(1i32)&hash))?;
                    let mut f: ConcurrentHashMap_Node<Object, Object> = _t2;
                    if _is_jnull(&f) {
                        let _t3: bool = ConcurrentHashMap::<Object, Object>::casTabAt(Default::default(), i, Default::default(), Clone::clone(&ConcurrentHashMap_Node::<Object, Object>::new_i_obj_obj(hash, Clone::clone(&key), Clone::clone(&value))?))?;
                    } else {
                        let mut fh = f.__get_hash();
                        if f.__get_hash() == -1i32 {
                            let _t3 = this.helpTransfer(Default::default(), Clone::clone(&f))?;
                            tab = _t3;
                        } else {
                            let mut fk = f.__get_key();
                            let _t3 = key.equals(Clone::clone(&fk))?;
                            let mut fv = f.__get_val();
                            if !_is_jnull(&f.__get_val()) {
                                return Ok(fv);
                            }
                            oldVal = Default::default();
                            let mut local_14: ConcurrentHashMap_Node<Object, Object> = f;
                            let _t4: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::tabAt(Default::default(), i)?;
                            if (fh>=0) {
                                binCount = 1i32;
                                let mut e: ConcurrentHashMap_Node<Object, Object> = f;
                                loop {
                                    if e.__get_hash() == hash {
                                        let mut ek = e.__get_key();
                                        if !_is_jnull(&ek) {
                                            let _t5 = key.equals(Clone::clone(&ek))?;
                                            if _t5 {
                                                oldVal = e.__get_val();
                                                e.__set_val(Clone::clone(&value));
                                            } else {
                                                let mut pred: ConcurrentHashMap_Node<Object, Object> = e;
                                                e = e.__get_next();
                                                if _is_jnull(&e.__get_next()) {
                                                    pred.__set_next(ConcurrentHashMap_Node::<Object, Object>::new_i_obj_obj(hash, Clone::clone(&key), Clone::clone(&value))?);
                                                    break;
                                                }
                                                binCount = binCount.wrapping_add(1i32);
                                                continue;
                                            }
                                        } else {
                                            let mut pred: ConcurrentHashMap_Node<Object, Object> = e;
                                            e = e.__get_next();
                                            if _is_jnull(&e.__get_next()) {
                                                pred.__set_next(ConcurrentHashMap_Node::<Object, Object>::new_i_obj_obj(hash, Clone::clone(&key), Clone::clone(&value))?);
                                                break;
                                            }
                                            binCount = binCount.wrapping_add(1i32);
                                            continue;
                                        }
                                    } else {
                                        let mut pred: ConcurrentHashMap_Node<Object, Object> = e;
                                        e = e.__get_next();
                                        if _is_jnull(&e.__get_next()) {
                                            pred.__set_next(ConcurrentHashMap_Node::<Object, Object>::new_i_obj_obj(hash, Clone::clone(&key), Clone::clone(&value))?);
                                            break;
                                        }
                                        binCount = binCount.wrapping_add(1i32);
                                        continue;
                                    }
                                    if ((panic!("stack underflow") as i32)!=0) { break; }
                                    e.__set_val(Clone::clone(&value));
                                    break;
                                    let mut pred: ConcurrentHashMap_Node<Object, Object> = e;
                                    e = e.__get_next();
                                    if _is_jnull(&e.__get_next()) {
                                        pred.__set_next(ConcurrentHashMap_Node::<Object, Object>::new_i_obj_obj(hash, Clone::clone(&key), Clone::clone(&value))?);
                                        break;
                                    }
                                    binCount = binCount.wrapping_add(1i32);
                                }
                            } else {
                                if false {
                                    binCount = 2i32;
                                    let _t5 = Default::default().putTreeVal(hash, Clone::clone(&key), Clone::clone(&value))?;
                                    let mut e: ConcurrentHashMap_TreeNode<Object, Object> = _t5;
                                    oldVal = e.__get_val();
                                    if !(onlyIfAbsent) {
                                        e.__set_val(Clone::clone(&value));
                                    }
                                } else {
                                    if false {
                                        return Err(JvmError::Custom("athrow".to_owned()));
                                    }
                                }
                            }
                            if binCount >= 8i32 {
                                this.treeifyBin(Default::default(), i)?;
                            }
                            return Ok(oldVal);
                        }
                        continue;
                    }
                }
                if _is_jnull(&(panic!("stack underflow") as i32)) { break; }
                return Ok(oldVal);
            }
            this.addCount(1i64, binCount)?;
            Ok(panic!("null"))
        }

        #[java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn putAll(&self, m: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.putAll:(Ljava/util/Map;)V")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn remove_obj(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "replaceNode", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;TV;Ljava/lang/Object;)TV;")]
        pub fn replaceNode(&self, key: Object, value: V, cv: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.replaceNode:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.clear:()V")
        }

        #[java_method(name = "keySet", descriptor = "()Ljava/util/concurrent/ConcurrentHashMap$KeySetView;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/ConcurrentHashMap$KeySetView<TK;TV;>;")]
        pub fn keySet(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.keySet:()Ljava/util/concurrent/ConcurrentHashMap$KeySetView;")
        }

        #[java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;")]
        pub fn values(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.values:()Ljava/util/Collection;")
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.entrySet:()Ljava/util/Set;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putIfAbsent(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove_obj_obj(&self, key: Object, value: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;TV;)Z")]
        pub fn replace_obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn replace_obj_obj(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;TV;)TV;")]
        pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TK;-TV;>;)V")]
        pub fn forEach_bicons(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEach:(Ljava/util/function/BiConsumer;)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)V")]
        pub fn replaceAll(&self, function: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.replaceAll:(Ljava/util/function/BiFunction;)V")
        }

        #[java_method(name = "removeEntryIf", descriptor = "(Ljava/util/function/Predicate;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-Ljava/util/Map$Entry<TK;TV;>;>;)Z")]
        pub fn removeEntryIf(&self, function: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.removeEntryIf:(Ljava/util/function/Predicate;)Z")
        }

        #[java_method(name = "removeValueIf", descriptor = "(Ljava/util/function/Predicate;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TV;>;)Z")]
        pub fn removeValueIf(&self, function: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.removeValueIf:(Ljava/util/function/Predicate;)Z")
        }

        #[java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/Function<-TK;+TV;>;)TV;")]
        pub fn computeIfAbsent(&self, key: K, mappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.computeIfAbsent:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn compute(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;")]
        pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "keys", descriptor = "()Ljava/util/Enumeration;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Enumeration<TK;>;")]
        pub fn keys(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.keys:()Ljava/util/Enumeration;")
        }

        #[java_method(name = "elements", descriptor = "()Ljava/util/Enumeration;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Enumeration<TV;>;")]
        pub fn elements(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.elements:()Ljava/util/Enumeration;")
        }

        #[java_method(name = "mappingCount", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mappingCount(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.mappingCount:()J")
        }

        #[java_method(name = "newKeySet", descriptor = "()Ljava/util/concurrent/ConcurrentHashMap$KeySetView;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;>()Ljava/util/concurrent/ConcurrentHashMap$KeySetView<TK;Ljava/lang/Boolean;>;")]
        pub fn newKeySet() -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.newKeySet:()Ljava/util/concurrent/ConcurrentHashMap$KeySetView;")
        }

        #[java_method(name = "newKeySet", descriptor = "(I)Ljava/util/concurrent/ConcurrentHashMap$KeySetView;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;>(I)Ljava/util/concurrent/ConcurrentHashMap$KeySetView<TK;Ljava/lang/Boolean;>;")]
        pub fn newKeySet_i(initialCapacity: i32) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.newKeySet:(I)Ljava/util/concurrent/ConcurrentHashMap$KeySetView;")
        }

        #[java_method(name = "keySet", descriptor = "(Ljava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$KeySetView;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TV;)Ljava/util/concurrent/ConcurrentHashMap$KeySetView<TK;TV;>;")]
        pub fn keySet_obj(&self, mappedValue: V) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.keySet:(Ljava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$KeySetView;")
        }

        #[java_method(name = "resizeStamp", descriptor = "(I)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resizeStamp(mut n: i32) -> Result<i32> {
            let _t0: i32 = Integer::numberOfLeadingZeros(n)?;
            Ok((_t0|406i32))
        }

        #[java_method(name = "initTable", descriptor = "()[Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;")]
        pub fn initTable(&self) -> Result<Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>> {
            let this = self;
            let mut sc = Default::default();
            loop {
                let mut tab = this.__get_table();
                if ((tab.borrow().len() as i32)!=0) { break; }
                sc = this.__get_sizeCtl();
                if (this.__get_sizeCtl()<0) {
                    Thread::yield_()?;
                    continue;
                }
                break;
            }
            let _t0 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::SIZECTL(), sc, -1i32)?;
            let mut tab = this.__get_table();
            let mut n = (if (sc>0) { sc } else { 16i32 });
            let mut _arr1: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); n as usize]));
            let mut nt: Rc<RefCell<Vec<Object>>> = _arr1;
            let mut tab: Rc<RefCell<Vec<Object>>> = nt;
            this.__set_table(Clone::clone(&tab));
            let mut sc = (n).wrapping_sub(((n as u32>>(2i32&0x1f)) as i32));
            this.__set_sizeCtl(sc);
            Ok(Default::default())
        }

        #[java_method(name = "addCount", descriptor = "(JI)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addCount(&self, mut x: i64, mut check: i32) -> Result<()> {
            let this = self;
            let mut cs = this.__get_counterCells();
            let mut b = this.__get_baseCount();
            let mut s = (b).wrapping_add(x);
            let _t0 = ConcurrentHashMap::<Object, Object>::BASECOUNT().compareAndSetLong(this.__get_baseCount().into(), ConcurrentHashMap::<Object, Object>::BASECOUNT(), (b).wrapping_add(x), ConcurrentHashMap::<Object, Object>::BASECOUNT())?;
            let mut uncontended: i32 = 1i32;
            let mut m = ((cs.borrow().len() as i32)).wrapping_sub(1i32);
            let _t1: i32 = ThreadLocalRandom::getProbe()?;
            let mut c = Clone::clone(&cs.borrow()[(_t1&m) as usize]);
            let mut v = c.__get_value();
            let _t2 = c.compareAndSetLong(ConcurrentHashMap::<Object, Object>::CELLVALUE().into(), c.__get_value(), ConcurrentHashMap::<Object, Object>::CELLVALUE(), (v).wrapping_add(x))?;
            uncontended = (_t2) as i32;
            if !(_t2) {
                this.fullAddCount(x, (uncontended != 0i32))?;
                return Ok(());
            }
            if check <= 1i32 {
                return Ok(());
            }
            let _t3 = this.sumCount()?;
            s = _t3;
            loop {
                m = this.__get_sizeCtl();
                if (((s>((this.__get_sizeCtl() as i64))) as i32-((s)<((this.__get_sizeCtl() as i64))) as i32)<0) { break; }
                let mut c = this.__get_table();
                let mut n = (c.borrow().len() as i32);
                let _t4: i32 = ConcurrentHashMap::<Object, Object>::resizeStamp(n)?;
                uncontended = (_t4<<(16i32&0x1f));
                if (m<0) {
                    let mut v = this.__get_nextTable();
                    if (this.__get_transferIndex()<=0) {
                        break;
                    }
                    let _t5 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::SIZECTL(), m, (m).wrapping_add(1i32))?;
                    this.transfer(Default::default(), Default::default())?;
                } else {
                    let _t5 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::SIZECTL(), m, (uncontended).wrapping_add(2i32))?;
                    if _t5 {
                        this.transfer(Default::default(), Default::default())?;
                    }
                }
                let _t5 = this.sumCount()?;
                s = _t5;
            }
            Ok(())
        }

        #[java_method(name = "helpTransfer", descriptor = "([Ljava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)[Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;)[Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;")]
        pub fn helpTransfer(&self, mut tab: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>, mut f: ConcurrentHashMap_Node<K, V>) -> Result<Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>> {
            let this = self;
            let mut nextTab = Default::default().__get_nextTable();
            let _t0: i32 = ConcurrentHashMap::<Object, Object>::resizeStamp((tab.borrow().len() as i32))?;
            let mut rs = (_t0<<(16i32&0x1f));
            loop {
                let mut sc = this.__get_sizeCtl();
                if (this.__get_transferIndex()<=0) {
                    break;
                }
                let _t1 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::SIZECTL(), sc, (sc).wrapping_add(1i32))?;
                if _t1 { break; }
            }
            this.transfer(Clone::clone(&tab), Default::default())?;
            return Ok(Default::default());
            Ok(Default::default())
        }

        #[java_method(name = "tryPresize", descriptor = "(I)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryPresize(&self, mut size: i32) -> Result<()> {
            let this = self;
            let mut _merged1: i32;
            if size >= 464i32 {
                _merged1 = 1073741824i32;
            } else {
                let _t0: i32 = ConcurrentHashMap::<Object, Object>::tableSizeFor(((size).wrapping_add(((size as u32>>(1i32&0x1f)) as i32))).wrapping_add(1i32))?;
                _merged1 = _t0;
            }
            let mut c: i32 = _merged1;
            loop {
                let mut sc = this.__get_sizeCtl();
                if (this.__get_sizeCtl()<0) { break; }
                let mut tab = this.__get_table();
                let mut n = (tab.borrow().len() as i32);
                if ((tab.borrow().len() as i32)==0) {
                    n = (if sc > c { sc } else { c });
                    let _t2 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::SIZECTL(), sc, -1i32)?;
                    if Object::from_any(this.__get_table().clone()) == Object::from_any(tab.clone()) {
                        let mut _arr3: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); n as usize]));
                        let mut nt: Rc<RefCell<Vec<Object>>> = _arr3;
                        this.__set_table(Clone::clone(&nt));
                        sc = (n).wrapping_sub(((n as u32>>(2i32&0x1f)) as i32));
                    }
                    this.__set_sizeCtl(sc);
                } else {
                    if n >= 1073741824i32 {
                        break;
                    }
                    let _t2: i32 = ConcurrentHashMap::<Object, Object>::resizeStamp(n)?;
                    let mut nt: i32 = _t2;
                    let _t3 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::SIZECTL(), sc, ((nt<<(16i32&0x1f))).wrapping_add(2i32))?;
                    if _t3 {
                        this.transfer(Default::default(), Default::default())?;
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "transfer", descriptor = "([Ljava/util/concurrent/ConcurrentHashMap$Node;[Ljava/util/concurrent/ConcurrentHashMap$Node;)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;[Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;)V")]
        pub fn transfer(&self, mut tab: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>, mut nextTab: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>) -> Result<()> {
            let this = self;
            let mut n = (tab.borrow().len() as i32);
            let mut stride = (if (ConcurrentHashMap::<Object, Object>::NCPU()) > 1i32 { (((n as u32>>(3i32&0x1f)) as i32)/ConcurrentHashMap::<Object, Object>::NCPU()) } else { n });
            if (if (ConcurrentHashMap::<Object, Object>::NCPU()) > 1i32 { (((n as u32>>(3i32&0x1f)) as i32)/ConcurrentHashMap::<Object, Object>::NCPU()) } else { n }) < 16i32 {
                stride = 16i32;
            }
            if _is_jnull(&nextTab) {
                let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); (n<<(1i32&0x1f)) as usize]));
                let mut nt: Rc<RefCell<Vec<Object>>> = _arr0;
                let mut nextTab: Rc<RefCell<Vec<Object>>> = nt;
                let mut nt = (panic!("stack underflow") as i32);
                this.__set_sizeCtl(2147483647i32);
                return Ok(());
                this.__set_nextTable(Clone::clone(&nextTab));
                this.__set_transferIndex(n);
            }
            let mut nt = (nextTab.borrow().len() as i32);
            let mut fwd = ConcurrentHashMap_ForwardingNode::<Object, Object>::new(Clone::clone(&nextTab))?;
            let mut advance: i32 = 1i32;
            let mut finishing: i32 = 0i32;
            let mut i: i32 = 0i32;
            let mut bound: i32 = 0i32;
            loop {
                i = i.wrapping_sub(1i32);
        let mut nextBound = Default::default();
                if (finishing!=0) {
                    advance = 0i32;
                } else {
                    let mut nextIndex = this.__get_transferIndex();
        nextBound = Default::default();
                    if (this.__get_transferIndex()<=0) {
                        i = -1i32;
                        advance = 0i32;
                    } else {
                        nextBound = (if nextIndex > stride { (nextIndex).wrapping_sub(stride) } else { 0i32 });
                        let _t0 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::TRANSFERINDEX(), nextIndex, (if nextIndex > stride { (nextIndex).wrapping_sub(stride) } else { 0i32 }))?;
                        if _t0 {
                            bound = nextBound;
                            i = (nextIndex).wrapping_sub(1i32);
                            advance = 0i32;
                        }
                    }
                }
                continue;
                if (i).wrapping_add(n) >= nt {
                    if (finishing!=0) {
                        this.__set_nextTable(Default::default());
                        this.__set_table(Clone::clone(&nextTab));
                        this.__set_sizeCtl(((n<<(1i32&0x1f))).wrapping_sub(((n as u32>>(1i32&0x1f)) as i32)));
                        return Ok(());
                    }
                    let mut nextIndex = this.__get_sizeCtl();
                    let _t0 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::SIZECTL(), this.__get_sizeCtl(), (nextIndex).wrapping_sub(1i32))?;
                    let _t1: i32 = ConcurrentHashMap::<Object, Object>::resizeStamp(n)?;
                    if (nextIndex).wrapping_sub(2i32) != (_t1<<(16i32&0x1f)) {
                        return Ok(());
                    }
                    advance = 1i32;
                    finishing = 1i32;
                    i = n;
                } else {
                    let _t0: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::tabAt(Clone::clone(&tab), i)?;
                    let mut f: ConcurrentHashMap_Node<Object, Object> = _t0;
                    if _is_jnull(&f) {
                        let _t1: bool = ConcurrentHashMap::<Object, Object>::casTabAt(Clone::clone(&tab), i, Default::default(), Clone::clone(&fwd).into())?;
                        advance = (_t1) as i32;
                    } else {
                        let mut fh = f.__get_hash();
                        if f.__get_hash() == -1i32 {
                            advance = 1i32;
                        } else {
                            let mut nextIndex: ConcurrentHashMap_Node<Object, Object> = f;
                            let _t1: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::tabAt(Clone::clone(&tab), i)?;
        let mut b = Default::default();
        let mut p = Default::default();
        let mut lastRun: ConcurrentHashMap_Node<Object, Object> = Default::default();
        let mut hn: ConcurrentHashMap_Node<K, V> = Default::default();
                            if (fh>=0) {
                                let mut runBit = (fh&n);
                                lastRun = f;
                                p = f.__get_next();
                                b = Default::default();
                                loop {
                                    if _is_jnull(&p) { break; }
                                    b = (p.__get_hash()&n);
                                    if b != runBit {
                                        runBit = b;
                                        lastRun = p;
                                    }
                                    p = p.__get_next();
                                }
        hn = Default::default();
                                if (runBit==0) {
                                    nextBound = lastRun;
                                    hn = Default::default();
                                } else {
                                    hn = lastRun;
                                    nextBound = Object::default();
                                }
                                p = f;
                                loop {
                                    if Object::from_any(p.clone()) == Object::from_any(lastRun.clone()) { break; }
                                    b = p.__get_hash();
                                    let mut pk = p.__get_key();
                                    let mut pv = p.__get_val();
                                    if ((b&n)==0) {
                                        nextBound = ConcurrentHashMap_Node::<Object, Object>::new_i_obj_obj_concur(b, Clone::clone(&pk), Clone::clone(&pv), Clone::clone(&nextBound))?;
                                    } else {
                                        hn = ConcurrentHashMap_Node::<Object, Object>::new_i_obj_obj_concur(b, Clone::clone(&pk), Clone::clone(&pv), Clone::clone(&hn))?;
                                    }
                                    p = p.__get_next();
                                }
                                ConcurrentHashMap::<Object, Object>::setTabAt(Clone::clone(&nextTab), i, Clone::clone(&nextBound))?;
                                ConcurrentHashMap::<Object, Object>::setTabAt(Clone::clone(&nextTab), (i).wrapping_add(n), Clone::clone(&hn))?;
                                ConcurrentHashMap::<Object, Object>::setTabAt(Clone::clone(&tab), i, Clone::clone(&fwd).into())?;
                                advance = 1i32;
                            } else {
                                if false {
                                    let mut runBit: ConcurrentHashMap_TreeBin<Object, Object> = Default::default();
                                    lastRun = Default::default();
                                    p = Default::default();
                                    b = Object::default();
                                    let mut pk: K = Default::default();
                                    let mut pv: i32 = 0i32;
                                    let mut hc: i32 = 0i32;
                                    let mut e = runBit.__get_first();
                                    loop {
                                        if _is_jnull(&e) { break; }
                                        let mut h = e.__get_hash();
                                        p = ConcurrentHashMap_TreeNode::<Object, Object>::new(h, Clone::clone(&e.__get_key()), Clone::clone(&e.__get_val()), Default::default(), Default::default())?;
                                        if ((h&n)==0) {
                                            p.__set_prev(Clone::clone(&p));
                                            if _is_jnull(&p) {
                                                lastRun = p;
                                            } else {
                                                p.__set_next(Clone::clone(&<_ as Into<ConcurrentHashMap_Node<Object, Object>>>::into(p)));
                                            }
                                            p = p;
                                            pv = pv.wrapping_add(1i32);
                                        } else {
                                            p.__set_prev(Clone::clone(&pk));
                                            if _is_jnull(&pk) {
                                                b = p;
                                            } else {
                                                pk.__set_next(Clone::clone(&<_ as Into<ConcurrentHashMap_Node<Object, Object>>>::into(p)));
                                            }
                                            let mut pk: ConcurrentHashMap_TreeNode<Object, Object> = p;
                                            hc = hc.wrapping_add(1i32);
                                        }
                                        let mut e = e.__get_next();
                                    }
                                    let mut _merged3: ConcurrentHashMap_Node<Object, Object>;
                                    if pv <= 6i32 {
                                        let _t2: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::untreeify(Clone::clone(&lastRun).into())?;
                                        _merged3 = _t2;
                                    } else {
                                        _merged3 = (if (hc!=0) { ConcurrentHashMap_TreeBin::<Object, Object>::new(Clone::clone(&lastRun))? } else { runBit });
                                    }
                                    nextBound = _merged3;
                                    let mut _merged5: ConcurrentHashMap_Node<Object, Object>;
                                    if hc <= 6i32 {
                                        let _t4: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::untreeify(Clone::clone(&b))?;
                                        _merged5 = _t4;
                                    } else {
                                        _merged5 = (if (pv!=0) { ConcurrentHashMap_TreeBin::<Object, Object>::new(Clone::clone(&b))? } else { runBit });
                                    }
                                    hn = _merged5;
                                    ConcurrentHashMap::<Object, Object>::setTabAt(Clone::clone(&nextTab), i, Clone::clone(&nextBound))?;
                                    ConcurrentHashMap::<Object, Object>::setTabAt(Clone::clone(&nextTab), (i).wrapping_add(n), Clone::clone(&hn))?;
                                    ConcurrentHashMap::<Object, Object>::setTabAt(Clone::clone(&tab), i, Clone::clone(&fwd).into())?;
                                    advance = 1i32;
                                } else {
                                    if false {
                                        return Err(JvmError::Custom("athrow".to_owned()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "sumCount", descriptor = "()J", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sumCount(&self) -> Result<i64> {
            let this = self;
            let mut cs = this.__get_counterCells();
            let mut sum = this.__get_baseCount();
            let mut local_4: Rc<RefCell<Vec<ConcurrentHashMap_CounterCell>>> = cs;
            let mut local_5 = (local_4.borrow().len() as i32);
            let mut local_6: i32 = 0i32;
            loop {
                if local_6 >= local_5 { break; }
                let mut c = Clone::clone(&local_4.borrow()[local_6 as usize]);
                if !_is_jnull(&c) {
                    sum = (sum).wrapping_add(c.__get_value());
                }
                local_6 = local_6.wrapping_add(1i32);
            }
            Ok(sum)
        }

        #[java_method(name = "fullAddCount", descriptor = "(JZ)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fullAddCount(&self, mut x: i64, mut wasUncontended: bool) -> Result<()> {
            let this = self;
            let _t0: i32 = ThreadLocalRandom::getProbe()?;
            let mut h: i32 = _t0;
            if (h==0) {
                ThreadLocalRandom::localInit()?;
                let _t1: i32 = ThreadLocalRandom::getProbe()?;
                h = _t1;
                let mut wasUncontended: i32 = 1i32;
            }
            let mut collide: i32 = 0i32;
            loop {
                let mut cs = this.__get_counterCells();
                let mut n = (cs.borrow().len() as i32);
                if ((cs.borrow().len() as i32)>0) {
                    let mut c = Clone::clone(&cs.borrow()[((n).wrapping_sub(1i32)&h) as usize]);
                    if _is_jnull(&Clone::clone(&cs.borrow()[((n).wrapping_sub(1i32)&h) as usize])) {
                        if (this.__get_cellsBusy()==0) {
                            let mut r = ConcurrentHashMap_CounterCell::new(x)?;
                            if (this.__get_cellsBusy()==0) {
                                let _t1 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::CELLSBUSY(), 0i32, 1i32)?;
                                if _t1 {
                                    let mut created: i32 = 0i32;
                                    let mut rs = this.__get_counterCells();
                                    let mut m = (rs.borrow().len() as i32);
                                    let mut j = ((m).wrapping_sub(1i32)&h);
                                    if _is_jnull(&Clone::clone(&rs.borrow()[((m).wrapping_sub(1i32)&h) as usize])) {
                                        rs.borrow_mut()[j as usize] = Clone::clone(&r);
                                        created = 1i32;
                                    }
                                    this.__set_cellsBusy(0i32);
                                } else {
                                    collide = 0i32;
                                    let _t2: i32 = ThreadLocalRandom::advanceProbe(h)?;
                                    h = _t2;
                                    continue;
                                }
                            } else {
                                collide = 0i32;
                                let _t1: i32 = ThreadLocalRandom::advanceProbe(h)?;
                                h = _t1;
                                continue;
                            }
                        } else {
                            collide = 0i32;
                            let _t1: i32 = ThreadLocalRandom::advanceProbe(h)?;
                            h = _t1;
                            continue;
                        }
                    } else {
                        if (wasUncontended==0) {
                            wasUncontended = 1i32;
                        } else {
                            let mut v = c.__get_value();
                            let _t1 = c.compareAndSetLong(ConcurrentHashMap::<Object, Object>::CELLVALUE().into(), c.__get_value(), ConcurrentHashMap::<Object, Object>::CELLVALUE(), (v).wrapping_add(x))?;
                            if _t1 {
                                break;
                            }
                            if n >= (ConcurrentHashMap::<Object, Object>::NCPU()) {
                                collide = 0i32;
                            } else {
                                if (collide==0) {
                                    collide = 1i32;
                                } else {
                                    let _t2 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::CELLSBUSY(), 0i32, 1i32)?;
                                    if Object::from_any(this.__get_counterCells().clone()) == Object::from_any(cs.clone()) {
                                        let _t3: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Default::default(), (n<<(1i32&0x1f)))?;
                                        this.__set_counterCells(Default::default());
                                    }
                                    this.__set_cellsBusy(0i32);
                                    collide = 0i32;
                                    continue;
                                }
                            }
                        }
                    }
                    let _t1: i32 = ThreadLocalRandom::advanceProbe(h)?;
                    h = _t1;
                } else {
                    if (this.__get_cellsBusy()==0) {
                        if Object::from_any(this.__get_counterCells().clone()) == Object::from_any(cs.clone()) {
                            let _t1 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap::<Object, Object>::CELLSBUSY(), 0i32, 1i32)?;
                            if _t1 {
                                let mut r: i32 = 0i32;
                                if Object::from_any(this.__get_counterCells().clone()) == Object::from_any(cs.clone()) {
                                    let mut _arr2: Rc<RefCell<Vec<ConcurrentHashMap_CounterCell>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
                                    let mut created: Rc<RefCell<Vec<ConcurrentHashMap_CounterCell>>> = _arr2;
                                    created.borrow_mut()[(h&1i32) as usize] = Clone::clone(&ConcurrentHashMap_CounterCell::new(x)?);
                                    this.__set_counterCells(Clone::clone(&created));
                                    r = 1i32;
                                }
                                this.__set_cellsBusy(0i32);
                                if (r!=0) {
                                } else {
                                    continue;
                                }
                            } else {
                                let mut v = this.__get_baseCount();
                                let _t2 = this.compareAndSetLong(ConcurrentHashMap::<Object, Object>::BASECOUNT().into(), this.__get_baseCount(), ConcurrentHashMap::<Object, Object>::BASECOUNT(), (v).wrapping_add(x))?;
                                if _t2 {
                                } else {
                                    continue;
                                }
                            }
                        } else {
                            let mut v = this.__get_baseCount();
                            let _t1 = this.compareAndSetLong(ConcurrentHashMap::<Object, Object>::BASECOUNT().into(), this.__get_baseCount(), ConcurrentHashMap::<Object, Object>::BASECOUNT(), (v).wrapping_add(x))?;
                            if _t1 {
                            } else {
                                continue;
                            }
                        }
                    } else {
                        let mut v = this.__get_baseCount();
                        let _t1 = this.compareAndSetLong(ConcurrentHashMap::<Object, Object>::BASECOUNT().into(), this.__get_baseCount(), ConcurrentHashMap::<Object, Object>::BASECOUNT(), (v).wrapping_add(x))?;
                        if _t1 {
                        } else {
                            continue;
                        }
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "treeifyBin", descriptor = "([Ljava/util/concurrent/ConcurrentHashMap$Node;I)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;I)V")]
        pub fn treeifyBin(&self, mut tab: Rc<RefCell<Vec<ConcurrentHashMap_Node<K, V>>>>, mut index: i32) -> Result<()> {
            let this = self;
            let mut n = (tab.borrow().len() as i32);
        let mut hd: ConcurrentHashMap_TreeNode<K, V> = Default::default();
            if (tab.borrow().len() as i32) < 64i32 {
                this.tryPresize((n<<(1i32&0x1f)))?;
            } else {
                let _t0: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::tabAt(Clone::clone(&tab), index)?;
                let mut b: ConcurrentHashMap_Node<Object, Object> = _t0;
                let mut local_5: ConcurrentHashMap_Node<Object, Object> = b;
                let _t1: ConcurrentHashMap_Node<Object, Object> = ConcurrentHashMap::<Object, Object>::tabAt(Clone::clone(&tab), index)?;
                hd = Default::default();
                let mut tl: ConcurrentHashMap_TreeNode<K, V> = Default::default();
                let mut e: ConcurrentHashMap_Node<Object, Object> = b;
                loop {
                    if _is_jnull(&e) { break; }
                    let mut p = ConcurrentHashMap_TreeNode::<Object, Object>::new(e.__get_hash(), Clone::clone(&e.__get_key()), Clone::clone(&e.__get_val()), Default::default(), Default::default())?;
                    p.__set_prev(Clone::clone(&tl));
                    if _is_jnull(&tl) {
                        hd = p;
                    } else {
                        tl.__set_next(Clone::clone(&<_ as Into<ConcurrentHashMap_Node<Object, Object>>>::into(p)));
                    }
                    let mut tl: ConcurrentHashMap_TreeNode<Object, Object> = p;
                    e = e.__get_next();
                }
                ConcurrentHashMap::<Object, Object>::setTabAt(Clone::clone(&tab), index, Clone::clone(&ConcurrentHashMap_TreeBin::<Object, Object>::new(Clone::clone(&hd))?).into())?;
            }
            Ok(())
        }

        #[java_method(name = "untreeify", descriptor = "(Ljava/util/concurrent/ConcurrentHashMap$Node;)Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;)Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;")]
        pub fn untreeify(mut b: ConcurrentHashMap_Node<K, V>) -> Result<ConcurrentHashMap_Node<K, V>> {
            let mut hd: ConcurrentHashMap_Node<K, V> = Default::default();
            let mut tl: ConcurrentHashMap_Node<K, V> = Default::default();
            let mut q: ConcurrentHashMap_Node<K, V> = b;
            loop {
                if _is_jnull(&q) { break; }
                let mut p = ConcurrentHashMap_Node::<Object, Object>::new_i_obj_obj(q.__get_hash(), Clone::clone(&q.__get_key()), Clone::clone(&q.__get_val()))?;
                if _is_jnull(&tl) {
                    let mut hd: ConcurrentHashMap_Node<Object, Object> = p;
                } else {
                    tl.__set_next(Clone::clone(&p));
                }
                let mut tl: ConcurrentHashMap_Node<Object, Object> = p;
                let mut q = q.__get_next();
            }
            Ok(Default::default())
        }

        #[java_method(name = "batchFor", descriptor = "(J)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn batchFor(&self, b: i64) -> Result<i32> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.batchFor:(J)I")
        }

        #[java_method(name = "forEach", descriptor = "(JLjava/util/function/BiConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/BiConsumer<-TK;-TV;>;)V")]
        pub fn forEach_l_bicons(&self, parallelismThreshold: i64, arg1: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEach:(JLjava/util/function/BiConsumer;)V")
        }

        #[java_method(name = "forEach", descriptor = "(JLjava/util/function/BiFunction;Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/BiFunction<-TK;-TV;+TU;>;Ljava/util/function/Consumer<-TU;>;)V")]
        pub fn forEach_l_bifunc_consum(&self, parallelismThreshold: i64, arg1: Object, transformer: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEach:(JLjava/util/function/BiFunction;Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "search", descriptor = "(JLjava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/BiFunction<-TK;-TV;+TU;>;)TU;")]
        pub fn search(&self, parallelismThreshold: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.search:(JLjava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduce", descriptor = "(JLjava/util/function/BiFunction;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/BiFunction<-TK;-TV;+TU;>;Ljava/util/function/BiFunction<-TU;-TU;+TU;>;)TU;")]
        pub fn reduce(&self, parallelismThreshold: i64, arg1: Object, transformer: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduce:(JLjava/util/function/BiFunction;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceToDouble", descriptor = "(JLjava/util/function/ToDoubleBiFunction;DLjava/util/function/DoubleBinaryOperator;)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToDoubleBiFunction<-TK;-TV;>;DLjava/util/function/DoubleBinaryOperator;)D")]
        pub fn reduceToDouble(&self, parallelismThreshold: i64, arg1: Object, transformer: f64, basis: Object) -> Result<f64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceToDouble:(JLjava/util/function/ToDoubleBiFunction;DLjava/util/function/DoubleBinaryOperator;)D")
        }

        #[java_method(name = "reduceToLong", descriptor = "(JLjava/util/function/ToLongBiFunction;JLjava/util/function/LongBinaryOperator;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToLongBiFunction<-TK;-TV;>;JLjava/util/function/LongBinaryOperator;)J")]
        pub fn reduceToLong(&self, parallelismThreshold: i64, arg1: Object, transformer: i64, basis: Object) -> Result<i64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceToLong:(JLjava/util/function/ToLongBiFunction;JLjava/util/function/LongBinaryOperator;)J")
        }

        #[java_method(name = "reduceToInt", descriptor = "(JLjava/util/function/ToIntBiFunction;ILjava/util/function/IntBinaryOperator;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToIntBiFunction<-TK;-TV;>;ILjava/util/function/IntBinaryOperator;)I")]
        pub fn reduceToInt(&self, parallelismThreshold: i64, arg1: Object, transformer: i32, basis: Object) -> Result<i32> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceToInt:(JLjava/util/function/ToIntBiFunction;ILjava/util/function/IntBinaryOperator;)I")
        }

        #[java_method(name = "forEachKey", descriptor = "(JLjava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/Consumer<-TK;>;)V")]
        pub fn forEachKey_l_consum(&self, parallelismThreshold: i64, arg1: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEachKey:(JLjava/util/function/Consumer;)V")
        }

        #[java_method(name = "forEachKey", descriptor = "(JLjava/util/function/Function;Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<-TK;+TU;>;Ljava/util/function/Consumer<-TU;>;)V")]
        pub fn forEachKey_l_functi_consum(&self, parallelismThreshold: i64, arg1: Object, transformer: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEachKey:(JLjava/util/function/Function;Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "searchKeys", descriptor = "(JLjava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<-TK;+TU;>;)TU;")]
        pub fn searchKeys(&self, parallelismThreshold: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.searchKeys:(JLjava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceKeys", descriptor = "(JLjava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/BiFunction<-TK;-TK;+TK;>;)TK;")]
        pub fn reduceKeys_l_bifunc(&self, parallelismThreshold: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceKeys:(JLjava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceKeys", descriptor = "(JLjava/util/function/Function;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<-TK;+TU;>;Ljava/util/function/BiFunction<-TU;-TU;+TU;>;)TU;")]
        pub fn reduceKeys_l_functi_bifunc(&self, parallelismThreshold: i64, arg1: Object, transformer: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceKeys:(JLjava/util/function/Function;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceKeysToDouble", descriptor = "(JLjava/util/function/ToDoubleFunction;DLjava/util/function/DoubleBinaryOperator;)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToDoubleFunction<-TK;>;DLjava/util/function/DoubleBinaryOperator;)D")]
        pub fn reduceKeysToDouble(&self, parallelismThreshold: i64, arg1: Object, transformer: f64, basis: Object) -> Result<f64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceKeysToDouble:(JLjava/util/function/ToDoubleFunction;DLjava/util/function/DoubleBinaryOperator;)D")
        }

        #[java_method(name = "reduceKeysToLong", descriptor = "(JLjava/util/function/ToLongFunction;JLjava/util/function/LongBinaryOperator;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToLongFunction<-TK;>;JLjava/util/function/LongBinaryOperator;)J")]
        pub fn reduceKeysToLong(&self, parallelismThreshold: i64, arg1: Object, transformer: i64, basis: Object) -> Result<i64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceKeysToLong:(JLjava/util/function/ToLongFunction;JLjava/util/function/LongBinaryOperator;)J")
        }

        #[java_method(name = "reduceKeysToInt", descriptor = "(JLjava/util/function/ToIntFunction;ILjava/util/function/IntBinaryOperator;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToIntFunction<-TK;>;ILjava/util/function/IntBinaryOperator;)I")]
        pub fn reduceKeysToInt(&self, parallelismThreshold: i64, arg1: Object, transformer: i32, basis: Object) -> Result<i32> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceKeysToInt:(JLjava/util/function/ToIntFunction;ILjava/util/function/IntBinaryOperator;)I")
        }

        #[java_method(name = "forEachValue", descriptor = "(JLjava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/Consumer<-TV;>;)V")]
        pub fn forEachValue_l_consum(&self, parallelismThreshold: i64, arg1: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEachValue:(JLjava/util/function/Consumer;)V")
        }

        #[java_method(name = "forEachValue", descriptor = "(JLjava/util/function/Function;Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<-TV;+TU;>;Ljava/util/function/Consumer<-TU;>;)V")]
        pub fn forEachValue_l_functi_consum(&self, parallelismThreshold: i64, arg1: Object, transformer: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEachValue:(JLjava/util/function/Function;Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "searchValues", descriptor = "(JLjava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<-TV;+TU;>;)TU;")]
        pub fn searchValues(&self, parallelismThreshold: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.searchValues:(JLjava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceValues", descriptor = "(JLjava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;")]
        pub fn reduceValues_l_bifunc(&self, parallelismThreshold: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceValues:(JLjava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceValues", descriptor = "(JLjava/util/function/Function;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<-TV;+TU;>;Ljava/util/function/BiFunction<-TU;-TU;+TU;>;)TU;")]
        pub fn reduceValues_l_functi_bifunc(&self, parallelismThreshold: i64, arg1: Object, transformer: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceValues:(JLjava/util/function/Function;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceValuesToDouble", descriptor = "(JLjava/util/function/ToDoubleFunction;DLjava/util/function/DoubleBinaryOperator;)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToDoubleFunction<-TV;>;DLjava/util/function/DoubleBinaryOperator;)D")]
        pub fn reduceValuesToDouble(&self, parallelismThreshold: i64, arg1: Object, transformer: f64, basis: Object) -> Result<f64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceValuesToDouble:(JLjava/util/function/ToDoubleFunction;DLjava/util/function/DoubleBinaryOperator;)D")
        }

        #[java_method(name = "reduceValuesToLong", descriptor = "(JLjava/util/function/ToLongFunction;JLjava/util/function/LongBinaryOperator;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToLongFunction<-TV;>;JLjava/util/function/LongBinaryOperator;)J")]
        pub fn reduceValuesToLong(&self, parallelismThreshold: i64, arg1: Object, transformer: i64, basis: Object) -> Result<i64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceValuesToLong:(JLjava/util/function/ToLongFunction;JLjava/util/function/LongBinaryOperator;)J")
        }

        #[java_method(name = "reduceValuesToInt", descriptor = "(JLjava/util/function/ToIntFunction;ILjava/util/function/IntBinaryOperator;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToIntFunction<-TV;>;ILjava/util/function/IntBinaryOperator;)I")]
        pub fn reduceValuesToInt(&self, parallelismThreshold: i64, arg1: Object, transformer: i32, basis: Object) -> Result<i32> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceValuesToInt:(JLjava/util/function/ToIntFunction;ILjava/util/function/IntBinaryOperator;)I")
        }

        #[java_method(name = "forEachEntry", descriptor = "(JLjava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/Consumer<-Ljava/util/Map$Entry<TK;TV;>;>;)V")]
        pub fn forEachEntry_l_consum(&self, parallelismThreshold: i64, arg1: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEachEntry:(JLjava/util/function/Consumer;)V")
        }

        #[java_method(name = "forEachEntry", descriptor = "(JLjava/util/function/Function;Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<Ljava/util/Map$Entry<TK;TV;>;+TU;>;Ljava/util/function/Consumer<-TU;>;)V")]
        pub fn forEachEntry_l_functi_consum(&self, parallelismThreshold: i64, arg1: Object, transformer: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.forEachEntry:(JLjava/util/function/Function;Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "searchEntries", descriptor = "(JLjava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<Ljava/util/Map$Entry<TK;TV;>;+TU;>;)TU;")]
        pub fn searchEntries(&self, parallelismThreshold: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.searchEntries:(JLjava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceEntries", descriptor = "(JLjava/util/function/BiFunction;)Ljava/util/Map$Entry;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/BiFunction<Ljava/util/Map$Entry<TK;TV;>;Ljava/util/Map$Entry<TK;TV;>;+Ljava/util/Map$Entry<TK;TV;>;>;)Ljava/util/Map$Entry<TK;TV;>;")]
        pub fn reduceEntries_l_bifunc(&self, parallelismThreshold: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceEntries:(JLjava/util/function/BiFunction;)Ljava/util/Map$Entry;")
        }

        #[java_method(name = "reduceEntries", descriptor = "(JLjava/util/function/Function;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(JLjava/util/function/Function<Ljava/util/Map$Entry<TK;TV;>;+TU;>;Ljava/util/function/BiFunction<-TU;-TU;+TU;>;)TU;")]
        pub fn reduceEntries_l_functi_bifunc(&self, parallelismThreshold: i64, arg1: Object, transformer: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceEntries:(JLjava/util/function/Function;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "reduceEntriesToDouble", descriptor = "(JLjava/util/function/ToDoubleFunction;DLjava/util/function/DoubleBinaryOperator;)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToDoubleFunction<Ljava/util/Map$Entry<TK;TV;>;>;DLjava/util/function/DoubleBinaryOperator;)D")]
        pub fn reduceEntriesToDouble(&self, parallelismThreshold: i64, arg1: Object, transformer: f64, basis: Object) -> Result<f64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceEntriesToDouble:(JLjava/util/function/ToDoubleFunction;DLjava/util/function/DoubleBinaryOperator;)D")
        }

        #[java_method(name = "reduceEntriesToLong", descriptor = "(JLjava/util/function/ToLongFunction;JLjava/util/function/LongBinaryOperator;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToLongFunction<Ljava/util/Map$Entry<TK;TV;>;>;JLjava/util/function/LongBinaryOperator;)J")]
        pub fn reduceEntriesToLong(&self, parallelismThreshold: i64, arg1: Object, transformer: i64, basis: Object) -> Result<i64> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceEntriesToLong:(JLjava/util/function/ToLongFunction;JLjava/util/function/LongBinaryOperator;)J")
        }

        #[java_method(name = "reduceEntriesToInt", descriptor = "(JLjava/util/function/ToIntFunction;ILjava/util/function/IntBinaryOperator;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/ToIntFunction<Ljava/util/Map$Entry<TK;TV;>;>;ILjava/util/function/IntBinaryOperator;)I")]
        pub fn reduceEntriesToInt(&self, parallelismThreshold: i64, arg1: Object, transformer: i32, basis: Object) -> Result<i32> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap.reduceEntriesToInt:(JLjava/util/function/ToIntFunction;ILjava/util/function/IntBinaryOperator;)I")
        }
    }
}
