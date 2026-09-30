use crate::prelude::*;
use super::EventHelper;

// jdk.internal.event.EventHelper（JFR 事件的 System.Logger 旁路日志）：按调用链按需实现，其余保持存根。

impl EventHelper {
    /// `isLoggingSecurity()Z`：`jdk.event.security` 日志器是否在 DEBUG 级可记录。JDK 缺省日志
    /// 配置（INFO）下恒 false；原生二进制无日志配置文件，同取缺省——安全事件旁路日志不输出。
    #[jvm_boundary]
    pub fn isLoggingSecurity() -> Result<bool> {
        Ok(false)
    }
}
