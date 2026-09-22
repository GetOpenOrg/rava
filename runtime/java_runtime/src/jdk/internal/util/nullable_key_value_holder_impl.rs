//! `jdk/internal/util/NullableKeyValueHolder` 手写伴生：内部边界类，按调用链
//! 按需实现（K-2 规则）。JDK 语义：不可变 (key, value) 对（键值均可 null），
//! `SequencedCollection` 视图的 `firstEntry`/`lastEntry`/`pollFirstEntry` 等
//! 以它包装内部 Map.Entry（防暴露内部条目）。
//!
//! 已知缺口（非本域）：toString（`key + "=" + value`）无法进入可观察输出——
//! 内部边界类的方法整体为生成侧占位（规则 3b，upcalls 不能使边界类方法翻译），
//! 手写 toString 又会使生成侧按设计摘除 to_string_vtable 桥接
//! （`_root_method_vtable_owner`：手写 inherent 方法 → None），字符串拼接的
//! Display 捷径因此回落 `std::any::type_name`。需 emitter 侧为「边界类手写
//! toString」保留桥接（codegen 域）。equals/hashCode 同为占位（消费面未达）。

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
    #[jvm_boundary]
    pub fn new_map_entry(entry: Object) -> Result<Self> {
        if entry.0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let e = Into::<Map_Entry<Object, Object>>::into(entry);
        let k = e.getKey()?;
        let v = e.getValue()?;
        let mut this = Self::default();
        this._init_not_null();
        this.__set_key(K::from(k));
        this.__set_value(V::from(v));
        Ok(this)
    }
}
