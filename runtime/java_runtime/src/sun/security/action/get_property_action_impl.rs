//! `sun/security/action/GetPropertyAction` 手写伴生：内部边界类，按调用链
//! 按需实现（K-2 规则），其余保持 panic 存根。
//!
//! 消费链：MethodHandleStatics.<clinit>（DEBUG 属性扫描）。原生二进制无
//! -D 注入机制，系统属性恒空——privilegedGetProperties 返回空 Properties
//! （属性缺席的 JDK 等价语义）；privilegedGetProperty 恒缺席值。

use crate::prelude::*;
use super::get_property_action::GetPropertyAction;
use crate::java::lang::String;
use crate::java::util::Properties;
use crate::java::util::concurrent::ConcurrentHashMap;

impl GetPropertyAction {
    /// static `privilegedGetProperties()`：全量系统属性快照——原生二进制
    /// 属性恒空，返回空 Properties。构造不经 JDK 构造器链（Properties.<init>
    /// → Hashtable 族的种子在 sig_types 载体化上有 codegen 域缺口，本域禁改），
    /// 按擦除字段协议直接挂空后备 ConcurrentHashMap（Properties.getProperty
    /// 消费 `map` 字段——空表 → 全部查询缺席，与「属性恒缺席」语义一致）。
    pub fn privilegedGetProperties() -> Result<Properties> {
        let mut p = Properties::default();
        p._init_not_null();
        p.__set_map(ConcurrentHashMap::<Object, Object>::new()?);
        Ok(p)
    }

    /// static `privilegedGetProperty(String)`：单属性查询——恒缺席 → null。
    pub fn privilegedGetProperty(theProp: String) -> Result<String> {
        let _ = theProp;
        Ok(String::default())
    }

    /// static `privilegedGetProperty(String, String)`：带默认值——属性缺席
    /// 时返回默认值（JDK 语义）。
    pub fn privilegedGetProperty_str_str(theProp: String, defaultVal: String) -> Result<String> {
        let _ = theProp;
        Ok(defaultVal)
    }
}
