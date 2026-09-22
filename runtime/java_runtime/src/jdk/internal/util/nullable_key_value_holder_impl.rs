//! `jdk/internal/util/NullableKeyValueHolder` 手写伴生：内部边界类，按调用链
//! 按需实现（K-2 规则）。JDK 语义：不可变 (key, value) 对（键值均可 null），
//! `SequencedCollection` 视图的 `firstEntry`/`lastEntry`/`pollFirstEntry` 等
//! 以它包装内部 Map.Entry（防暴露内部条目）。
//!
//! 已知缺口（非本域）：toString 已由 `__impl_toString` 手写体承接（JDK 语义
//! `key + "=" + value`，经 vtable 槽位参与根类桥接——`_root_method_vtable_owner`
//! 只摘除 inherent 形态的手写 toString，`__impl_` 形态的声明保留在宏块内，
//! to_string_vtable 属性不摘除，与 Double/Integer 的既有先例同型）。equals /
//! hashCode 仍为占位（消费面未达）。

use crate::prelude::*;
use super::nullable_key_value_holder::NullableKeyValueHolder;
use crate::java::util::Map_Entry;

impl<K, V> NullableKeyValueHolder<K, V>
where
    K: Clone + Default + From<Object> + Into<Object> + 'static,
    V: Clone + Default + From<Object> + Into<Object> + 'static,
{
    /// `<init>(K, V)`：直存两引用（可为 null 载体）。
    #[jvm_boundary]
    pub fn new_obj_obj(k: K, v: V) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_key(k);
        this.__set_value(v);
        Ok(this)
    }

    /// `<init>(Map.Entry<K, V>)`：`Objects.requireNonNull(entry)` 后取
    /// entry 的 key/value。调用方（生成侧）已做 `try_cast_iface` 到
    /// `java/util/Map$Entry`；此处经接口载体分派 getKey/getValue。
    /// 签名随 A-4 批次 4 载体化对齐：调用点 Map$Entry 类型位置已产 Map_Entry 载体
    /// （stubs×a4b3 语义冲突第三处——同 JavaUtilCollectionAccess 族）。
    #[jvm_boundary]
    pub fn new_map_entry(entry: Map_Entry<Object, Object>) -> Result<Self> {
        let k = entry.getKey()?;
        let v = entry.getValue()?;
        let mut this = Self::default();
        this._init_not_null();
        this.__set_key(K::from(k));
        this.__set_value(V::from(v));
        Ok(this)
    }

    /// `toString()`：`key + "=" + value`（JDK 语义；键值 null 经 Object 的
    /// null vtable 呈现 "null"）。手写体取 `__impl_toString` 形态而非 inherent
    /// `toString`：方法声明保留在 java_class! 宏块内（`body = "handwritten"`），
    /// 经 vtable 槽位参与根类桥接与子类覆盖（to_string_vtable 属性不摘除），
    /// 字符串拼接（makeConcat → Object::toString → __obj_str）由此取到本实现
    /// 而非生成侧的边界类占位（BINARY_NAME）。
    #[jvm_boundary]
    pub fn __impl_toString(&self) -> Result<String> {
        let k: Object = Into::into(Clone::clone(&self.__get_key()));
        let v: Object = Into::into(Clone::clone(&self.__get_value()));
        Ok(String::from(format!("{}={}", k, v).as_str()))
    }
}
