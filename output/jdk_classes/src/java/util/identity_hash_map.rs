#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/IdentityHashMap",
    super_class = "java/util/AbstractMap",
    interfaces  = "java/util/Map,java/io/Serializable,java/lang/Cloneable",
    access      = "public",
    source      = "IdentityHashMap.java",
))]
pub struct IdentityHashMap<K, V> {
    #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/lang/Object;", access = ""))]
    pub table: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I"))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = ""))]
    pub modCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
}

impl<K: Clone + 'static, V: Clone + 'static> IdentityHashMap<K, V> {
    // java: maskNull(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn maskNull(key: Object) -> Result<Object> {
        Ok(key)
    }

    // java: unmaskNull(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn unmaskNull(key: Object) -> Result<Object> {
        /* TODO: aconst_null  */
        Ok(key)
    }

    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { table: Field::new(Default::default()), size: Field::new(0), modCount: Field::new(0), entrySet: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractMap.<init>:()V */
        this.init(32i32)?;
        Ok(this)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(expectedMaxSize: i32) -> Result<Self> {
        let this = Self { table: Field::new(Default::default()), size: Field::new(0), modCount: Field::new(0), entrySet: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractMap.<init>:()V */
        String::new().append(&String::from("expectedMaxSize is negative:"))?;
        String::new().append(&expectedMaxSize)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i32 = IdentityHashMap::capacity(expectedMaxSize)?;
        this.init(_t0)?;
        Ok(this)
    }

    // java: capacity(I)I
    pub fn capacity(expectedMaxSize: i32) -> Result<i32> {
        let _t0: i32 = Integer::highestOneBit((expectedMaxSize).wrapping_add((expectedMaxSize<<(1i32&0x1f))))?;
        Ok(_t0)
    }

    // java: init(I)V
    pub fn init(&self, initCapacity: i32) -> Result<()> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity((2i32).wrapping_mul(initCapacity) as usize);
        this.table.set(_arr0);
        Ok(())
    }

    // java: <init>(Ljava/util/Map;)V
    // java: <init>(Ljava/util/Map;)V
    pub fn new__map(m: Object) -> Result<Self> {
        let this = Self { table: Field::new(Default::default()), size: Field::new(0), modCount: Field::new(0), entrySet: Field::new(Default::default()) };
        let _t0 = m.size()?;
        /* invokespecial Method java/util/IdentityHashMap.<init>:(I)V */
        this.putAll(m)?;
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(this.size.get()==0i32)
    }

    // java: hash(Ljava/lang/Object;I)I
    pub fn hash(x: Object, length: i32) -> Result<i32> {
        let _t0: i32 = System::identityHashCode(x)?;
        let mut h: i32 = _t0;
        Ok((((h<<(1i32&0x1f))).wrapping_sub((h<<(8i32&0x1f)))&(length).wrapping_sub(1i32)))
    }

    // java: nextKeyIndex(II)I
    pub fn nextKeyIndex(i: i32, len: i32) -> Result<i32> {
        Ok(0i32)
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<V> {
        let this = self;
        let _t0: Object = IdentityHashMap::maskNull(key)?;
        let mut k: Object = _t0;
        let mut tab: Vec<Object> = this.table.get();
        let mut len: i32 = (tab.len() as i32);
        let _t1: i32 = IdentityHashMap::hash(k, len)?;
        let mut i: i32 = _t1;
        let mut item: Object = tab[i as usize].clone();
        return Ok(tab[(i).wrapping_add(1i32) as usize].clone());
        /* TODO: aconst_null  */
        return Ok(item);
        let _t2: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
        i = _t2;
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = IdentityHashMap::maskNull(key)?;
        let mut k: Object = _t0;
        let mut tab: Vec<Object> = this.table.get();
        let mut len: i32 = (tab.len() as i32);
        let _t1: i32 = IdentityHashMap::hash(k, len)?;
        let mut i: i32 = _t1;
        let mut item: Object = tab[i as usize].clone();
        return Ok(1i32);
        return Ok(0i32);
        let _t2: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
        i = _t2;
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, value: Object) -> Result<bool> {
        let this = self;
        let mut tab: Vec<Object> = this.table.get();
        let mut i: i32 = 1i32;
        loop {
            if i >= (tab.len() as i32) { break; }
            return Ok(1i32);
            i = i.wrapping_add(2i32);
        }
        Ok(0i32)
    }

    // java: containsMapping(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn containsMapping(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = IdentityHashMap::maskNull(key)?;
        let mut k: Object = _t0;
        let mut tab: Vec<Object> = this.table.get();
        let mut len: i32 = (tab.len() as i32);
        let _t1: i32 = IdentityHashMap::hash(k, len)?;
        let mut i: i32 = _t1;
        let mut item: Object = tab[i as usize].clone();
        return Ok(/* if_acmpne */ true);
        return Ok(0i32);
        let _t2: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
        i = _t2;
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0: Object = IdentityHashMap::maskNull(key)?;
        let mut k: Object = _t0;
        loop {
            let mut tab: Vec<Object> = this.table.get();
            let mut len: i32 = (tab.len() as i32);
            let _t0: i32 = IdentityHashMap::hash(k, len)?;
            let mut i: i32 = _t0;
            let mut item: Object = tab[i as usize].clone();
            let mut oldValue: Object = tab[(i).wrapping_add(1i32) as usize].clone();
            tab[(i).wrapping_add(1i32) as usize] = value;
            return Ok(oldValue);
            let _t1: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
            i = _t1;
            item = (this.size.get()).wrapping_add(1i32);
            if (item).wrapping_add((item<<(1i32&0x1f))) <= len { break; }
            let _t0 = this.resize(len)?;
        }
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        tab[i as usize] = k;
        tab[(i).wrapping_add(1i32) as usize] = value;
        this.size.set(item);
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    // java: resize(I)Z
    pub fn resize(&self, newCapacity: i32) -> Result<bool> {
        let this = self;
        let mut newLength: i32 = (newCapacity).wrapping_mul(2i32);
        let mut oldTable: Vec<Object> = this.table.get();
        let mut oldLength: i32 = (oldTable.len() as i32);
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(0i32);
        return Ok(0i32);
        let mut _arr0: Vec<Object> = Vec::with_capacity(newLength as usize);
        let mut newTable: Vec<Object> = _arr0;
        let mut j: i32 = 0i32;
        loop {
            if j >= oldLength { break; }
            let mut key: Object = oldTable[j as usize].clone();
            let mut value: Object = oldTable[(j).wrapping_add(1i32) as usize].clone();
            /* TODO: aconst_null  */
            key[oldTable as usize] = j;
            /* TODO: aconst_null  */
            todo!("stack underflow")[oldTable as usize] = (j).wrapping_add(1i32);
            let _t0: i32 = IdentityHashMap::hash(key, newLength)?;
            let mut i: i32 = _t0;
            let _t1: i32 = IdentityHashMap::nextKeyIndex(i, newLength)?;
            i = _t1;
            newTable[i as usize] = key;
            newTable[(i).wrapping_add(1i32) as usize] = value;
            j = j.wrapping_add(2i32);
        }
        this.table.set(newTable);
        Ok(1i32)
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, m: Object) -> Result<()> {
        let this = self;
        let _t0 = m.size()?;
        let mut n: i32 = _t0;
        return Ok(());
        let _t1: i32 = IdentityHashMap::capacity(n)?;
        let _t2 = this.resize(_t1)?;
        let _t3 = m.entrySet()?;
        let _t4 = _t3.iterator()?;
        let mut local_3: Object = _t4;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut e: Object = _t0;
            let _t1 = e.getKey()?;
            let _t2 = e.getValue()?;
            let _t3 = this.put(_t1, _t2)?;
        }
        Ok(())
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<V> {
        let this = self;
        let _t0: Object = IdentityHashMap::maskNull(key)?;
        let mut k: Object = _t0;
        let mut tab: Vec<Object> = this.table.get();
        let mut len: i32 = (tab.len() as i32);
        let _t1: i32 = IdentityHashMap::hash(k, len)?;
        let mut i: i32 = _t1;
        let mut item: Object = tab[i as usize].clone();
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.size.set((this.size.get()).wrapping_sub(1i32));
        let mut oldValue: Object = tab[(i).wrapping_add(1i32) as usize].clone();
        /* TODO: aconst_null  */
        k[tab as usize] = (i).wrapping_add(1i32);
        /* TODO: aconst_null  */
        item[tab as usize] = i;
        this.closeDeletion(i)?;
        return Ok(oldValue);
        /* TODO: aconst_null  */
        return Ok(item);
        let _t2: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
        i = _t2;
    }

    // java: removeMapping(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn removeMapping(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = IdentityHashMap::maskNull(key)?;
        let mut k: Object = _t0;
        let mut tab: Vec<Object> = this.table.get();
        let mut len: i32 = (tab.len() as i32);
        let _t1: i32 = IdentityHashMap::hash(k, len)?;
        let mut i: i32 = _t1;
        let mut item: Object = tab[i as usize].clone();
        return Ok(0i32);
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        this.size.set((this.size.get()).wrapping_sub(1i32));
        /* TODO: aconst_null  */
        value[tab as usize] = i;
        /* TODO: aconst_null  */
        tab[(i).wrapping_add(1i32) as usize].clone()[tab as usize] = (i).wrapping_add(1i32);
        this.closeDeletion(i)?;
        return Ok(1i32);
        return Ok(0i32);
        let _t2: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
        i = _t2;
    }

    // java: closeDeletion(I)V
    pub fn closeDeletion(&self, d: i32) -> Result<()> {
        let this = self;
        let mut tab: Vec<Object> = this.table.get();
        let mut len: i32 = (tab.len() as i32);
        let _t0: i32 = IdentityHashMap::nextKeyIndex(d, len)?;
        let mut i: i32 = _t0;
        loop {
            let mut item: Object = tab[i as usize].clone();
            if tab[i as usize].clone().is_none() { break; }
            let _t0: i32 = IdentityHashMap::hash(item, len)?;
            let mut r: i32 = _t0;
            tab[d as usize] = item;
            tab[(d).wrapping_add(1i32) as usize] = tab[(i).wrapping_add(1i32) as usize].clone();
            /* TODO: aconst_null  */
            i[tab as usize] = i;
            /* TODO: aconst_null  */
            d[tab as usize] = (i).wrapping_add(1i32);
            d = i;
            let _t1: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
            i = _t1;
        }
        Ok(())
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        let mut tab: Vec<Object> = this.table.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (tab.len() as i32) { break; }
            /* TODO: aconst_null  */
            todo!("stack underflow")[tab as usize] = i;
            i = i.wrapping_add(1i32);
        }
        this.size.set(0i32);
        Ok(())
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut m: Object = o;
        let _t0 = m.size()?;
        return Ok(0i32);
        let mut tab: Vec<Object> = m.table.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (tab.len() as i32) { break; }
            let mut k: Object = tab[i as usize].clone();
            let _t0 = this.containsMapping(k, tab[(i).wrapping_add(1i32) as usize].clone())?;
            return Ok(0i32);
            i = i.wrapping_add(2i32);
        }
        return Ok(1i32);
        let mut m: Object = o;
        let _t1 = this.entrySet()?;
        let _t2 = m.entrySet()?;
        let _t3 = _t1.equals(_t2)?;
        return Ok(_t3);
        Ok(0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut result: i32 = 0i32;
        let mut tab: Vec<Object> = this.table.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (tab.len() as i32) { break; }
            let mut key: Object = tab[i as usize].clone();
            let _t0: Object = IdentityHashMap::unmaskNull(key)?;
            let mut k: Object = _t0;
            let _t1: i32 = System::identityHashCode(k)?;
            let _t2: i32 = System::identityHashCode(tab[(i).wrapping_add(1i32) as usize].clone())?;
            result = (result).wrapping_add((_t1^_t2));
            i = i.wrapping_add(2i32);
        }
        Ok(result)
    }

    // java: clone()Ljava/lang/Object;
    pub fn clone(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractMap::clone()?;
        let mut m: Object = _t0;
        /* TODO: aconst_null  */
        this.entrySet.set(m);
        let _t1 = this.table.get().clone()?;
        m.table.set(_t1);
        return Ok(m);
        m = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        let this = self;
        let mut ks: Object = this.keySet.get();
        ks = IdentityHashMap_KeySet::new(this)?;
        this.keySet.set(ks);
        Ok(ks)
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        let this = self;
        let mut vs: Object = this.values.get();
        vs = IdentityHashMap_Values::new(this)?;
        this.values.set(vs);
        Ok(vs)
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        let mut es: Object = this.entrySet.get();
        return Ok(es);
        this.entrySet.set(IdentityHashMap_EntrySet::new(this)?);
        Ok(IdentityHashMap_EntrySet::new(this)?)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultWriteObject()?;
        s.writeInt(this.size.get())?;
        let mut tab: Vec<Object> = this.table.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (tab.len() as i32) { break; }
            let mut key: Object = tab[i as usize].clone();
            let _t0: Object = IdentityHashMap::unmaskNull(key)?;
            s.writeObject(_t0)?;
            s.writeObject(tab[(i).wrapping_add(1i32) as usize].clone())?;
            i = i.wrapping_add(2i32);
        }
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.readFields()?;
        let _t1 = s.readInt()?;
        let mut size: i32 = _t1;
        String::new().append(&String::from("Illegal mappings count:"))?;
        String::new().append(&size)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2: i32 = IdentityHashMap::capacity(size)?;
        let mut cap: i32 = _t2;
        let _t3: Object = SharedSecrets::getJavaObjectInputStreamAccess()?;
        _t3.checkArray(s, 159i32, (cap).wrapping_mul(2i32))?;
        this.size.set(size);
        this.init(cap)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = s.readObject()?;
            let mut key: Object = _t0;
            let _t1 = s.readObject()?;
            let mut value: Object = _t1;
            this.putForCreate(key, value)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: putForCreate(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn putForCreate(&self, key: K, value: V) -> Result<()> {
        let this = self;
        let _t0: Object = IdentityHashMap::maskNull(key)?;
        let mut k: Object = _t0;
        let mut tab: Vec<Object> = this.table.get();
        let mut len: i32 = (tab.len() as i32);
        let _t1: i32 = IdentityHashMap::hash(k, len)?;
        let mut i: i32 = _t1;
        loop {
            let mut item: Object = tab[i as usize].clone();
            if tab[i as usize].clone().is_none() { break; }
            return Err(JvmError::Custom("athrow".to_owned()));
            let _t0: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
            i = _t0;
        }
        tab[i as usize] = k;
        tab[(i).wrapping_add(1i32) as usize] = value;
        Ok(())
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let mut expectedModCount: i32 = this.modCount.get();
        let mut t: Vec<Object> = this.table.get();
        let mut index: i32 = 0i32;
        loop {
            if index >= (t.len() as i32) { break; }
            let mut k: Object = t[index as usize].clone();
            let _t0: Object = IdentityHashMap::unmaskNull(k)?;
            action.accept(_t0, t[(index).wrapping_add(1i32) as usize].clone())?;
            return Err(JvmError::Custom("athrow".to_owned()));
            index = index.wrapping_add(2i32);
        }
        Ok(())
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(function)?;
        let mut expectedModCount: i32 = this.modCount.get();
        let mut t: Vec<Object> = this.table.get();
        let mut index: i32 = 0i32;
        loop {
            if index >= (t.len() as i32) { break; }
            let mut k: Object = t[index as usize].clone();
            let _t0: Object = IdentityHashMap::unmaskNull(k)?;
            let _t1 = function.apply(_t0, t[(index).wrapping_add(1i32) as usize].clone())?;
            t[(index).wrapping_add(1i32) as usize] = _t1;
            return Err(JvmError::Custom("athrow".to_owned()));
            index = index.wrapping_add(2i32);
        }
        Ok(())
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.removeMapping(key, value)?;
        Ok(_t0)
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
        let this = self;
        let _t0: Object = IdentityHashMap::maskNull(key)?;
        let mut k: Object = _t0;
        let mut tab: Vec<Object> = this.table.get();
        let mut len: i32 = (tab.len() as i32);
        let _t1: i32 = IdentityHashMap::hash(k, len)?;
        let mut i: i32 = _t1;
        let mut item: Object = tab[i as usize].clone();
        return Ok(0i32);
        tab[(i).wrapping_add(1i32) as usize] = newValue;
        return Ok(1i32);
        return Ok(0i32);
        let _t2: i32 = IdentityHashMap::nextKeyIndex(i, len)?;
        i = _t2;
    }
}
