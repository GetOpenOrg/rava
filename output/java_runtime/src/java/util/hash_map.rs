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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<HashMap<K, V>> for AbstractMap<K, V> {
    fn from(v: HashMap<K, V>) -> AbstractMap<K, V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/HashMap"]
    #[super_class       = "java/util/AbstractMap"]
    #[interfaces        = "java/util/Map,java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/AbstractMap<TK;TV;>;Ljava/util/Map<TK;TV;>;Ljava/lang/Cloneable;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "HashMap.java"]
    #[inner_classes     = "java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/HashMap$TreeNode:java/util/HashMap:TreeNode:24;java/util/HashMap$KeySet:java/util/HashMap:KeySet:16;java/util/HashMap$Values:java/util/HashMap:Values:16;java/util/HashMap$EntrySet:java/util/HashMap:EntrySet:16;java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/util/HashMap$UnsafeHolder:java/util/HashMap:UnsafeHolder:26;java/util/HashMap$EntrySpliterator:java/util/HashMap:EntrySpliterator:24;java/util/HashMap$ValueSpliterator:java/util/HashMap:ValueSpliterator:24;java/util/HashMap$KeySpliterator:java/util/HashMap:KeySpliterator:24;java/util/HashMap$HashMapSpliterator:java/util/HashMap:HashMapSpliterator:8;java/util/HashMap$EntryIterator:java/util/HashMap:EntryIterator:16;java/util/HashMap$ValueIterator:java/util/HashMap:ValueIterator:16;java/util/HashMap$KeyIterator:java/util/HashMap:KeyIterator:16;java/util/HashMap$HashIterator:java/util/HashMap:HashIterator:1024"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractMap<K, V>"]
    #[superclass_fields(keySet: Object, values: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/AbstractMap;java/util/HashMap;java/util/Map"]

    pub struct HashMap<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/util/HashMap$Node;", access = "package", modifiers = "transient", is_static = false, generic_signature = "[Ljava/util/HashMap$Node<TK;TV;>;"))]
        pub table: Rc<RefCell<Vec<HashMap_Node<K, V>>>>,
        #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;"))]
        pub entrySet: Object,
        #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub size: i32,
        #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub modCount: i32,
        #[cfg_attr(any(), java_field(name = "threshold", descriptor = "I", is_static = false))]
        pub threshold: i32,
        #[cfg_attr(any(), java_field(name = "loadFactor", descriptor = "F", access = "package", modifiers = "final", is_static = false))]
        pub loadFactor: f32,
    }

    impl<K, V> HashMap<K, V> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "362498820763181265"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            362498820763181265i64
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_INITIAL_CAPACITY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: DEFAULT_INITIAL_CAPACITY:I
        pub fn DEFAULT_INITIAL_CAPACITY() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "MAXIMUM_CAPACITY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1073741824"))]
        // static field: MAXIMUM_CAPACITY:I
        pub fn MAXIMUM_CAPACITY() -> i32 {
            1073741824
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_LOAD_FACTOR", descriptor = "F", access = "package", modifiers = "static final", is_static = true, constant_value = "0.75"))]
        // static field: DEFAULT_LOAD_FACTOR:F
        pub fn DEFAULT_LOAD_FACTOR() -> f32 {
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

        #[java_method(name = "hash", descriptor = "(Ljava/lang/Object;)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hash(mut key: Object) -> Result<i32> {
            let mut _merged1: i32;
            if _is_jnull(&key) {
                _merged1 = 0i32;
            } else {
                let _vdispatch0: i32 = if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                let mut h: i32 = _vdispatch0;
                _merged1 = (h^((h as u32>>(16i32&0x1f)) as i32));
            }
            Ok(_merged1)
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

        #[java_method(name = "tableSizeFor", descriptor = "(I)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tableSizeFor(mut cap: i32) -> Result<i32> {
            let _t0: i32 = Integer::numberOfLeadingZeros((cap).wrapping_sub(1i32))?;
            let mut n = ((-1i32 as u32>>(_t0&0x1f)) as i32);
            Ok((if (n<0) { 1i32 } else { (if n >= 1073741824i32 { 1073741824i32 } else { (n).wrapping_add(1i32) }) }))
        }

        #[java_method(name = "<init>", descriptor = "(IF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(IF)V
        pub fn new_i_f(mut initialCapacity: i32, mut loadFactor: f32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractMap::new()?);
            if (initialCapacity<0) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Illegal initial capacity: ")))?;
                let _t1 = _t0.append_i(initialCapacity)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if initialCapacity > 1073741824i32 {
                initialCapacity = 1073741824i32;
            }
            let _t0: bool = Float::isNaN_f(loadFactor)?;
            if _t0 {
                let _t1 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Illegal load factor: ")))?;
                let _t2 = _t1.append_f(loadFactor)?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_loadFactor(loadFactor);
            let _t1: i32 = HashMap::<Object, Object>::tableSizeFor(initialCapacity)?;
            this.__set_threshold(_t1);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(I)V
        pub fn new_i(mut initialCapacity: i32) -> Result<Self> {
            let mut this = Self::default();
            this = HashMap::new_i_f(initialCapacity, 0.75f32)?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractMap::new()?);
            this.__set_loadFactor(0.75f32);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn new_map(m: Object) -> Result<Self> {
            panic!("stub: java/util/HashMap.<init>:(Ljava/util/Map;)V")
        }

        #[java_method(name = "putMapEntries", descriptor = "(Ljava/util/Map;Z)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;Z)V")]
        pub fn putMapEntries(&self, m: Object, evict: bool) -> Result<()> {
            panic!("stub: java/util/HashMap.putMapEntries:(Ljava/util/Map;Z)V")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_size())
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            let this = self;
            Ok((this.__get_size()==0))
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn get(&self, mut key: Object) -> Result<V> {
            let this = self;
            let _t0 = this.getNode(Clone::clone(&key))?;
            let mut e: HashMap_Node<Object, Object> = _t0;
            Ok(panic!("null"))
        }

        #[java_method(name = "getNode", descriptor = "(Ljava/lang/Object;)Ljava/util/HashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)Ljava/util/HashMap$Node<TK;TV;>;")]
        pub fn getNode(&self, mut key: Object) -> Result<HashMap_Node<K, V>> {
            let this = self;
            let mut tab = this.__get_table();
            let mut n = (tab.borrow().len() as i32);
            let _t0: i32 = HashMap::<Object, Object>::hash(Clone::clone(&key))?;
            let mut hash: i32 = _t0;
            let mut first = Clone::clone(&tab.borrow()[((n).wrapping_sub(1i32)&hash) as usize]);
            let mut k = first.__get_key();
            let _vdispatch1: bool = if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&k))? } else { Default::default() };
            if _vdispatch1 {
                return Ok(Default::default());
            }
            let mut e = first.__get_next();
            if false {
                let _t2 = Default::default().getTreeNode(hash, Clone::clone(&key))?;
                return Ok(<_ as Into<HashMap_Node<K, V>>>::into(_t2));
            }
            loop {
                k = e.__get_key();
                let _vdispatch2: bool = if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&k))? } else { Default::default() };
                if _vdispatch2 {
                    return Ok(Default::default());
                }
                e = e.__get_next();
                if _is_jnull(&e.__get_next()) { break; }
            }
            Ok(Default::default())
        }

        #[java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsKey(&self, mut key: Object) -> Result<bool> {
            let this = self;
            let _t0 = this.getNode(Clone::clone(&key))?;
            Ok(!_is_jnull(&_t0))
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn put(&self, mut key: K, mut value: V) -> Result<V> {
            let this = self;
            let _t0: i32 = HashMap::<Object, Object>::hash(Clone::clone(&key))?;
            let _t1 = this.putVal(_t0, Clone::clone(&key), Clone::clone(&value), (0i32 != 0i32), (1i32 != 0i32))?;
            Ok(panic!("null"))
        }

        #[java_method(name = "putVal", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;ZZ)TV;")]
        pub fn putVal(&self, mut hash: i32, mut key: K, mut value: V, mut onlyIfAbsent: bool, mut evict: bool) -> Result<V> {
            let this = self;
            let mut tab = this.__get_table();
            let mut n = (tab.borrow().len() as i32);
            if ((tab.borrow().len() as i32)==0) {
                let _t0 = this.resize()?;
                tab = _t0;
                n = (tab.borrow().len() as i32);
            }
            let mut i = ((n).wrapping_sub(1i32)&hash);
            let mut p = Clone::clone(&tab.borrow()[((n).wrapping_sub(1i32)&hash) as usize]);
            if _is_jnull(&Clone::clone(&tab.borrow()[((n).wrapping_sub(1i32)&hash) as usize])) {
                let _t0 = this.newNode(hash, Clone::clone(&key), Clone::clone(&value), Default::default())?;
                tab.borrow_mut()[i as usize] = Clone::clone(&_t0);
            } else {
        let mut e: HashMap_Node<Object, Object> = Default::default();
                if p.__get_hash() == hash {
                    let mut k = p.__get_key();
                    if !_is_jnull(&key) {
                        let _t0 = key.equals(Clone::clone(&k))?;
                        if _t0 {
                            e = p;
                        } else {
                            if false {
                                let _t1 = Default::default().putTreeVal(Clone::clone(this), Default::default(), hash, Clone::clone(&key), Clone::clone(&value))?;
                                e = _t1;
                            } else {
                                let mut binCount: i32 = 0i32;
                                loop {
                                    e = p.__get_next();
                                    if _is_jnull(&p.__get_next()) {
                                        let _t1 = this.newNode(hash, Clone::clone(&key), Clone::clone(&value), Default::default())?;
                                        p.__set_next(Clone::clone(&_t1));
                                        this.treeifyBin(Default::default(), hash)?;
                                    } else {
                                        if e.__get_hash() == hash {
                                            k = e.__get_key();
                                            if !_is_jnull(&key) {
                                                let _t1 = key.equals(Clone::clone(&k))?;
                                                if _t1 {
                                                    break;
                                                }
                                            } else {
                                                p = e;
                                                binCount = binCount.wrapping_add(1i32);
                                                continue;
                                            }
                                        } else {
                                            p = e;
                                            binCount = binCount.wrapping_add(1i32);
                                            continue;
                                        }
                                    }
                                    if (panic!("stack underflow") as i32) < (panic!("stack underflow") as i32) { break; }
                                    this.treeifyBin(Default::default(), hash)?;
                                    break;
                                    if e.__get_hash() == hash {
                                        k = e.__get_key();
                                        if !_is_jnull(&key) {
                                            let _t1 = key.equals(Clone::clone(&k))?;
                                            if _t1 {
                                                break;
                                            }
                                        } else {
                                            p = e;
                                            binCount = binCount.wrapping_add(1i32);
                                            continue;
                                        }
                                    } else {
                                        p = e;
                                        binCount = binCount.wrapping_add(1i32);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if false {
                            let _t0 = Default::default().putTreeVal(Clone::clone(this), Default::default(), hash, Clone::clone(&key), Clone::clone(&value))?;
                            e = _t0;
                        } else {
                            let mut binCount: i32 = 0i32;
                            loop {
                                e = p.__get_next();
                                if _is_jnull(&p.__get_next()) {
                                    let _t0 = this.newNode(hash, Clone::clone(&key), Clone::clone(&value), Default::default())?;
                                    p.__set_next(Clone::clone(&_t0));
                                    this.treeifyBin(Default::default(), hash)?;
                                } else {
                                    if e.__get_hash() == hash {
                                        k = e.__get_key();
                                        if !_is_jnull(&key) {
                                            let _t0 = key.equals(Clone::clone(&k))?;
                                            if _t0 {
                                                break;
                                            }
                                        } else {
                                            p = e;
                                            binCount = binCount.wrapping_add(1i32);
                                            continue;
                                        }
                                    } else {
                                        p = e;
                                        binCount = binCount.wrapping_add(1i32);
                                        continue;
                                    }
                                }
                                if (panic!("stack underflow") as i32) < (panic!("stack underflow") as i32) { break; }
                                this.treeifyBin(Default::default(), hash)?;
                                break;
                                if e.__get_hash() == hash {
                                    k = e.__get_key();
                                    if !_is_jnull(&key) {
                                        let _t0 = key.equals(Clone::clone(&k))?;
                                        if _t0 {
                                            break;
                                        }
                                    } else {
                                        p = e;
                                        binCount = binCount.wrapping_add(1i32);
                                        continue;
                                    }
                                } else {
                                    p = e;
                                    binCount = binCount.wrapping_add(1i32);
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    if false {
                        let _t0 = Default::default().putTreeVal(Clone::clone(this), Default::default(), hash, Clone::clone(&key), Clone::clone(&value))?;
                        e = _t0;
                    } else {
                        let mut binCount: i32 = 0i32;
                        loop {
                            e = p.__get_next();
                            if _is_jnull(&p.__get_next()) {
                                let _t0 = this.newNode(hash, Clone::clone(&key), Clone::clone(&value), Default::default())?;
                                p.__set_next(Clone::clone(&_t0));
                                this.treeifyBin(Default::default(), hash)?;
                            } else {
                                if e.__get_hash() == hash {
                                    let mut k = e.__get_key();
                                    if !_is_jnull(&key) {
                                        let _t0 = key.equals(Clone::clone(&k))?;
                                        if _t0 {
                                            break;
                                        }
                                    } else {
                                        p = e;
                                        binCount = binCount.wrapping_add(1i32);
                                        continue;
                                    }
                                } else {
                                    p = e;
                                    binCount = binCount.wrapping_add(1i32);
                                    continue;
                                }
                            }
                            if (panic!("stack underflow") as i32) < (panic!("stack underflow") as i32) { break; }
                            this.treeifyBin(Default::default(), hash)?;
                            break;
                            if e.__get_hash() == hash {
                                let mut k = e.__get_key();
                                if !_is_jnull(&key) {
                                    let _t0 = key.equals(Clone::clone(&k))?;
                                    if _t0 {
                                        break;
                                    }
                                } else {
                                    p = e;
                                    binCount = binCount.wrapping_add(1i32);
                                    continue;
                                }
                            } else {
                                p = e;
                                binCount = binCount.wrapping_add(1i32);
                                continue;
                            }
                        }
                    }
                }
                let mut binCount = e.__get_value();
                if _is_jnull(&binCount) {
                    e.__set_value(Clone::clone(&value));
                }
                this.afterNodeAccess(Clone::clone(&e))?;
                return Ok(panic!("null"));
            }
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            this.__set_size((this.__get_size()).wrapping_add(1i32));
            if (this.__get_size()).wrapping_add(1i32) > this.__get_threshold() {
                let _t0 = this.resize()?;
            }
            this.afterNodeInsertion(evict)?;
            Ok(panic!("null"))
        }

        #[java_method(name = "resize", descriptor = "()[Ljava/util/HashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/util/HashMap$Node<TK;TV;>;")]
        pub fn resize(&self) -> Result<Rc<RefCell<Vec<HashMap_Node<K, V>>>>> {
            let this = self;
            let mut oldTab = this.__get_table();
            let mut oldCap = (if _is_jnull(&oldTab) { 0i32 } else { (oldTab.borrow().len() as i32) });
            let mut oldThr = this.__get_threshold();
            let mut newThr: i32 = 0i32;
        let mut newCap = Default::default();
            if (oldCap>0) {
                if oldCap >= 1073741824i32 {
                    this.__set_threshold(2147483647i32);
                    return Ok(Default::default());
                }
                newCap = (oldCap<<(1i32&0x1f));
                newThr = (oldThr<<(1i32&0x1f));
            } else {
                if (oldThr>0) {
                    newCap = oldThr;
                } else {
                    newCap = 16i32;
                    newThr = 12i32;
                }
            }
            let mut ft = ((newCap as f32)*this.__get_loadFactor());
            newThr = (if newCap < 1073741824i32 { (if (((ft>(1073741824.0f32)) as i32-((ft)<(1073741824.0f32)) as i32)<0) { (ft as i32) } else { 2147483647i32 }) } else { 2147483647i32 });
            this.__set_threshold(newThr);
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); newCap as usize]));
            let mut ft: Rc<RefCell<Vec<Object>>> = _arr0;
            this.__set_table(Clone::clone(&ft));
            let mut j: i32 = 0i32;
            loop {
                if j >= oldCap { break; }
                let mut e = Clone::clone(&oldTab.borrow()[j as usize]);
                oldTab.borrow_mut()[j as usize] = Default::default();
                if _is_jnull(&e.__get_next()) {
                    ft.borrow_mut()[(e.__get_hash()&(newCap).wrapping_sub(1i32)) as usize] = Object::from_any(e.clone());
                } else {
        let mut hiTail: HashMap_Node<K, V> = Default::default();
        let mut hiHead: HashMap_Node<K, V> = Default::default();
        let mut loTail: HashMap_Node<K, V> = Default::default();
        let mut loHead: HashMap_Node<K, V> = Default::default();
                    if false {
                        Default::default().split(Clone::clone(this), Default::default(), j, oldCap)?;
                    } else {
                        loHead = Default::default();
                        loTail = Default::default();
                        hiHead = Default::default();
                        hiTail = Default::default();
                        loop {
                            let mut next = e.__get_next();
                            if ((e.__get_hash()&oldCap)==0) {
                                if _is_jnull(&loTail) {
                                    loHead = e;
                                } else {
                                    loTail.__set_next(Clone::clone(&e));
                                }
                                loTail = e;
                            } else {
                                if _is_jnull(&hiTail) {
                                    hiHead = e;
                                } else {
                                    hiTail.__set_next(Clone::clone(&e));
                                }
                                hiTail = e;
                            }
                            e = next;
                            if _is_jnull(&e) { break; }
                        }
                        if !_is_jnull(&loTail) {
                            loTail.__set_next(Default::default());
                            ft.borrow_mut()[j as usize] = Object::from_any(loHead.clone());
                        }
                        if !_is_jnull(&hiTail) {
                            hiTail.__set_next(Default::default());
                            ft.borrow_mut()[(j).wrapping_add(oldCap) as usize] = Object::from_any(hiHead.clone());
                        }
                    }
                }
                j = j.wrapping_add(1i32);
            }
            Ok(Default::default())
        }

        #[java_method(name = "treeifyBin", descriptor = "([Ljava/util/HashMap$Node;I)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/HashMap$Node<TK;TV;>;I)V")]
        pub fn treeifyBin(&self, mut tab: Rc<RefCell<Vec<HashMap_Node<K, V>>>>, mut hash: i32) -> Result<()> {
            let this = self;
            let mut n = (tab.borrow().len() as i32);
        let mut hd: HashMap_TreeNode<K, V> = Default::default();
            if (tab.borrow().len() as i32) < 64i32 {
                let _t0 = this.resize()?;
            } else {
                let mut index = ((n).wrapping_sub(1i32)&hash);
                let mut e = Clone::clone(&tab.borrow()[((n).wrapping_sub(1i32)&hash) as usize]);
                hd = Default::default();
                let mut tl: HashMap_TreeNode<K, V> = Default::default();
                loop {
                    let _t0 = this.replacementTreeNode(Clone::clone(&e), Default::default())?;
                    let mut p: HashMap_TreeNode<Object, Object> = _t0;
                    if _is_jnull(&tl) {
                        hd = p;
                    } else {
                        p.__set_prev(Clone::clone(&tl));
                        tl.__set_next(Clone::clone(&<_ as Into<HashMap_Node<Object, Object>>>::into(p)));
                    }
                    let mut tl: HashMap_TreeNode<Object, Object> = p;
                    let mut e = e.__get_next();
                    if _is_jnull(&e.__get_next()) { break; }
                }
                tab.borrow_mut()[index as usize] = Clone::clone(&hd);
                if !_is_jnull(&hd) {
                    hd.treeify(Clone::clone(&tab))?;
                }
            }
            Ok(())
        }

        #[java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V")]
        pub fn putAll(&self, m: Object) -> Result<()> {
            panic!("stub: java/util/HashMap.putAll:(Ljava/util/Map;)V")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn remove_obj(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/HashMap.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "removeNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/util/HashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/util/HashMap$Node<TK;TV;>;")]
        pub fn removeNode(&self, hash: i32, key: Object, value: Object, matchValue: bool, movable: bool) -> Result<HashMap_Node<Object, Object>> {
            panic!("stub: java/util/HashMap.removeNode:(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/util/HashMap$Node;")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/HashMap.clear:()V")
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/HashMap.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<TK;>;")]
        pub fn keySet(&self) -> Result<Object> {
            let this = self;
            let mut ks = this.__get_keySet();
            if _is_jnull(&ks) {
                let mut ks = HashMap_KeySet::new(Clone::clone(this))?;
                this.__set_keySet(Object::from_any(ks.clone()));
            }
            Ok(Object::from_any(ks.clone()))
        }

        #[java_method(name = "prepareArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        pub fn prepareArray(&self, mut a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            let this = self;
            let mut size = this.__get_size();
            if (a.borrow().len() as i32) < size {
                let _t0: Object = Object::default();
                let _vdispatch1: Object = if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _t2: Object = Array::newInstance_class_i(Clone::clone(&_vdispatch1), size)?;
                return Ok((_t2).downcast::<Rc<RefCell<Vec<Object>>>>());
            }
            if (a.borrow().len() as i32) > size {
                a.borrow_mut()[size as usize] = Clone::clone(&Object::default());
            }
            Ok(a)
        }

        #[java_method(name = "keysToArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        pub fn keysToArray(&self, mut a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            let this = self;
            let mut r: Rc<RefCell<Vec<Object>>> = a;
            let mut idx: i32 = 0i32;
            let mut tab = this.__get_table();
            let mut local_5: Rc<RefCell<Vec<HashMap_Node<Object, Object>>>> = tab;
            let mut local_6 = (local_5.borrow().len() as i32);
            let mut local_7: i32 = 0i32;
            loop {
                if local_7 >= local_6 { break; }
                let mut e = Clone::clone(&local_5.borrow()[local_7 as usize]);
                loop {
                    if _is_jnull(&e) { break; }
                    idx = idx.wrapping_add(1i32);
                    r.borrow_mut()[idx as usize] = Clone::clone(&e.__get_key());
                    e = e.__get_next();
                }
                local_7 = local_7.wrapping_add(1i32);
            }
            Ok(a)
        }

        #[java_method(name = "valuesToArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        pub fn valuesToArray(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/HashMap.valuesToArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
        }

        #[java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;")]
        pub fn values(&self) -> Result<Object> {
            panic!("stub: java/util/HashMap.values:()Ljava/util/Collection;")
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            let this = self;
            let mut es = this.__get_entrySet();
            let mut _merged0: HashMap_EntrySet;
            if _is_jnull(&this.__get_entrySet()) {
                this.__set_entrySet(Object::from_any(HashMap_EntrySet::new(Clone::clone(this))?.clone()));
                _merged0 = HashMap_EntrySet::new(Clone::clone(this))?;
            } else {
                _merged0 = (es).downcast::<HashMap_EntrySet>();
            }
            Ok(Object::from_any(_merged0.clone()))
        }

        #[java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;TV;)TV;")]
        pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<Object> {
            panic!("stub: java/util/HashMap.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn putIfAbsent(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/HashMap.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove_obj_obj(&self, key: Object, value: Object) -> Result<bool> {
            panic!("stub: java/util/HashMap.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;TV;)Z")]
        pub fn replace_obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
            panic!("stub: java/util/HashMap.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn replace_obj_obj(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: java/util/HashMap.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/Function<-TK;+TV;>;)TV;")]
        pub fn computeIfAbsent(&self, mut key: K, mut mappingFunction: Object) -> Result<V> {
            let this = self;
            if _is_jnull(&mappingFunction) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: i32 = HashMap::<Object, Object>::hash(Clone::clone(&key))?;
            let mut hash: i32 = _t0;
            let mut binCount: i32 = 0i32;
            let mut t: HashMap_TreeNode<K, V> = Default::default();
            let mut old: HashMap_Node<K, V> = Default::default();
            let mut tab = this.__get_table();
            let mut n = (tab.borrow().len() as i32);
            if ((tab.borrow().len() as i32)==0) {
                let _t1 = this.resize()?;
                tab = _t1;
                n = (tab.borrow().len() as i32);
            }
            let mut i = ((n).wrapping_sub(1i32)&hash);
            let mut first = Clone::clone(&tab.borrow()[((n).wrapping_sub(1i32)&hash) as usize]);
            if false {
                let mut t: HashMap_TreeNode<Object, Object> = Default::default();
                let _t1 = Default::default().getTreeNode(hash, Clone::clone(&key))?;
                let mut old: HashMap_TreeNode<Object, Object> = _t1;
            } else {
                let mut e: HashMap_Node<Object, Object> = first;
                loop {
                    if e.__get_hash() == hash {
                        let mut k = e.__get_key();
                        if !_is_jnull(&key) {
                            let _t1 = key.equals(Clone::clone(&k))?;
                            if _t1 {
                                let mut old: HashMap_Node<Object, Object> = e;
                                break;
                            }
                        } else {
                            binCount = binCount.wrapping_add(1i32);
                            e = e.__get_next();
                        }
                    } else {
                        binCount = binCount.wrapping_add(1i32);
                        e = e.__get_next();
                    }
                    if _is_jnull(&(panic!("stack underflow") as i32)) { break; }
                }
            }
            let mut e = old.__get_value();
            if !_is_jnull(&old.__get_value()) {
                this.afterNodeAccess(Clone::clone(&old).into())?;
                return Ok(Default::default());
            }
            let mut e = this.__get_modCount();
            let _vdispatch1: Object = if let Some(__f) = mappingFunction.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&key))? } else { Default::default() };
            let mut k: K = _vdispatch1;
            if e != this.__get_modCount() {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if _is_jnull(&k) {
                return Ok(panic!("null"));
            }
            if !_is_jnull(&old) {
                old.__set_value(Clone::clone(&k));
                this.afterNodeAccess(Clone::clone(&old).into())?;
                return Ok(Default::default());
            }
            if !_is_jnull(&t) {
                let _t2 = t.putTreeVal(Clone::clone(this), Default::default(), hash, Clone::clone(&key), Clone::clone(&k))?;
            } else {
                let _t2 = this.newNode(hash, Clone::clone(&key), Clone::clone(&k), Clone::clone(&first))?;
                tab.borrow_mut()[i as usize] = Clone::clone(&_t2);
                if binCount >= 7i32 {
                    this.treeifyBin(Default::default(), hash)?;
                }
            }
            this.__set_modCount((e).wrapping_add(1i32));
            this.__set_size((this.__get_size()).wrapping_add(1i32));
            this.afterNodeInsertion((1i32 != 0i32))?;
            Ok(Default::default())
        }

        #[java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/HashMap.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;")]
        pub fn compute(&self, key: K, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/HashMap.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;")]
        pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/HashMap.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TK;-TV;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/HashMap.forEach:(Ljava/util/function/BiConsumer;)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)V")]
        pub fn replaceAll(&self, function: Object) -> Result<()> {
            panic!("stub: java/util/HashMap.replaceAll:(Ljava/util/function/BiFunction;)V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/HashMap.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "loadFactor", descriptor = "()F", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadFactor(&self) -> Result<f32> {
            panic!("stub: java/util/HashMap.loadFactor:()F")
        }

        #[java_method(name = "capacity", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn capacity(&self) -> Result<i32> {
            panic!("stub: java/util/HashMap.capacity:()I")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/HashMap.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/HashMap.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "newNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$Node<TK;TV;>;")]
        pub fn newNode(&self, mut hash: i32, mut key: K, mut value: V, mut next: HashMap_Node<K, V>) -> Result<HashMap_Node<K, V>> {
            let this = self;
            Ok(Default::default())
        }

        #[java_method(name = "replacementNode", descriptor = "(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$Node<TK;TV;>;")]
        pub fn replacementNode(&self, mut p: HashMap_Node<K, V>, mut next: HashMap_Node<K, V>) -> Result<HashMap_Node<K, V>> {
            let this = self;
            Ok(Default::default())
        }

        #[java_method(name = "newTreeNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn newTreeNode(&self, mut hash: i32, mut key: K, mut value: V, mut next: HashMap_Node<K, V>) -> Result<HashMap_TreeNode<K, V>> {
            let this = self;
            Ok(Default::default())
        }

        #[java_method(name = "replacementTreeNode", descriptor = "(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn replacementTreeNode(&self, mut p: HashMap_Node<K, V>, mut next: HashMap_Node<K, V>) -> Result<HashMap_TreeNode<K, V>> {
            let this = self;
            Ok(Default::default())
        }

        #[java_method(name = "reinitialize", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reinitialize(&self) -> Result<()> {
            panic!("stub: java/util/HashMap.reinitialize:()V")
        }

        #[java_method(name = "afterNodeAccess", descriptor = "(Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;)V")]
        pub fn afterNodeAccess(&self, mut p: HashMap_Node<K, V>) -> Result<()> {
            let this = self;
            Ok(())
        }

        #[java_method(name = "afterNodeInsertion", descriptor = "(Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn afterNodeInsertion(&self, mut evict: bool) -> Result<()> {
            let this = self;
            Ok(())
        }

        #[java_method(name = "afterNodeRemoval", descriptor = "(Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;)V")]
        pub fn afterNodeRemoval(&self, p: HashMap_Node<K, V>) -> Result<()> {
            panic!("stub: java/util/HashMap.afterNodeRemoval:(Ljava/util/HashMap$Node;)V")
        }

        #[java_method(name = "internalWriteEntries", descriptor = "(Ljava/io/ObjectOutputStream;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn internalWriteEntries(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/HashMap.internalWriteEntries:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "calculateHashMapCapacity", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn calculateHashMapCapacity(mut numMappings: i32) -> Result<i32> {
            let _t0: f64 = Math::ceil(((numMappings as f64)/0.75f64))?;
            Ok((_t0 as i32))
        }

        #[java_method(name = "newHashMap", descriptor = "(I)Ljava/util/HashMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(I)Ljava/util/HashMap<TK;TV;>;")]
        pub fn newHashMap(mut numMappings: i32) -> Result<HashMap<K, V>> {
            if (numMappings<0) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Negative number of mappings: ")))?;
                let _t1 = _t0.append_i(numMappings)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: i32 = HashMap::<Object, Object>::calculateHashMapCapacity(numMappings)?;
            Ok(Default::default())
        }
    }
}
