#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedMap",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Map,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedMap<K, V> {
    #[cfg_attr(any(), java_field(name = "m", descriptor = "Ljava/util/Map;", access = "private final"))]
    pub m: Field<Object>,
    #[cfg_attr(any(), java_field(name = "mutex", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub mutex: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub keySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "private"))]
    pub values: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_SynchronizedMap<K, V> {
    // java: <init>(Ljava/util/Map;)V
    // java: <init>(Ljava/util/Map;)V
    pub fn new__map(m: Object) -> Result<Self> {
        let this = Self { m: Field::new(Default::default()), mutex: Field::new(Default::default()), keySet: Field::new(Default::default()), entrySet: Field::new(Default::default()), values: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(m)?;
        this.m.set(_t0);
        this.mutex.set(this);
        Ok(this)
    }

    // java: <init>(Ljava/util/Map;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/Map;Ljava/lang/Object;)V
    pub fn new__map_obj(m: Object, mutex: Object) -> Result<Self> {
        let this = Self { m: Field::new(Default::default()), mutex: Field::new(Default::default()), keySet: Field::new(Default::default()), entrySet: Field::new(Default::default()), values: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.m.set(m);
        this.mutex.set(mutex);
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().size()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: i32 = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().isEmpty()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().containsKey(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, value: Object) -> Result<bool> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().containsValue(value)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<V> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().get(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().put(key, value)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<V> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().remove(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, map: Object) -> Result<()> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.m.get().putAll(map)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.m.get().clear()?;
        /* TODO: monitorexit  */
        let mut local_2: Object = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().keySet()?;
        this.keySet.set(Collections_SynchronizedSet::new(_t0, this.mutex.get())?);
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = this.keySet.get();
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().entrySet()?;
        this.entrySet.set(Collections_SynchronizedSet::new(_t0, this.mutex.get())?);
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = this.entrySet.get();
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().values()?;
        this.values.set(Collections_SynchronizedCollection::new(_t0, this.mutex.get())?);
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = this.values.get();
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().equals(o)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().hashCode()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: i32 = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().toString()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: String = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn getOrDefault(&self, k: Object, defaultValue: V) -> Result<V> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().getOrDefault(k, defaultValue)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.m.get().forEach(action)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        this.m.get().replaceAll(function)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().putIfAbsent(key, value)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().remove(key, value)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
        let this = self;
        let mut local_4: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().replace(key, oldValue, newValue)?;
        /* TODO: monitorexit  */
        return Ok(local_4);
        let mut local_5: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().replace(key, value)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: K, mappingFunction: Object) -> Result<V> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().computeIfAbsent(key, mappingFunction)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<V> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().computeIfPresent(key, remappingFunction)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: K, remappingFunction: Object) -> Result<V> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().compute(key, remappingFunction)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<V> {
        let this = self;
        let mut local_4: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.m.get().merge(key, value, remappingFunction)?;
        /* TODO: monitorexit  */
        return Ok(local_4);
        let mut local_5: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        s.defaultWriteObject()?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
