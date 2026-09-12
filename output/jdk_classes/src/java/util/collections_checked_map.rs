#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedMap",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Map,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedMap<K, V> {
    #[cfg_attr(any(), java_field(name = "m", descriptor = "Ljava/util/Map;", access = "private final"))]
    pub m: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keyType", descriptor = "Ljava/lang/Class;", access = "final"))]
    pub keyType: Field<Object>,
    #[cfg_attr(any(), java_field(name = "valueType", descriptor = "Ljava/lang/Class;", access = "final"))]
    pub valueType: Field<Object>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_CheckedMap<K, V> {
    // java: typeCheck(Ljava/lang/Object;Ljava/lang/Object;)V
    // java: typeCheck(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn typeCheck__obj_obj(&self, key: Object, value: Object) -> Result<()> {
        let this = self;
        let _t0 = this.keyType.get().isInstance(key)?;
        let _t1 = this.badKeyMsg(key)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2 = this.valueType.get().isInstance(value)?;
        let _t3 = this.badValueMsg(value)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: typeCheck(Ljava/util/function/BiFunction;)Ljava/util/function/BiFunction;
    // java: typeCheck(Ljava/util/function/BiFunction;)Ljava/util/function/BiFunction;
    pub fn typeCheck__bifunc(&self, func: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(func)?;
        /* TODO: invokedynamic 35 */
        Ok(func)
    }

    // java: badKeyMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badKeyMsg(&self, key: Object) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Attempt to insert"))?;
        let _t0 = key.getClass()?;
        String::new().append(&_t0)?;
        String::new().append(&String::from("key into map with key type"))?;
        String::new().append(&this.keyType.get())?;
        Ok(String::new())
    }

    // java: badValueMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badValueMsg(&self, value: Object) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Attempt to insert"))?;
        let _t0 = value.getClass()?;
        String::new().append(&_t0)?;
        String::new().append(&String::from("value into map with value type"))?;
        String::new().append(&this.valueType.get())?;
        Ok(String::new())
    }

    // java: <init>(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)V
    pub fn new(m: Object, keyType: Object, valueType: Object) -> Result<Self> {
        let this = Self { m: Field::new(Default::default()), keyType: Field::new(Default::default()), valueType: Field::new(Default::default()), entrySet: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(m)?;
        this.m.set(_t0);
        let _t1: Object = Objects::requireNonNull__obj(keyType)?;
        this.keyType.set(_t1);
        let _t2: Object = Objects::requireNonNull__obj(valueType)?;
        this.valueType.set(_t2);
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.m.get().size()?;
        Ok(_t0)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().isEmpty()?;
        Ok(_t0)
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().containsKey(key)?;
        Ok(_t0)
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, v: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().containsValue(v)?;
        Ok(_t0)
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<V> {
        let this = self;
        let _t0 = this.m.get().get(key)?;
        Ok(_t0)
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<V> {
        let this = self;
        let _t0 = this.m.get().remove(key)?;
        Ok(_t0)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.m.get().clear()?;
        Ok(())
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.m.get().keySet()?;
        Ok(_t0)
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.m.get().values()?;
        Ok(_t0)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().equals(o)?;
        Ok(_t0!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.m.get().hashCode()?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.m.get().toString()?;
        Ok(_t0)
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: K, value: V) -> Result<V> {
        let this = self;
        this.typeCheck(key, value)?;
        let _t0 = this.m.get().put(key, value)?;
        Ok(_t0)
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, t: Object) -> Result<()> {
        let this = self;
        let _t0 = t.entrySet()?;
        let _t1 = _t0.toArray()?;
        let mut entries: Vec<Object> = _t1;
        let mut checked: ArrayList<_> = ArrayList::<_>::new()?;
        let mut local_4: Vec<Object> = entries;
        let mut e: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= e { break; }
            let mut o: Object = local_4[local_6 as usize].clone();
            let mut e: Object = o;
            let _t0 = e.getKey()?;
            let mut k: Object = _t0;
            let _t1 = e.getValue()?;
            let mut v: Object = _t1;
            this.typeCheck(k, v)?;
            let _t2 = checked.add(AbstractMap_SimpleImmutableEntry::new(k, v)?)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t2 = checked.iterator()?;
        local_4 = _t2;
        loop {
            let _t0 = local_4.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_4.next()?;
            e = _t0;
            let _t1 = e.getKey()?;
            let _t2 = e.getValue()?;
            let _t3 = this.m.get().put(_t1, _t2)?;
        }
        Ok(())
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.m.get().entrySet()?;
        this.entrySet.set(Collections_CheckedMap_CheckedEntrySet::new(_t0, this.valueType.get())?);
        Ok(this.entrySet.get())
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        this.m.get().forEach(action)?;
        Ok(())
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        let this = self;
        let _t0 = this.typeCheck(function)?;
        this.m.get().replaceAll(_t0)?;
        Ok(())
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: K, value: V) -> Result<V> {
        let this = self;
        this.typeCheck(key, value)?;
        let _t0 = this.m.get().putIfAbsent(key, value)?;
        Ok(_t0)
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().remove(key, value)?;
        Ok(_t0)
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
        let this = self;
        this.typeCheck(key, newValue)?;
        let _t0 = this.m.get().replace(key, oldValue, newValue)?;
        Ok(_t0)
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: K, value: V) -> Result<V> {
        let this = self;
        this.typeCheck(key, value)?;
        let _t0 = this.m.get().replace(key, value)?;
        Ok(_t0)
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: K, mappingFunction: Object) -> Result<V> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(mappingFunction)?;
        /* TODO: invokedynamic 195 */
        let _t1 = key.computeIfAbsent(this, mappingFunction)?;
        Ok(_t1)
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<V> {
        let this = self;
        let _t0 = this.typeCheck(remappingFunction)?;
        let _t1 = this.m.get().computeIfPresent(key, _t0)?;
        Ok(_t1)
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: K, remappingFunction: Object) -> Result<V> {
        let this = self;
        let _t0 = this.typeCheck(remappingFunction)?;
        let _t1 = this.m.get().compute(key, _t0)?;
        Ok(_t1)
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<V> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(remappingFunction)?;
        /* TODO: invokedynamic 209 */
        let _t1 = key.merge(value, this, remappingFunction)?;
        Ok(_t1)
    }
}
