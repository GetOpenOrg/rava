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
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/util/AbstractMap.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public"))]
    pub fn clear(&self) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn compute(&self, key: K, rf: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public"))]
    pub fn computeIfAbsent(&self, key: K, mf: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn computeIfPresent(&self, key: K, rf: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn merge(&self, key: K, value: V, rf: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn put(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public"))]
    pub fn putAll(&self, m: Object) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn putIfAbsent(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public"))]
    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: K, value: V) -> Result<V> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public"))]
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: K, oldValue: V, newValue: V) -> Result<bool> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public"))]
    pub fn replaceAll(&self, f: Object) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn getOrDefault(&self, key: Object, defaultValue: V) -> Result<V> {
        let this = self;
        let _t0 = this.get(key)?;
        let mut v: Object = _t0;
        Ok(defaultValue)
    }
}
