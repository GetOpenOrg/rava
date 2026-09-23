//! `sun/security/action/GetPropertyAction` 手写伴生：内部边界类，按调用链
//! 按需实现（K-2 规则），其余保持 panic 存根。
//!
//! 属性真源：`System.props`（System.registerNatives 在 initPhase1 对应物
//! 阶段填充的 VM 快照子集，见 system_impl.rs）。本类是特权读取的无操作
//! 包装——安全器不存在，读取即直查。

use crate::prelude::*;
use super::get_property_action::GetPropertyAction;
use crate::java::lang::String;
use crate::java::util::Properties;

impl GetPropertyAction {
    /// static `privilegedGetProperties()`：全量系统属性快照——返回
    /// `System.props`（VM 快照子集；无安全器，无需副本）。
    #[jvm_boundary(upcalls = "java/util/Properties.getProperty:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn privilegedGetProperties() -> Result<Properties> {
        crate::java::lang::System::props()
    }

    /// static `privilegedGetProperty(String)`：单属性查询，缺席 → null。
    #[jvm_boundary(upcalls = "java/util/Properties.getProperty:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn privilegedGetProperty(theProp: String) -> Result<String> {
        crate::java::lang::System::props()?.getProperty_str(theProp)
    }

    /// static `privilegedGetProperty(String, String)`：带默认值——属性缺席
    /// 时返回默认值（JDK 语义）。
    #[jvm_boundary(upcalls = "java/util/Properties.getProperty:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")]
    pub fn privilegedGetProperty_str_str(theProp: String, defaultVal: String) -> Result<String> {
        crate::java::lang::System::props()?.getProperty_str_str(theProp, defaultVal)
    }
}
