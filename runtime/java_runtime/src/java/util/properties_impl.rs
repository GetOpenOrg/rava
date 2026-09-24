//! `java/util/Properties` 手写伴生（虚槽方法体）：属性查询消费快照 `map`
//! 字段（System.initPhase1 填充的 VM 快照子集——见 system_impl.rs 的
//! registerNatives）。

use crate::prelude::*;
use super::properties::Properties;
use crate::java::lang::Object;
use crate::java::lang::String;

impl Properties {
    /// `getProperty(String)`（虚槽手写体）。JDK 语义：`Hashtable.get` 后
    /// 取 String 值——非 String 或缺席 → null；`defaults` 链缺席（构造
    /// 不设 defaults，快照构造路径亦不设）。
    pub fn __impl_getProperty_str(&self, key: String) -> Result<String> {
        self.__impl_lookup_string(key)
    }

    /// `getProperty(String, String)`：单参语义 + 缺席回落默认值。
    /// null 判定用 String wrapper 的 vtable 钩子 `is_jvm_null()`（S-3.1 唯一判定）；
    /// `_is_jnull` 只对 `Object` 载体生效（`downcast_ref::<Object>`），对 wrapper
    /// 类型恒 false——缺席键会漏回落默认值（TestAtomics 的 MethodHandleStatics
    /// clinit parseInt(null) 即此回归）。
    pub fn __impl_getProperty_str_str(&self, key: String, defaultValue: String) -> Result<String> {
        let v = self.__impl_lookup_string(key)?;
        if v.is_jvm_null() {
            Ok(defaultValue)
        } else {
            Ok(v)
        }
    }

    /// 快照查询的私有核：键与值统一经 Object 载体（map 为
    /// `ConcurrentHashMap<Object, Object>`）；值按运行时类名判 String。
    fn __impl_lookup_string(&self, key: String) -> Result<String> {
        let oval = self.__get_map().get(Object::from(key))?;
        if oval.0.is_jvm_null() {
            return Ok(String::default());
        }
        if oval.0.__class_name() == "java/lang/String" {
            return Ok(oval.try_cast::<String>("java/lang/String")?);
        }
        Ok(String::default())
    }
}
