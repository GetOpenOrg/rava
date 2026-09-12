#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Map",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Map.java",
))]
pub struct Map<K, V>;

impl<K: Clone + 'static, V: Clone + 'static> Map<K, V> {
    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Map.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Map.isEmpty")
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Map.containsKey")
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Map.containsValue")
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/Map.get")
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, arg0: Object, arg1: Object) -> Result<Object> {
        todo!("abstract java/util/Map.put")
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/Map.remove")
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/Map.putAll")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Map.clear")
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        todo!("abstract java/util/Map.keySet")
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        todo!("abstract java/util/Map.values")
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        todo!("abstract java/util/Map.entrySet")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Map.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Map.hashCode")
    }

    // java: getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<V> {
        let this = self;
        let _t0 = this.get(key)?;
        let mut v: Object = _t0;
        let _t1 = this.containsKey(key)?;
        Ok(defaultValue)
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let _t1 = this.entrySet()?;
        let _t2 = _t1.iterator()?;
        let mut local_2: Object = _t2;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut entry: Object = _t0;
            let _t1 = entry.getKey()?;
            let mut k: Object = _t1;
            let _t2 = entry.getValue()?;
            let mut v: Object = _t2;
            let mut ise: i32 = todo!("stack underflow");
            return Err(JvmError::Custom("athrow".to_owned()));
            action.accept(k, v)?;
        }
        Ok(())
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(function)?;
        let _t1 = this.entrySet()?;
        let _t2 = _t1.iterator()?;
        let mut local_2: Object = _t2;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut entry: Object = _t0;
            let _t1 = entry.getKey()?;
            let mut k: Object = _t1;
            let _t2 = entry.getValue()?;
            let mut v: Object = _t2;
            let mut ise: i32 = todo!("stack underflow");
            return Err(JvmError::Custom("athrow".to_owned()));
            let _t3 = function.apply(k, v)?;
            v = _t3;
            let _t4 = entry.setValue(v)?;
            ise = todo!("stack underflow");
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(())
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0 = this.get(key)?;
        let mut v: Object = _t0;
        let _t1 = this.put(key, value)?;
        v = _t1;
        Ok(v)
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.get(key)?;
        let mut curValue: Object = _t0;
        let _t1: bool = Objects::equals(curValue, value)?;
        let _t2 = this.containsKey(key)?;
        return Ok(0i32);
        let _t3 = this.remove(key)?;
        Ok(1i32)
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
        let this = self;
        let _t0 = this.get(key)?;
        let mut curValue: Object = _t0;
        let _t1: bool = Objects::equals(curValue, oldValue)?;
        let _t2 = this.containsKey(key)?;
        return Ok(0i32);
        let _t3 = this.put(key, newValue)?;
        Ok(1i32)
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0 = this.get(key)?;
        let mut curValue: Object = _t0;
        let _t1 = this.containsKey(key)?;
        let _t2 = this.put(key, value)?;
        curValue = _t2;
        Ok(curValue)
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: K, mappingFunction: Object) -> Result<V> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(mappingFunction)?;
        let _t1 = this.get(key)?;
        let mut v: Object = _t1;
        let _t2 = mappingFunction.apply(key)?;
        let mut newValue: Object = _t2;
        let _t3 = this.put(key, newValue)?;
        return Ok(newValue);
        Ok(v)
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<V> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(remappingFunction)?;
        let _t1 = this.get(key)?;
        let mut oldValue: Object = _t1;
        let _t2 = remappingFunction.apply(key, oldValue)?;
        let mut newValue: Object = _t2;
        let _t3 = this.put(key, newValue)?;
        return Ok(newValue);
        let _t4 = this.remove(key)?;
        /* TODO: aconst_null  */
        return Ok(newValue);
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: K, remappingFunction: Object) -> Result<V> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(remappingFunction)?;
        let _t1 = this.get(key)?;
        let mut oldValue: Object = _t1;
        let _t2 = remappingFunction.apply(key, oldValue)?;
        let mut newValue: Object = _t2;
        let _t3 = this.containsKey(key)?;
        let _t4 = this.remove(key)?;
        /* TODO: aconst_null  */
        return Ok(_t3);
        /* TODO: aconst_null  */
        return Ok(oldValue);
        let _t5 = this.put(key, newValue)?;
        Ok(newValue)
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<V> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(remappingFunction)?;
        let _t1: Object = Objects::requireNonNull(value)?;
        let _t2 = this.get(key)?;
        let mut oldValue: Object = _t2;
        let _t3 = remappingFunction.apply(oldValue, value)?;
        let mut newValue: Object = _t3;
        let _t4 = this.remove(key)?;
        let _t5 = this.put(key, newValue)?;
        Ok(newValue)
    }

    // java: of()Ljava/util/Map;
    // java: of()Ljava/util/Map;
    pub fn of() -> Result<Object> {
        Ok(ImmutableCollections::EMPTY_MAP())
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj(k1: K, v1: V) -> Result<Object> {
        Ok(ImmutableCollections_Map1::new(k1, v1)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(4i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(6i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        _arr0[4i32 as usize] = k3;
        _arr0[5i32 as usize] = v3;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V, k4: K, v4: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(8i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        _arr0[4i32 as usize] = k3;
        _arr0[5i32 as usize] = v3;
        _arr0[6i32 as usize] = k4;
        _arr0[7i32 as usize] = v4;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V, k4: K, v4: V, k5: K, v5: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(10i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        _arr0[4i32 as usize] = k3;
        _arr0[5i32 as usize] = v3;
        _arr0[6i32 as usize] = k4;
        _arr0[7i32 as usize] = v4;
        _arr0[8i32 as usize] = k5;
        _arr0[9i32 as usize] = v5;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V, k4: K, v4: V, k5: K, v5: V, k6: K, v6: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(12i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        _arr0[4i32 as usize] = k3;
        _arr0[5i32 as usize] = v3;
        _arr0[6i32 as usize] = k4;
        _arr0[7i32 as usize] = v4;
        _arr0[8i32 as usize] = k5;
        _arr0[9i32 as usize] = v5;
        _arr0[10i32 as usize] = k6;
        _arr0[11i32 as usize] = v6;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V, k4: K, v4: V, k5: K, v5: V, k6: K, v6: V, k7: K, v7: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(14i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        _arr0[4i32 as usize] = k3;
        _arr0[5i32 as usize] = v3;
        _arr0[6i32 as usize] = k4;
        _arr0[7i32 as usize] = v4;
        _arr0[8i32 as usize] = k5;
        _arr0[9i32 as usize] = v5;
        _arr0[10i32 as usize] = k6;
        _arr0[11i32 as usize] = v6;
        _arr0[12i32 as usize] = k7;
        _arr0[13i32 as usize] = v7;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V, k4: K, v4: V, k5: K, v5: V, k6: K, v6: V, k7: K, v7: V, k8: K, v8: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(16i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        _arr0[4i32 as usize] = k3;
        _arr0[5i32 as usize] = v3;
        _arr0[6i32 as usize] = k4;
        _arr0[7i32 as usize] = v4;
        _arr0[8i32 as usize] = k5;
        _arr0[9i32 as usize] = v5;
        _arr0[10i32 as usize] = k6;
        _arr0[11i32 as usize] = v6;
        _arr0[12i32 as usize] = k7;
        _arr0[13i32 as usize] = v7;
        _arr0[14i32 as usize] = k8;
        _arr0[15i32 as usize] = v8;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V, k4: K, v4: V, k5: K, v5: V, k6: K, v6: V, k7: K, v7: V, k8: K, v8: V, k9: K, v9: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(18i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        _arr0[4i32 as usize] = k3;
        _arr0[5i32 as usize] = v3;
        _arr0[6i32 as usize] = k4;
        _arr0[7i32 as usize] = v4;
        _arr0[8i32 as usize] = k5;
        _arr0[9i32 as usize] = v5;
        _arr0[10i32 as usize] = k6;
        _arr0[11i32 as usize] = v6;
        _arr0[12i32 as usize] = k7;
        _arr0[13i32 as usize] = v7;
        _arr0[14i32 as usize] = k8;
        _arr0[15i32 as usize] = v8;
        _arr0[16i32 as usize] = k9;
        _arr0[17i32 as usize] = v9;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V, k4: K, v4: V, k5: K, v5: V, k6: K, v6: V, k7: K, v7: V, k8: K, v8: V, k9: K, v9: V, k10: K, v10: V) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(20i32 as usize);
        _arr0[0i32 as usize] = k1;
        _arr0[1i32 as usize] = v1;
        _arr0[2i32 as usize] = k2;
        _arr0[3i32 as usize] = v2;
        _arr0[4i32 as usize] = k3;
        _arr0[5i32 as usize] = v3;
        _arr0[6i32 as usize] = k4;
        _arr0[7i32 as usize] = v4;
        _arr0[8i32 as usize] = k5;
        _arr0[9i32 as usize] = v5;
        _arr0[10i32 as usize] = k6;
        _arr0[11i32 as usize] = v6;
        _arr0[12i32 as usize] = k7;
        _arr0[13i32 as usize] = v7;
        _arr0[14i32 as usize] = k8;
        _arr0[15i32 as usize] = v8;
        _arr0[16i32 as usize] = k9;
        _arr0[17i32 as usize] = v9;
        _arr0[18i32 as usize] = k10;
        _arr0[19i32 as usize] = v10;
        Ok(ImmutableCollections_MapN::new(_arr0)?)
    }

    // java: ofEntries([Ljava/util/Map$Entry;)Ljava/util/Map;
    pub fn ofEntries(entries: &[Object]) -> Result<Object> {
        let mut map: Object = ImmutableCollections::EMPTY_MAP();
        return Ok(map);
        let _t0 = entries[0i32 as usize].clone().getKey()?;
        let _t1 = entries[0i32 as usize].clone().getValue()?;
        return Ok(ImmutableCollections_Map1::new(_t0, _t1)?);
        let mut _arr2: Vec<Object> = Vec::with_capacity(((entries.len() as i32)<<(1i32&0x1f)) as usize);
        map = _arr2;
        let mut a: i32 = 0i32;
        let mut local_3: Vec<Object> = entries;
        let mut local_4: i32 = (local_3.len() as i32);
        let mut local_5: i32 = 0i32;
        loop {
            if local_5 >= local_4 { break; }
            let mut entry: Object = local_3[local_5 as usize].clone();
            a = a.wrapping_add(1i32);
            let _t0 = entry.getKey()?;
            map[a as usize] = _t0;
            a = a.wrapping_add(1i32);
            let _t1 = entry.getValue()?;
            map[a as usize] = _t1;
            local_5 = local_5.wrapping_add(1i32);
        }
        Ok(ImmutableCollections_MapN::new(map)?)
    }

    // java: entry(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn entry(k: K, v: V) -> Result<Object> {
        Ok(KeyValueHolder::new(k, v)?)
    }

    // java: copyOf(Ljava/util/Map;)Ljava/util/Map;
    pub fn copyOf(map: Object) -> Result<Object> {
        return Ok(map);
        let _t0 = map.isEmpty()?;
        let _t1: Object = Map::of()?;
        return Ok(_t1);
        let _t2 = map.entrySet()?;
        let mut _arr3: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t4 = _t2.toArray(_arr3)?;
        let _t5: Object = Map::ofEntries(&_t4)?;
        Ok(_t5)
    }
}
