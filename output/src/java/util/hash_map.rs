#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/HashMap",
    super_class = "java/util/AbstractMap",
    interfaces  = "java/util/Map,java/lang/Cloneable,java/io/Serializable",
    access      = "public",
    source      = "HashMap.java",
))]
pub struct HashMap {
    #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/util/HashMap$Node;", access = ""))]
    pub table: Field<JvmObject>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = ""))]
    pub entrySet: Field<JvmObject>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = ""))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = ""))]
    pub modCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "threshold", descriptor = "I"))]
    pub threshold: Field<i32>,
    #[cfg_attr(any(), java_field(name = "loadFactor", descriptor = "F", access = "final"))]
    pub loadFactor: Field<f32>,
}

impl HashMap {
    #[cfg_attr(any(), java_method(name = "hash", descriptor = "(Ljava/lang/Object;)I", access = "static final"))]
    pub fn hash(key: JvmObject) -> Result<i32> {
        let _t0 = key.hashCode()?;
        let mut h: i32 = _t0;
        Ok((_t0^((h as u32>>(16i32&0x1f)) as i32)))
    }

    #[cfg_attr(any(), java_method(name = "comparableClassFor", descriptor = "(Ljava/lang/Object;)Ljava/lang/Class;", access = "static"))]
    pub fn comparableClassFor(x: JvmObject) -> Result<JvmObject> {
        let _t0 = x.getClass()?;
        let mut c: JvmObject = _t0;
        return Ok(c);
        let _t1 = c.getGenericInterfaces()?;
        let mut ts: JvmObject = _t1;
        let mut local_5: JvmObject = ts;
        let mut local_6: i32 = (local_5.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut t: JvmObject = local_5[local_7 as usize].clone();
            let mut p: JvmObject = t;
            let _t0 = t.getRawType()?;
            let _t1 = p.getActualTypeArguments()?;
            let mut as_: JvmObject = _t1;
            return Ok(c);
            local_7 = local_7.wrapping_add(1i32);
        }
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "compareComparables", descriptor = "(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)I", access = "static"))]
    pub fn compareComparables(kc: JvmObject, k: JvmObject, x: JvmObject) -> Result<i32> {
        let _t0 = x.getClass()?;
        let _t1 = k.compareTo(x)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "tableSizeFor", descriptor = "(I)I", access = "static final"))]
    pub fn tableSizeFor(cap: i32) -> Result<i32> {
        let _t0: i32 = Integer::numberOfLeadingZeros((cap).wrapping_sub(1i32))?;
        let mut n: i32 = ((-1i32 as u32>>(_t0&0x1f)) as i32);
        Ok((n).wrapping_add(1i32))
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IF)V", access = "public"))]
    pub fn new(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
        let this = Self { table: Field::new(Default::default()), entrySet: Field::new(Default::default()), size: Field::new(0), modCount: Field::new(0), threshold: Field::new(0), loadFactor: Field::new(0.0) };
        /* invokespecial Method java/util/AbstractMap.<init>:()V */
        String::new().append(&String::from("Illegal initial capacity:"))?;
        String::new().append(&initialCapacity)?;
        panic!("{}", /* IllegalArgumentException::new(String::new())? */);
        initialCapacity = 1073741824i32;
        /* TODO: fcmpg  */
        let _t0: bool = Float::isNaN(loadFactor)?;
        String::new().append(&String::from("Illegal load factor:"))?;
        String::new().append(&loadFactor)?;
        panic!("{}", /* IllegalArgumentException::new(String::new())? */);
        this.loadFactor.set(loadFactor);
        let _t1: i32 = HashMap::tableSizeFor(initialCapacity)?;
        this.threshold.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    pub fn new(initialCapacity: i32) -> Result<Self> {
        let this = Self { table: Field::new(Default::default()), entrySet: Field::new(Default::default()), size: Field::new(0), modCount: Field::new(0), threshold: Field::new(0), loadFactor: Field::new(0.0) };
        /* invokespecial Method java/util/HashMap.<init>:(IF)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self { table: Field::new(Default::default()), entrySet: Field::new(Default::default()), size: Field::new(0), modCount: Field::new(0), threshold: Field::new(0), loadFactor: Field::new(0.0) };
        /* invokespecial Method java/util/AbstractMap.<init>:()V */
        this.loadFactor.set(0.75f32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Map;)V", access = "public"))]
    pub fn new(m: JvmObject) -> Result<Self> {
        let this = Self { table: Field::new(Default::default()), entrySet: Field::new(Default::default()), size: Field::new(0), modCount: Field::new(0), threshold: Field::new(0), loadFactor: Field::new(0.0) };
        /* invokespecial Method java/util/AbstractMap.<init>:()V */
        this.loadFactor.set(0.75f32);
        this.putMapEntries(m, 0i32)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "putMapEntries", descriptor = "(Ljava/util/Map;Z)V", access = "final"))]
    pub fn putMapEntries(&self, m: JvmObject, evict: bool) -> Result<()> {
        let this = self;
        let _t0 = m.size()?;
        let mut s: i32 = _t0;
        let _t1: f64 = (((s as f64)/(this.loadFactor.get() as f64)) as f64).ceil();
        let mut dt: f64 = _t1;
        /* TODO: dcmpg  */
        let mut t: i32 = 1073741824i32;
        let _t2: i32 = HashMap::tableSizeFor(t)?;
        this.threshold.set(_t2);
        loop {
            if s <= this.threshold.get() { break; }
            let _t0 = this.resize()?;
        }
        let _t3 = m.entrySet()?;
        let _t4 = _t3.iterator()?;
        dt = _t4;
        loop {
            let _t0 = dt.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = dt.next()?;
            let mut e: JvmObject = _t0;
            let _t1 = e.getKey()?;
            t = _t1;
            let _t2 = e.getValue()?;
            let mut value: JvmObject = _t2;
            let _t3: i32 = HashMap::hash(t)?;
            let _t4 = this.putVal(_t3, t, value, 0i32, evict)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(this.size.get()==0i32)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, key: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.getNode(key)?;
        let mut e: JvmObject = _t0;
        /* TODO: aconst_null  */
        Ok(e.value.get())
    }

    #[cfg_attr(any(), java_method(name = "getNode", descriptor = "(Ljava/lang/Object;)Ljava/util/HashMap$Node;", access = "final"))]
    pub fn getNode(&self, key: JvmObject) -> Result<JvmObject> {
        let this = self;
        let mut tab: JvmObject = this.table.get();
        let mut n: i32 = (tab.len() as i32);
        let _t0: i32 = HashMap::hash(key)?;
        let mut hash: i32 = _t0;
        let mut first: JvmObject = tab[((n).wrapping_sub(1i32)&_t0) as usize].clone();
        let mut k: JvmObject = first.key.get();
        let _t1 = key.equals(k)?;
        return Ok(first);
        let mut e: JvmObject = first.next.get();
        let _t2 = first.getTreeNode(hash, key)?;
        return Ok(_t2);
        k = e.key.get();
        let _t3 = key.equals(k)?;
        return Ok(e);
        e = e.next.get();
        /* TODO: aconst_null  */
        Ok(e.next.get())
    }

    #[cfg_attr(any(), java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn containsKey(&self, key: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = this.getNode(key)?;
        Ok(!_t0.is_none())
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn put(&self, key: JvmObject, value: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: i32 = HashMap::hash(key)?;
        let _t1 = this.putVal(_t0, key, value, 0i32, 1i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "putVal", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/lang/Object;", access = "final"))]
    pub fn putVal(&self, hash: i32, key: JvmObject, value: JvmObject, onlyIfAbsent: bool, evict: bool) -> Result<JvmObject> {
        let this = self;
        let mut tab: JvmObject = this.table.get();
        let mut n: i32 = (tab.len() as i32);
        let _t0 = this.resize()?;
        tab = _t0;
        n = (_t0.len() as i32);
        let mut i: i32 = ((n).wrapping_sub(1i32)&hash);
        let mut p: JvmObject = tab[((n).wrapping_sub(1i32)&hash) as usize].clone();
        /* TODO: aconst_null  */
        let _t1 = i.newNode(this, hash, key, value)?;
        tab[((n).wrapping_sub(1i32)&hash) as usize].clone()[tab as usize] = _t1;
        let mut k: JvmObject = p.key.get();
        let _t2 = key.equals(k)?;
        let mut e: JvmObject = p;
        let _t3 = p.putTreeVal(this, tab, hash, key, value)?;
        e = _t3;
        let mut binCount: i32 = 0i32;
        loop {
            e = p.next.get();
            /* TODO: aconst_null  */
            let _t0 = p.newNode(this, hash, key, value)?;
            p.next.get().next.set(_t0);
            if binCount < 7i32 { break; }
            this.treeifyBin(tab, hash)?;
            k = e.key.get();
            let _t0 = key.equals(k)?;
            p = e;
            binCount = binCount.wrapping_add(1i32);
        }
        binCount = e.value.get();
        e.value.set(value);
        this.afterNodeAccess(e)?;
        return Ok(binCount);
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.size.set((this.size.get()).wrapping_add(1i32));
        let _t4 = this.resize()?;
        this.afterNodeInsertion(evict)?;
        /* TODO: aconst_null  */
        Ok(this.threshold.get())
    }

    #[cfg_attr(any(), java_method(name = "resize", descriptor = "()[Ljava/util/HashMap$Node;", access = "final"))]
    pub fn resize(&self) -> Result<JvmObject> {
        let this = self;
        let mut oldTab: JvmObject = this.table.get();
        let mut oldCap: i32 = (oldTab.len() as i32);
        let mut oldThr: i32 = this.threshold.get();
        let mut newThr: i32 = 0i32;
        this.threshold.set(2147483647i32);
        return Ok(oldTab);
        let mut newCap: i32 = (oldCap<<(1i32&0x1f));
        newThr = (oldThr<<(1i32&0x1f));
        newCap = oldThr;
        newCap = 16i32;
        newThr = 12i32;
        let mut ft: f32 = ((newCap as f32)*this.loadFactor.get());
        /* TODO: fcmpg  */
        newThr = 2147483647i32;
        this.threshold.set(newThr);
        let mut _arr0: Vec<JvmObject> = Vec::with_capacity(newCap as usize);
        ft = _arr0;
        this.table.set(ft);
        let mut j: i32 = 0i32;
        loop {
            if j >= oldCap { break; }
            let mut e: JvmObject = oldTab[j as usize].clone();
            /* TODO: aconst_null  */
            oldTab[j as usize].clone()[oldTab as usize] = j;
            ft[(e.hash.get()&(newCap).wrapping_sub(1i32)) as usize] = e;
            e.split(this, ft, j, oldCap)?;
            /* TODO: aconst_null  */
            let mut loHead: bool = true;
            /* TODO: aconst_null  */
            let mut loTail: JvmObject = e;
            /* TODO: aconst_null  */
            let mut hiHead: JvmObject = e.next.get();
            /* TODO: aconst_null  */
            let mut hiTail: i32 = /* UNDERFLOW */;
            let mut next: JvmObject = e.next.get();
            loHead = e;
            loTail.next.set(e);
            loTail = e;
            hiHead = e;
            hiTail.next.set(e);
            hiTail = e;
            e = next;
            /* TODO: aconst_null  */
            loTail.next.set(loTail);
            ft[j as usize] = loHead;
            /* TODO: aconst_null  */
            hiTail.next.set(hiTail);
            ft[(j).wrapping_add(oldCap) as usize] = hiHead;
            j = j.wrapping_add(1i32);
        }
        Ok(ft)
    }

    #[cfg_attr(any(), java_method(name = "treeifyBin", descriptor = "([Ljava/util/HashMap$Node;I)V", access = "final"))]
    pub fn treeifyBin(&self, tab: JvmObject, hash: i32) -> Result<()> {
        let this = self;
        let mut n: i32 = (tab.len() as i32);
        let _t0 = this.resize()?;
        let mut index: i32 = ((n).wrapping_sub(1i32)&hash);
        let mut e: JvmObject = tab[((n).wrapping_sub(1i32)&hash) as usize].clone();
        /* TODO: aconst_null  */
        let mut hd: JvmObject = tab[((n).wrapping_sub(1i32)&hash) as usize].clone();
        /* TODO: aconst_null  */
        let mut tl: i32 = 64i32;
        /* TODO: aconst_null  */
        let _t1 = (tab.len() as i32).replacementTreeNode(this, e)?;
        let mut p: JvmObject = _t1;
        hd = p;
        p.prev.set(tl);
        tl.next.set(p);
        tl = p;
        e = e.next.get();
        /* TODO: dup_x2  */
        tab[index as usize] = hd;
        hd.treeify(tab)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public"))]
    pub fn putAll(&self, m: JvmObject) -> Result<()> {
        let this = self;
        this.putMapEntries(m, 1i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn remove(&self, key: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: i32 = HashMap::hash(key)?;
        /* TODO: aconst_null  */
        let _t1 = /* UNDERFLOW */.removeNode(this, _t0, key, 0i32, 1i32)?;
        let mut e: JvmObject = _t1;
        /* TODO: aconst_null  */
        Ok(e.value.get())
    }

    #[cfg_attr(any(), java_method(name = "removeNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/util/HashMap$Node;", access = "final"))]
    pub fn removeNode(&self, hash: i32, key: JvmObject, value: JvmObject, matchValue: bool, movable: bool) -> Result<JvmObject> {
        let this = self;
        let mut tab: JvmObject = this.table.get();
        let mut n: i32 = (tab.len() as i32);
        let mut index: i32 = ((n).wrapping_sub(1i32)&hash);
        let mut p: JvmObject = tab[((n).wrapping_sub(1i32)&hash) as usize].clone();
        /* TODO: aconst_null  */
        let mut node: JvmObject = tab[((n).wrapping_sub(1i32)&hash) as usize].clone();
        let mut k: JvmObject = p.key.get();
        let _t0 = key.equals(k)?;
        node = p;
        let mut e: JvmObject = p.next.get();
        let _t1 = p.getTreeNode(hash, key)?;
        node = _t1;
        k = e.key.get();
        let _t2 = key.equals(k)?;
        node = e;
        p = e;
        e = e.next.get();
        let mut v: JvmObject = node.value.get();
        let _t3 = value.equals(v)?;
        node.removeTreeNode(this, tab, movable)?;
        tab[index as usize] = node.next.get();
        p.next.set(node.next.get());
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.size.set((this.size.get()).wrapping_sub(1i32));
        this.afterNodeRemoval(node)?;
        return Ok(node);
        /* TODO: aconst_null  */
        Ok(p)
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public"))]
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut tab: JvmObject = this.table.get();
        this.size.set(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= (tab.len() as i32) { break; }
            /* TODO: aconst_null  */
            /* UNDERFLOW */[tab as usize] = i;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn containsValue(&self, value: JvmObject) -> Result<bool> {
        let this = self;
        let mut tab: JvmObject = this.table.get();
        let mut local_4: JvmObject = tab;
        let mut local_5: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut e: JvmObject = local_4[local_6 as usize].clone();
            let mut v: JvmObject = e.value.get();
            let _t0 = value.equals(v)?;
            return Ok(1i32);
            e = e.next.get();
            local_6 = local_6.wrapping_add(1i32);
        }
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn keySet(&self) -> Result<JvmObject> {
        let this = self;
        let mut ks: JvmObject = this.keySet.get();
        ks = HashMap$KeySet::new(this)?;
        this.keySet.set(ks);
        Ok(ks)
    }

    #[cfg_attr(any(), java_method(name = "prepareArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "final"))]
    pub fn prepareArray(&self, a: JvmObject) -> Result<JvmObject> {
        let this = self;
        let mut size: i32 = this.size.get();
        let _t0 = a.getClass()?;
        let _t1 = _t0.getComponentType()?;
        let _t2: JvmObject = Array::newInstance(_t1, size)?;
        return Ok(_t2);
        /* TODO: aconst_null  */
        size[a as usize] = size;
        Ok(a)
    }

    #[cfg_attr(any(), java_method(name = "keysToArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;"))]
    pub fn keysToArray(&self, a: JvmObject) -> Result<JvmObject> {
        let this = self;
        let mut r: JvmObject = a;
        let mut idx: i32 = 0i32;
        let mut tab: JvmObject = this.table.get();
        let mut local_5: JvmObject = tab;
        let mut local_6: i32 = (local_5.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut e: JvmObject = local_5[local_7 as usize].clone();
            idx = idx.wrapping_add(1i32);
            r[idx as usize] = e.key.get();
            e = e.next.get();
            local_7 = local_7.wrapping_add(1i32);
        }
        Ok(a)
    }

    #[cfg_attr(any(), java_method(name = "valuesToArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;"))]
    pub fn valuesToArray(&self, a: JvmObject) -> Result<JvmObject> {
        let this = self;
        let mut r: JvmObject = a;
        let mut idx: i32 = 0i32;
        let mut tab: JvmObject = this.table.get();
        let mut local_5: JvmObject = tab;
        let mut local_6: i32 = (local_5.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut e: JvmObject = local_5[local_7 as usize].clone();
            idx = idx.wrapping_add(1i32);
            r[idx as usize] = e.value.get();
            e = e.next.get();
            local_7 = local_7.wrapping_add(1i32);
        }
        Ok(a)
    }

    #[cfg_attr(any(), java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public"))]
    pub fn values(&self) -> Result<JvmObject> {
        let this = self;
        let mut vs: JvmObject = this.values.get();
        vs = HashMap$Values::new(this)?;
        this.values.set(vs);
        Ok(vs)
    }

    #[cfg_attr(any(), java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn entrySet(&self) -> Result<JvmObject> {
        let this = self;
        let mut es: JvmObject = this.entrySet.get();
        this.entrySet.set(HashMap$EntrySet::new(this)?);
        Ok(es)
    }

    #[cfg_attr(any(), java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn getOrDefault(&self, key: JvmObject, defaultValue: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.getNode(key)?;
        let mut e: JvmObject = _t0;
        Ok(e.value.get())
    }

    #[cfg_attr(any(), java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn putIfAbsent(&self, key: JvmObject, value: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: i32 = HashMap::hash(key)?;
        let _t1 = this.putVal(_t0, key, value, 1i32, 1i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public"))]
    pub fn remove(&self, key: JvmObject, value: JvmObject) -> Result<bool> {
        let this = self;
        let _t0: i32 = HashMap::hash(key)?;
        let _t1 = this.removeNode(_t0, key, value, 1i32, 1i32)?;
        Ok(!_t1.is_none())
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public"))]
    pub fn replace(&self, key: JvmObject, oldValue: JvmObject, newValue: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = this.getNode(key)?;
        let mut e: JvmObject = _t0;
        let mut v: JvmObject = e.value.get();
        let _t1 = v.equals(oldValue)?;
        e.value.set(newValue);
        this.afterNodeAccess(e)?;
        return Ok(1i32);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn replace(&self, key: JvmObject, value: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.getNode(key)?;
        let mut e: JvmObject = _t0;
        let mut oldValue: JvmObject = e.value.get();
        e.value.set(value);
        this.afterNodeAccess(e)?;
        return Ok(oldValue);
        /* TODO: aconst_null  */
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public"))]
    pub fn computeIfAbsent(&self, key: JvmObject, mappingFunction: JvmObject) -> Result<JvmObject> {
        let this = self;
        panic!("{}", /* NullPointerException::new()? */);
        let _t0: i32 = HashMap::hash(key)?;
        let mut hash: i32 = _t0;
        let mut binCount: i32 = 0i32;
        /* TODO: aconst_null  */
        let mut t: JvmObject = mappingFunction;
        /* TODO: aconst_null  */
        let mut old: i32 = /* UNDERFLOW */;
        let mut tab: JvmObject = this.table.get();
        let mut n: i32 = (tab.len() as i32);
        let _t1 = this.resize()?;
        tab = _t1;
        n = (_t1.len() as i32);
        let mut i: i32 = ((n).wrapping_sub(1i32)&hash);
        let mut first: JvmObject = tab[((n).wrapping_sub(1i32)&hash) as usize].clone();
        t = first;
        let _t2 = first.getTreeNode(hash, key)?;
        old = _t2;
        let mut e: JvmObject = first;
        let mut k: JvmObject = e.key.get();
        let _t3 = key.equals(k)?;
        old = e;
        binCount = binCount.wrapping_add(1i32);
        e = e.next.get();
        e = old.value.get();
        this.afterNodeAccess(old)?;
        return Ok(e);
        e = this.modCount.get();
        let _t4 = mappingFunction.apply(key)?;
        k = _t4;
        panic!("{}", /* ConcurrentModificationException::new()? */);
        /* TODO: aconst_null  */
        return Ok(k);
        old.value.set(k);
        this.afterNodeAccess(old)?;
        return Ok(k);
        let _t5 = t.putTreeVal(this, tab, hash, key, k)?;
        let _t6 = this.newNode(hash, key, k, first)?;
        tab[i as usize] = _t6;
        this.treeifyBin(tab, hash)?;
        this.modCount.set((e).wrapping_add(1i32));
        this.size.set((this.size.get()).wrapping_add(1i32));
        this.afterNodeInsertion(1i32)?;
        Ok(k)
    }

    #[cfg_attr(any(), java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn computeIfPresent(&self, key: JvmObject, remappingFunction: JvmObject) -> Result<JvmObject> {
        let this = self;
        panic!("{}", /* NullPointerException::new()? */);
        let _t0 = this.getNode(key)?;
        let mut e: JvmObject = _t0;
        let mut oldValue: JvmObject = e.value.get();
        let mut mc: i32 = this.modCount.get();
        let _t1 = remappingFunction.apply(key, oldValue)?;
        let mut v: JvmObject = _t1;
        panic!("{}", /* ConcurrentModificationException::new()? */);
        e.value.set(v);
        this.afterNodeAccess(e)?;
        return Ok(v);
        let _t2: i32 = HashMap::hash(key)?;
        let mut hash: i32 = _t2;
        /* TODO: aconst_null  */
        let _t3 = v.removeNode(this, hash, key, 0i32, 1i32)?;
        /* TODO: aconst_null  */
        Ok(this.modCount.get())
    }

    #[cfg_attr(any(), java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn compute(&self, key: JvmObject, remappingFunction: JvmObject) -> Result<JvmObject> {
        let this = self;
        panic!("{}", /* NullPointerException::new()? */);
        let _t0: i32 = HashMap::hash(key)?;
        let mut hash: i32 = _t0;
        let mut binCount: i32 = 0i32;
        /* TODO: aconst_null  */
        let mut t: JvmObject = remappingFunction;
        /* TODO: aconst_null  */
        let mut old: i32 = /* UNDERFLOW */;
        let mut tab: JvmObject = this.table.get();
        let mut n: i32 = (tab.len() as i32);
        let _t1 = this.resize()?;
        tab = _t1;
        n = (_t1.len() as i32);
        let mut i: i32 = ((n).wrapping_sub(1i32)&hash);
        let mut first: JvmObject = tab[((n).wrapping_sub(1i32)&hash) as usize].clone();
        t = first;
        let _t2 = first.getTreeNode(hash, key)?;
        old = _t2;
        let mut e: JvmObject = first;
        let mut k: JvmObject = e.key.get();
        let _t3 = key.equals(k)?;
        old = e;
        binCount = binCount.wrapping_add(1i32);
        e = e.next.get();
        /* TODO: aconst_null  */
        e = old.value.get();
        k = this.modCount.get();
        let _t4 = remappingFunction.apply(key, e)?;
        let mut v: JvmObject = _t4;
        panic!("{}", /* ConcurrentModificationException::new()? */);
        old.value.set(v);
        this.afterNodeAccess(old)?;
        /* TODO: aconst_null  */
        let _t5 = v.removeNode(this, hash, key, 0i32, 1i32)?;
        let _t6 = t.putTreeVal(this, tab, hash, key, v)?;
        let _t7 = this.newNode(hash, key, v, first)?;
        tab[i as usize] = _t7;
        this.treeifyBin(tab, hash)?;
        this.modCount.set((k).wrapping_add(1i32));
        this.size.set((this.size.get()).wrapping_add(1i32));
        this.afterNodeInsertion(1i32)?;
        Ok(v)
    }

    #[cfg_attr(any(), java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn merge(&self, key: JvmObject, value: JvmObject, remappingFunction: JvmObject) -> Result<JvmObject> {
        let this = self;
        panic!("{}", /* NullPointerException::new()? */);
        let _t0: i32 = HashMap::hash(key)?;
        let mut hash: i32 = _t0;
        let mut binCount: i32 = 0i32;
        /* TODO: aconst_null  */
        let mut t: JvmObject = remappingFunction;
        /* TODO: aconst_null  */
        let mut old: JvmObject = value;
        let mut tab: JvmObject = this.table.get();
        let mut n: i32 = (tab.len() as i32);
        let _t1 = this.resize()?;
        tab = _t1;
        n = (_t1.len() as i32);
        let mut i: i32 = ((n).wrapping_sub(1i32)&hash);
        let mut first: JvmObject = tab[((n).wrapping_sub(1i32)&hash) as usize].clone();
        t = first;
        let _t2 = first.getTreeNode(hash, key)?;
        old = _t2;
        let mut e: JvmObject = first;
        let mut k: JvmObject = e.key.get();
        let _t3 = key.equals(k)?;
        old = e;
        binCount = binCount.wrapping_add(1i32);
        e = e.next.get();
        k = this.modCount.get();
        let _t4 = remappingFunction.apply(old.value.get(), value)?;
        e = _t4;
        panic!("{}", /* ConcurrentModificationException::new()? */);
        e = value;
        old.value.set(e);
        this.afterNodeAccess(old)?;
        /* TODO: aconst_null  */
        let _t5 = e.removeNode(this, hash, key, 0i32, 1i32)?;
        return Ok(e);
        let _t6 = t.putTreeVal(this, tab, hash, key, value)?;
        let _t7 = this.newNode(hash, key, value, first)?;
        tab[i as usize] = _t7;
        this.treeifyBin(tab, hash)?;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.size.set((this.size.get()).wrapping_add(1i32));
        this.afterNodeInsertion(1i32)?;
        Ok(value)
    }

    #[cfg_attr(any(), java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public"))]
    pub fn forEach(&self, action: JvmObject) -> Result<()> {
        let this = self;
        panic!("{}", /* NullPointerException::new()? */);
        let mut tab: JvmObject = this.table.get();
        let mut mc: i32 = this.modCount.get();
        let mut local_4: JvmObject = tab;
        let mut local_5: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut e: JvmObject = local_4[local_6 as usize].clone();
            action.accept(e.key.get(), e.value.get())?;
            e = e.next.get();
            local_6 = local_6.wrapping_add(1i32);
        }
        panic!("{}", /* ConcurrentModificationException::new()? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public"))]
    pub fn replaceAll(&self, function: JvmObject) -> Result<()> {
        let this = self;
        panic!("{}", /* NullPointerException::new()? */);
        let mut tab: JvmObject = this.table.get();
        let mut mc: i32 = this.modCount.get();
        let mut local_4: JvmObject = tab;
        let mut local_5: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut e: JvmObject = local_4[local_6 as usize].clone();
            let _t0 = function.apply(e.key.get(), e.value.get())?;
            e.value.set(_t0);
            e = e.next.get();
            local_6 = local_6.wrapping_add(1i32);
        }
        panic!("{}", /* ConcurrentModificationException::new()? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn clone(&self) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractMap::clone()?;
        let mut result: JvmObject = _t0;
        let mut e: java/util/HashMap = this;
        panic!("{}", /* InternalError::new(e)? */);
        result.reinitialize()?;
        result.putMapEntries(this, 0i32)?;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "loadFactor", descriptor = "()F", access = "final"))]
    pub fn loadFactor(&self) -> Result<f32> {
        let this = self;
        Ok(this.loadFactor.get())
    }

    #[cfg_attr(any(), java_method(name = "capacity", descriptor = "()I", access = "final"))]
    pub fn capacity(&self) -> Result<i32> {
        let this = self;
        Ok(16i32)
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private"))]
    pub fn writeObject(&self, s: JvmObject) -> Result<()> {
        let this = self;
        let _t0 = this.capacity()?;
        let mut buckets: i32 = _t0;
        s.defaultWriteObject()?;
        s.writeInt(buckets)?;
        s.writeInt(this.size.get())?;
        this.internalWriteEntries(s)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, s: JvmObject) -> Result<()> {
        let this = self;
        let _t0 = s.readFields()?;
        let mut fields: JvmObject = _t0;
        let _t1 = fields.get(String::from("loadFactor"), 0.75f32)?;
        let mut lf: f32 = _t1;
        /* TODO: fcmpg  */
        let _t2: bool = Float::isNaN(lf)?;
        String::new().append(&String::from("Illegal load factor:"))?;
        String::new().append(&lf)?;
        panic!("{}", /* InvalidObjectException::new(String::new())? */);
        let _t3: f32 = (lf).abs();
        lf = _t3;
        HashMap$UnsafeHolder::putLoadFactor(this, lf)?;
        this.reinitialize()?;
        let _t4 = s.readInt()?;
        let _t5 = s.readInt()?;
        let mut mappings: i32 = _t5;
        String::new().append(&String::from("Illegal mappings count:"))?;
        String::new().append(&mappings)?;
        panic!("{}", /* InvalidObjectException::new(String::new())? */);
        let _t6: f64 = (((mappings as f64)/(lf as f64)) as f64).ceil();
        let mut dc: f64 = _t6;
        /* TODO: dcmpg  */
        /* TODO: dcmpl  */
        let _t7: i32 = HashMap::tableSizeFor((dc as i32))?;
        let mut cap: i32 = _t7;
        let mut ft: f32 = ((cap as f32)*lf);
        /* TODO: fcmpg  */
        (ft as i32).threshold.set(2147483647i32);
        let _t8: JvmObject = SharedSecrets::getJavaObjectInputStreamAccess()?;
        _t8.checkArray(s, 364i32, cap)?;
        let mut _arr9: Vec<JvmObject> = Vec::with_capacity(cap as usize);
        let mut tab: Vec<JvmObject> = _arr9;
        this.table.set(tab);
        let mut i: i32 = 0i32;
        loop {
            if i >= mappings { break; }
            let _t0 = s.readObject()?;
            let mut key: JvmObject = _t0;
            let _t1 = s.readObject()?;
            let mut value: JvmObject = _t1;
            let _t2: i32 = HashMap::hash(key)?;
            let _t3 = this.putVal(_t2, key, value, 0i32, 0i32)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "newNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;"))]
    pub fn newNode(&self, hash: i32, key: JvmObject, value: JvmObject, next: JvmObject) -> Result<JvmObject> {
        let this = self;
        Ok(HashMap$Node::new(hash, key, value, next)?)
    }

    #[cfg_attr(any(), java_method(name = "replacementNode", descriptor = "(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;"))]
    pub fn replacementNode(&self, p: JvmObject, next: JvmObject) -> Result<JvmObject> {
        let this = self;
        Ok(HashMap$Node::new(p.hash.get(), p.key.get(), p.value.get(), next)?)
    }

    #[cfg_attr(any(), java_method(name = "newTreeNode", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;"))]
    pub fn newTreeNode(&self, hash: i32, key: JvmObject, value: JvmObject, next: JvmObject) -> Result<JvmObject> {
        let this = self;
        Ok(HashMap$TreeNode::new(hash, key, value, next)?)
    }

    #[cfg_attr(any(), java_method(name = "replacementTreeNode", descriptor = "(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;"))]
    pub fn replacementTreeNode(&self, p: JvmObject, next: JvmObject) -> Result<JvmObject> {
        let this = self;
        Ok(HashMap$TreeNode::new(p.hash.get(), p.key.get(), p.value.get(), next)?)
    }

    #[cfg_attr(any(), java_method(name = "reinitialize", descriptor = "()V"))]
    pub fn reinitialize(&self) -> Result<()> {
        let this = self;
        /* TODO: aconst_null  */
        /* UNDERFLOW */.table.set(this);
        /* TODO: aconst_null  */
        /* UNDERFLOW */.entrySet.set(this);
        /* TODO: aconst_null  */
        /* UNDERFLOW */.keySet.set(this);
        /* TODO: aconst_null  */
        /* UNDERFLOW */.values.set(this);
        this.modCount.set(0i32);
        this.threshold.set(0i32);
        this.size.set(0i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "afterNodeAccess", descriptor = "(Ljava/util/HashMap$Node;)V"))]
    pub fn afterNodeAccess(&self, p: JvmObject) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "afterNodeInsertion", descriptor = "(Z)V"))]
    pub fn afterNodeInsertion(&self, evict: bool) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "afterNodeRemoval", descriptor = "(Ljava/util/HashMap$Node;)V"))]
    pub fn afterNodeRemoval(&self, p: JvmObject) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "internalWriteEntries", descriptor = "(Ljava/io/ObjectOutputStream;)V"))]
    pub fn internalWriteEntries(&self, s: JvmObject) -> Result<()> {
        let this = self;
        let mut tab: JvmObject = this.table.get();
        let mut local_3: JvmObject = tab;
        let mut local_4: i32 = (local_3.len() as i32);
        let mut local_5: i32 = 0i32;
        loop {
            if local_5 >= local_4 { break; }
            let mut e: JvmObject = local_3[local_5 as usize].clone();
            s.writeObject(e.key.get())?;
            s.writeObject(e.value.get())?;
            e = e.next.get();
            local_5 = local_5.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "calculateHashMapCapacity", descriptor = "(I)I", access = "static"))]
    pub fn calculateHashMapCapacity(numMappings: i32) -> Result<i32> {
        let _t0: f64 = (((numMappings as f64)/0.75f64) as f64).ceil();
        Ok((_t0 as i32))
    }

    #[cfg_attr(any(), java_method(name = "newHashMap", descriptor = "(I)Ljava/util/HashMap;", access = "public static"))]
    pub fn newHashMap(numMappings: i32) -> Result<JvmObject> {
        String::new().append(&String::from("Negative number of mappings:"))?;
        String::new().append(&numMappings)?;
        panic!("{}", /* IllegalArgumentException::new(String::new())? */);
        let _t0: i32 = HashMap::calculateHashMapCapacity(numMappings)?;
        Ok(HashMap::<_, _>::new()?)
    }
}
