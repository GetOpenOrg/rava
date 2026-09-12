#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SingletonMap",
    super_class = "java/util/AbstractMap",
    interfaces  = "java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SingletonMap<K, V> {
    #[cfg_attr(any(), java_field(name = "k", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub k: Field<Object>,
    #[cfg_attr(any(), java_field(name = "v", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub v: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub keySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "private"))]
    pub values: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_SingletonMap<K, V> {
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new(key: K, value: V) -> Result<Self> {
        let this = Self { k: Field::new(Default::default()), v: Field::new(Default::default()), keySet: Field::new(Default::default()), entrySet: Field::new(Default::default()), values: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractMap.<init>:()V */
        this.k.set(key);
        this.v.set(value);
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(1i32)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        let this = self;
        let _t0: bool = Collections::eq(key, this.k.get())?;
        Ok(_t0)
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, value: Object) -> Result<bool> {
        let this = self;
        let _t0: bool = Collections::eq(value, this.v.get())?;
        Ok(_t0)
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<V> {
        let this = self;
        let _t0: bool = Collections::eq(key, this.k.get())?;
        /* TODO: aconst_null  */
        Ok(this.v.get())
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Collections::singleton(this.k.get())?;
        this.keySet.set(_t0);
        Ok(this.keySet.get())
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Collections::singleton(AbstractMap_SimpleImmutableEntry::new(this.k.get(), this.v.get())?)?;
        this.entrySet.set(_t0);
        Ok(this.entrySet.get())
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Collections::singleton(this.v.get())?;
        this.values.set(_t0);
        Ok(this.values.get())
    }

    // java: getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<V> {
        let this = self;
        let _t0: bool = Collections::eq(key, this.k.get())?;
        Ok(defaultValue)
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        action.accept(this.k.get(), this.v.get())?;
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
    pub fn remove(&self, key: Object, value: Object) -> Result<bool> {
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

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Objects::hashCode(this.k.get())?;
        let _t1: i32 = Objects::hashCode(this.v.get())?;
        Ok((_t0^_t1))
    }
}
