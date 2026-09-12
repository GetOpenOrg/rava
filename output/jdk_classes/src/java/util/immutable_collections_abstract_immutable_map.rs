#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$AbstractImmutableMap",
    super_class = "java/util/AbstractMap",
    interfaces  = "java/io/Serializable",
    access      = "abstract",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_AbstractImmutableMap<K, V>;

impl<K: Clone + 'static, V: Clone + 'static> ImmutableCollections_AbstractImmutableMap<K, V> {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/util/AbstractMap.<init>:()V */
        Ok(this)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: K, rf: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: K, mf: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: K, rf: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: K, value: V, rf: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, m: Object) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, f: Object) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<V> {
        let this = self;
        let _t0 = this.get(key)?;
        let mut v: Object = _t0;
        Ok(defaultValue)
    }
}
