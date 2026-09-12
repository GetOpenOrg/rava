#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$UnmodifiableMap",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Map,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_UnmodifiableMap<K, V> {
    #[cfg_attr(any(), java_field(name = "m", descriptor = "Ljava/util/Map;", access = "final"))]
    pub m: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub keySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "private"))]
    pub values: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_UnmodifiableMap<K, V> {
    // java: <init>(Ljava/util/Map;)V
    pub fn new(m: Object) -> Result<Self> {
        let this = Self { m: Field::new(Default::default()), keySet: Field::new(Default::default()), entrySet: Field::new(Default::default()), values: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        return Err(JvmError::Custom("athrow".to_owned()));
        this.m.set(m);
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
    pub fn containsValue(&self, val: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.m.get().containsValue(val)?;
        Ok(_t0)
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<V> {
        let this = self;
        let _t0 = this.m.get().get(key)?;
        Ok(_t0)
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: K, value: V) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, m: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.m.get().keySet()?;
        let _t1: Object = Collections::unmodifiableSet(_t0)?;
        this.keySet.set(_t1);
        Ok(this.keySet.get())
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.m.get().entrySet()?;
        this.entrySet.set(Collections_UnmodifiableMap_UnmodifiableEntrySet::new(_t0)?);
        Ok(this.entrySet.get())
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.m.get().values()?;
        let _t1: Object = Collections::unmodifiableCollection(_t0)?;
        this.values.set(_t1);
        Ok(this.values.get())
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

    // java: getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn getOrDefault(&self, k: Object, defaultValue: V) -> Result<V> {
        let this = self;
        let _t0 = this.m.get().getOrDefault(k, defaultValue)?;
        Ok(_t0)
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
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: K, value: V) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: K, value: V) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: K, mappingFunction: Object) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: K, remappingFunction: Object) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: K, remappingFunction: Object) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: K, value: V, remappingFunction: Object) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
