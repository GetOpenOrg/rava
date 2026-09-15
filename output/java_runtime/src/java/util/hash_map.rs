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
    binary_name       = "java/util/HashMap",
    super_class       = "java/util/AbstractMap",
    interfaces        = "java/util/Map,java/lang/Cloneable,java/io/Serializable",
    access            = "public",
    modifiers         = "",
    generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/AbstractMap<TK;TV;>;Ljava/util/Map<TK;TV;>;Ljava/lang/Cloneable;Ljava/io/Serializable;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "HashMap.java",
    inner_classes     = "java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/HashMap$TreeNode:java/util/HashMap:TreeNode:24;java/util/HashMap$KeySet:java/util/HashMap:KeySet:16;java/util/HashMap$Values:java/util/HashMap:Values:16;java/util/HashMap$EntrySet:java/util/HashMap:EntrySet:16;java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/util/HashMap$UnsafeHolder:java/util/HashMap:UnsafeHolder:26;java/util/HashMap$EntrySpliterator:java/util/HashMap:EntrySpliterator:24;java/util/HashMap$ValueSpliterator:java/util/HashMap:ValueSpliterator:24;java/util/HashMap$KeySpliterator:java/util/HashMap:KeySpliterator:24;java/util/HashMap$HashMapSpliterator:java/util/HashMap:HashMapSpliterator:8;java/util/HashMap$EntryIterator:java/util/HashMap:EntryIterator:16;java/util/HashMap$ValueIterator:java/util/HashMap:ValueIterator:16;java/util/HashMap$KeyIterator:java/util/HashMap:KeyIterator:16;java/util/HashMap$HashIterator:java/util/HashMap:HashIterator:1024",
    all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/AbstractMap;java/util/HashMap;java/util/Map",
)]
#[derive(Clone, Default, PartialEq)]
pub struct HashMap<K: Clone + Default + 'static, V: Clone + Default + 'static> {
    pub _super: AbstractMap<K, V>,
    #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/util/HashMap$Node;", access = "package", modifiers = "transient", is_static = false, generic_signature = "[Ljava/util/HashMap$Node<TK;TV;>;"))]
    pub table: JField<Rc<RefCell<Vec<HashMap_Node<K, V>>>>>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;"))]
    pub entrySet: JField<Object>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
    pub size: JField<i32>,
    #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
    pub modCount: JField<i32>,
    #[cfg_attr(any(), java_field(name = "threshold", descriptor = "I", is_static = false))]
    pub threshold: JField<i32>,
    #[cfg_attr(any(), java_field(name = "loadFactor", descriptor = "F", access = "package", modifiers = "final", is_static = false))]
    pub loadFactor: JField<f32>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> HashMap<K, V> {
    pub fn as_abstract_map(&self) -> &AbstractMap<K, V> { &self._super }
    pub fn into_abstract_map(self) -> AbstractMap<K, V> { self._super }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<HashMap<K, V>> for AbstractMap<K, V> {
    fn from(v: HashMap<K, V>) -> AbstractMap<K, V> { v._super }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> HashMap<K, V> {
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

    #[cfg_attr(any(), java_method(name = "hash", descriptor = "(Ljava/lang/Object;)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hash(mut key: Object) -> Result<i32> {
        let mut _merged1: i32;
        if _is_jnull(&key) {
            _merged1 = 0i32;
        } else {
            let _t0 = key.hashCode()?;
            let mut h: i32 = _t0;
            _merged1 = (h^((h as u32>>(16i32&0x1f)) as i32));
        }
        Ok(_merged1)
    }

    #[cfg_attr(any(), java_method(name = "comparableClassFor", descriptor = "(Ljava/lang/Object;)Ljava/lang/Class;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)Ljava/lang/Class<*>;"))]
    pub fn comparableClassFor(mut x: Object) -> Result<Object> {
        let _t0 = x.getClass()?;
        let mut c = (_t0).downcast::<Class<Object>>();
        if c == Object::default() {
            return Ok(Object::from_any(c.clone()));
        }
        let _t1 = c.getGenericInterfaces()?;
        let mut ts: Rc<RefCell<Vec<Object>>> = _t1;
        let mut local_5: Rc<RefCell<Vec<Object>>> = ts;
        let mut local_6 = (local_5.borrow().len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut t = Clone::clone(&local_5.borrow()[local_7 as usize]);
            let mut p = (t).downcast::<ParameterizedType>();
            let _t2 = (t).downcast::<ParameterizedType>().getRawType()?;
            let _t3 = p.getActualTypeArguments()?;
            let mut as_: Rc<RefCell<Vec<Object>>> = _t3;
            if Clone::clone(&as_.borrow()[0i32 as usize]) == Object::from_any(c.clone()) {
                return Ok(Object::from_any(c.clone()));
            }
            local_7 = local_7.wrapping_add(1i32);
        }
        Ok(Object::default())
    }

    #[cfg_attr(any(), java_method(name = "compareComparables", descriptor = "(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Object;Ljava/lang/Object;)I"))]
    pub fn compareComparables(mut kc: Object, mut k: Object, mut x: Object) -> Result<i32> {
        let _t0 = x.getClass()?;
        let mut _merged2: i32;
        if _t0 != kc {
            _merged2 = 0i32;
        } else {
            let _t1 = (k).downcast::<Comparable<Object>>().compareTo(Clone::clone(&x))?;
            _merged2 = _t1;
        }
        Ok(_merged2)
    }

    #[cfg_attr(any(), java_method(name = "tableSizeFor", descriptor = "(I)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn tableSizeFor(cap: i32) -> Result<i32> {
        panic!("stub: java/util/HashMap.tableSizeFor:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_i_f(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
        panic!("stub: java/util/HashMap.<init>:(IF)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_i(initialCapacity: i32) -> Result<Self> {
        panic!("stub: java/util/HashMap.<init>:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), table: JField::new(Default::default()), entrySet: JField::new(Default::default()), size: JField::new(0), modCount: JField::new(0), threshold: JField::new(0), loadFactor: JField::new(0.0), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = AbstractMap::new()?;
        this.loadFactor.set(0.75f32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V"))]
    pub fn new_map(m: Object) -> Result<Self> {
        panic!("stub: java/util/HashMap.<init>:(Ljava/util/Map;)V")
    }

    #[cfg_attr(any(), java_method(name = "putMapEntries", descriptor = "(Ljava/util/Map;Z)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;Z)V"))]
    pub fn putMapEntries(&self, m: Object, evict: bool) -> Result<()> {
        panic!("stub: java/util/HashMap.putMapEntries:(Ljava/util/Map;Z)V")
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/HashMap.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;"))]
    pub fn get(&self, mut key: Object) -> Result<V> {
        let this = self;
        let _t0 = this.getNode(Clone::clone(&key))?;
        let mut e: HashMap_Node<Object, Object> = _t0;
        Ok(panic!("null"))
    }

    #[cfg_attr(any(), java_method(name = "getNode", descriptor = "(Ljava/lang/Object;)Ljava/util/HashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)Ljava/util/HashMap$Node<TK;TV;>;"))]
    pub fn getNode(&self, mut key: Object) -> Result<HashMap_Node<Object, Object>> {
        let this = self;
        let mut tab = this.table.get();
        let mut n = (tab.borrow().len() as i32);
        let _t0: i32 = HashMap::<Object, Object>::hash(Clone::clone(&key))?;
        let mut hash: i32 = _t0;
        let mut first = Clone::clone(&tab.borrow()[((n).wrapping_sub(1i32)&hash) as usize]);
        let mut k = first.key.get();
        let _t1 = key.equals(Clone::clone(&k))?;
        if _t1 {
            return Ok(first);
        }
        let mut e = first.next.get();
        if false {
            let _t2 = Default::default().getTreeNode(hash, Clone::clone(&key))?;
            return Ok(<_ as Into<HashMap_Node<Object, Object>>>::into(_t2));
        }
        loop {
            k = e.key.get();
            let _t2 = key.equals(Clone::clone(&k))?;
            if _t2 {
                return Ok(e);
            }
            e = e.next.get();
            if _is_jnull(&e.next.get()) { break; }
        }
        Ok(Default::default())
    }

    #[cfg_attr(any(), java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn containsKey(&self, mut key: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.getNode(Clone::clone(&key))?;
        Ok(!_is_jnull(&_t0))
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;"))]
    pub fn put(&self, mut key: K, mut value: V) -> Result<V> {
        let this = self;
        let _t0: i32 = HashMap::<Object, Object>::hash(Clone::clone(&key))?;
        let _t1 = this.putVal(_t0, Clone::clone(&key), Clone::clone(&value), (0i32 != 0i32), (1i32 != 0i32))?;
        Ok(panic!("null"))
    }

    #[cfg_attr(any(), java_method(name = "putVal", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;ZZ)TV;"))]
    pub fn putVal(&self, mut hash: i32, mut key: K, mut value: V, mut onlyIfAbsent: bool, mut evict: bool) -> Result<V> {
        let this = self;
        let mut tab = this.table.get();
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
            if p.hash.get() == hash {
                let mut k = p.key.get();
                if !_is_jnull(&key) {
                    let _t0 = key.equals(Clone::clone(&k))?;
    e = Default::default();
                    if _t0 {
                        e = p;
                    } else {
                        if false {
                            let _t1 = Default::default().putTreeVal(Clone::clone(&this), Default::default(), hash, Clone::clone(&key), Clone::clone(&value))?;
                            e = _t1;
                        } else {
                            let mut binCount: i32 = 0i32;
                            loop {
                                e = p.next.get();
                                if _is_jnull(&p.next.get()) {
                                    let _t1 = this.newNode(hash, Clone::clone(&key), Clone::clone(&value), Default::default())?;
                                    p.next.set(Clone::clone(&_t1));
                                    this.treeifyBin(Default::default(), hash)?;
                                } else {
                                    if e.hash.get() == hash {
                                        k = e.key.get();
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
                                if e.hash.get() == hash {
                                    k = e.key.get();
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
                        let _t0 = Default::default().putTreeVal(Clone::clone(&this), Default::default(), hash, Clone::clone(&key), Clone::clone(&value))?;
                        e = _t0;
                    } else {
                        let mut binCount: i32 = 0i32;
                        loop {
                            e = p.next.get();
                            if _is_jnull(&p.next.get()) {
                                let _t0 = this.newNode(hash, Clone::clone(&key), Clone::clone(&value), Default::default())?;
                                p.next.set(Clone::clone(&_t0));
                                this.treeifyBin(Default::default(), hash)?;
                            } else {
                                if e.hash.get() == hash {
                                    k = e.key.get();
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
                            if e.hash.get() == hash {
                                k = e.key.get();
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
                    let _t0 = Default::default().putTreeVal(Clone::clone(&this), Default::default(), hash, Clone::clone(&key), Clone::clone(&value))?;
                    e = _t0;
                } else {
                    let mut binCount: i32 = 0i32;
                    loop {
                        e = p.next.get();
                        if _is_jnull(&p.next.get()) {
                            let _t0 = this.newNode(hash, Clone::clone(&key), Clone::clone(&value), Default::default())?;
                            p.next.set(Clone::clone(&_t0));
                            this.treeifyBin(Default::default(), hash)?;
                        } else {
                            if e.hash.get() == hash {
                                let mut k = e.key.get();
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
                        if e.hash.get() == hash {
                            let mut k = e.key.get();
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
            let mut binCount = e.value.get();
            if _is_jnull(&binCount) {
                e.value.set(Clone::clone(&value));
            }
            this.afterNodeAccess(Clone::clone(&e))?;
            return Ok(panic!("null"));
        }
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.size.set((this.size.get()).wrapping_add(1i32));
        if (this.size.get()).wrapping_add(1i32) > this.threshold.get() {
            let _t0 = this.resize()?;
        }
        this.afterNodeInsertion(evict)?;
        Ok(panic!("null"))
    }

    #[cfg_attr(any(), java_method(name = "resize", descriptor = "()[Ljava/util/HashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/util/HashMap$Node<TK;TV;>;"))]
    pub fn resize(&self) -> Result<Rc<RefCell<Vec<HashMap_Node<Object, Object>>>>> {
        let this = self;
        let mut oldTab = this.table.get();
        let mut oldCap = (if _is_jnull(&oldTab) { 0i32 } else { (oldTab.borrow().len() as i32) });
        let mut oldThr = this.threshold.get();
        let mut newThr: i32 = 0i32;
    let mut newCap = Default::default();
        if (oldCap>0) {
            if oldCap >= 1073741824i32 {
                this.threshold.set(2147483647i32);
                return Ok(oldTab);
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
        let mut ft = ((newCap as f32)*this.loadFactor.get());
        newThr = (if newCap < 1073741824i32 { (if (((ft>(1073741824.0f32)) as i32-((ft)<(1073741824.0f32)) as i32)<0) { (ft as i32) } else { 2147483647i32 }) } else { 2147483647i32 });
        this.threshold.set(newThr);
        let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); newCap as usize]));
        let mut ft: Rc<RefCell<Vec<Object>>> = _arr0;
        this.table.set(Clone::clone(&ft));
        let mut j: i32 = 0i32;
        loop {
            if j >= oldCap { break; }
            let mut e = Clone::clone(&oldTab.borrow()[j as usize]);
            oldTab.borrow_mut()[j as usize] = Default::default();
            if _is_jnull(&e.next.get()) {
                ft.borrow_mut()[(e.hash.get()&(newCap).wrapping_sub(1i32)) as usize] = Object::from_any(e.clone());
            } else {
    let mut hiTail: HashMap_Node<K, V> = Default::default();
    let mut hiHead: HashMap_Node<K, V> = Default::default();
    let mut loTail: HashMap_Node<K, V> = Default::default();
    let mut loHead: HashMap_Node<K, V> = Default::default();
                if false {
                    Default::default().split(Clone::clone(&this), Default::default(), j, oldCap)?;
                } else {
                    loHead = Default::default();
                    loTail = Default::default();
                    hiHead = Default::default();
                    hiTail = Default::default();
                    let mut next = Default::default();
                    loop {
                        next = e.next.get();
                        if ((e.hash.get()&oldCap)==0) {
                            if _is_jnull(&loTail) {
                                loHead = e;
                            } else {
                                loTail.next.set(Clone::clone(&e));
                            }
                            loTail = e;
                        } else {
                            if _is_jnull(&hiTail) {
                                hiHead = e;
                            } else {
                                hiTail.next.set(Clone::clone(&e));
                            }
                            hiTail = e;
                        }
                        e = next;
                        if _is_jnull(&e) { break; }
                    }
                    if !_is_jnull(&loTail) {
                        loTail.next.set(Default::default());
                        ft.borrow_mut()[j as usize] = Object::from_any(loHead.clone());
                    }
                    if !_is_jnull(&hiTail) {
                        hiTail.next.set(Default::default());
                        ft.borrow_mut()[(j).wrapping_add(oldCap) as usize] = Object::from_any(hiHead.clone());
                    }
                }
            }
            j = j.wrapping_add(1i32);
        }
        Ok(Default::default())
    }

    #[cfg_attr(any(), java_method(name = "treeifyBin", descriptor = "([Ljava/util/HashMap$Node;I)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/HashMap$Node<TK;TV;>;I)V"))]
    pub fn treeifyBin(&self, mut tab: Rc<RefCell<Vec<HashMap_Node<Object, Object>>>>, mut hash: i32) -> Result<()> {
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
                    p.prev.set(Clone::clone(&tl));
                    tl._super._super.next.set(Clone::clone(&p));
                }
                let mut tl: HashMap_TreeNode<Object, Object> = p;
                e = e.next.get();
                if _is_jnull(&e.next.get()) { break; }
            }
            tab.borrow_mut()[index as usize] = Clone::clone(&hd);
            if !_is_jnull(&hd) {
                hd.treeify(Default::default())?;
            }
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V"))]
    pub fn putAll(&self, m: Object) -> Result<()> {
        panic!("stub: java/util/HashMap.putAll:(Ljava/util/Map;)V")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;"))]
    pub fn remove_obj(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "removeNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/util/HashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/util/HashMap$Node<TK;TV;>;"))]
    pub fn removeNode(&self, hash: i32, key: Object, value: Object, matchValue: bool, movable: bool) -> Result<HashMap_Node<Object, Object>> {
        panic!("stub: java/util/HashMap.removeNode:(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/util/HashMap$Node;")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/HashMap.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn containsValue(&self, value: Object) -> Result<bool> {
        panic!("stub: java/util/HashMap.containsValue:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<TK;>;"))]
    pub fn keySet(&self) -> Result<Object> {
        panic!("stub: java/util/HashMap.keySet:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "prepareArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn prepareArray(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/HashMap.prepareArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "keysToArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn keysToArray(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/HashMap.keysToArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "valuesToArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn valuesToArray(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/HashMap.valuesToArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;"))]
    pub fn values(&self) -> Result<Object> {
        panic!("stub: java/util/HashMap.values:()Ljava/util/Collection;")
    }

    #[cfg_attr(any(), java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;"))]
    pub fn entrySet(&self) -> Result<Object> {
        panic!("stub: java/util/HashMap.entrySet:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;TV;)TV;"))]
    pub fn getOrDefault(&self, key: Object, defaultValue: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;"))]
    pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remove_obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        panic!("stub: java/util/HashMap.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;TV;)Z"))]
    pub fn replace_obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
        panic!("stub: java/util/HashMap.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;"))]
    pub fn replace_obj_obj(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/Function<-TK;+TV;>;)TV;"))]
    pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap.computeIfAbsent:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;"))]
    pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;"))]
    pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;"))]
    pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TK;-TV;>;)V"))]
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/HashMap.forEach:(Ljava/util/function/BiConsumer;)V")
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)V"))]
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        panic!("stub: java/util/HashMap.replaceAll:(Ljava/util/function/BiFunction;)V")
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clone(&self) -> Result<Object> {
        panic!("stub: java/util/HashMap.clone:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "loadFactor", descriptor = "()F", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn loadFactor(&self) -> Result<f32> {
        panic!("stub: java/util/HashMap.loadFactor:()F")
    }

    #[cfg_attr(any(), java_method(name = "capacity", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn capacity(&self) -> Result<i32> {
        panic!("stub: java/util/HashMap.capacity:()I")
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn writeObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/HashMap.writeObject:(Ljava/io/ObjectOutputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException"))]
    pub fn readObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/HashMap.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "newNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$Node<TK;TV;>;"))]
    pub fn newNode(&self, mut hash: i32, mut key: K, mut value: V, mut next: HashMap_Node<Object, Object>) -> Result<HashMap_Node<Object, Object>> {
        let this = self;
        Ok(HashMap_Node::<Object, Object>::new(hash, Clone::clone(&key), Clone::clone(&value), Clone::clone(&next))?)
    }

    #[cfg_attr(any(), java_method(name = "replacementNode", descriptor = "(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$Node<TK;TV;>;"))]
    pub fn replacementNode(&self, mut p: HashMap_Node<Object, Object>, mut next: HashMap_Node<Object, Object>) -> Result<HashMap_Node<Object, Object>> {
        let this = self;
        Ok(HashMap_Node::<Object, Object>::new(p.hash.get(), Clone::clone(&p.key.get()), Clone::clone(&p.value.get()), Clone::clone(&next))?)
    }

    #[cfg_attr(any(), java_method(name = "newTreeNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn newTreeNode(&self, mut hash: i32, mut key: K, mut value: V, mut next: HashMap_Node<Object, Object>) -> Result<HashMap_TreeNode<Object, Object>> {
        let this = self;
        Ok(HashMap_TreeNode::<Object, Object>::new(hash, Clone::clone(&key), Clone::clone(&value), Clone::clone(&next))?)
    }

    #[cfg_attr(any(), java_method(name = "replacementTreeNode", descriptor = "(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;Ljava/util/HashMap$Node<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn replacementTreeNode(&self, mut p: HashMap_Node<Object, Object>, mut next: HashMap_Node<Object, Object>) -> Result<HashMap_TreeNode<Object, Object>> {
        let this = self;
        Ok(HashMap_TreeNode::<Object, Object>::new(p.hash.get(), Clone::clone(&p.key.get()), Clone::clone(&p.value.get()), Clone::clone(&next))?)
    }

    #[cfg_attr(any(), java_method(name = "reinitialize", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reinitialize(&self) -> Result<()> {
        panic!("stub: java/util/HashMap.reinitialize:()V")
    }

    #[cfg_attr(any(), java_method(name = "afterNodeAccess", descriptor = "(Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;)V"))]
    pub fn afterNodeAccess(&self, mut p: HashMap_Node<Object, Object>) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "afterNodeInsertion", descriptor = "(Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn afterNodeInsertion(&self, mut evict: bool) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "afterNodeRemoval", descriptor = "(Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap$Node<TK;TV;>;)V"))]
    pub fn afterNodeRemoval(&self, p: HashMap_Node<Object, Object>) -> Result<()> {
        panic!("stub: java/util/HashMap.afterNodeRemoval:(Ljava/util/HashMap$Node;)V")
    }

    #[cfg_attr(any(), java_method(name = "internalWriteEntries", descriptor = "(Ljava/io/ObjectOutputStream;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn internalWriteEntries(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/HashMap.internalWriteEntries:(Ljava/io/ObjectOutputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "calculateHashMapCapacity", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn calculateHashMapCapacity(numMappings: i32) -> Result<i32> {
        panic!("stub: java/util/HashMap.calculateHashMapCapacity:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "newHashMap", descriptor = "(I)Ljava/util/HashMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(I)Ljava/util/HashMap<TK;TV;>;"))]
    pub fn newHashMap(numMappings: i32) -> Result<HashMap<Object, Object>> {
        panic!("stub: java/util/HashMap.newHashMap:(I)Ljava/util/HashMap;")
    }
}
