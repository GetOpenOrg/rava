//! `jdk/internal/logger/LazyLoggers` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 的 `System.getLogger(name)` 经 LazyLoggers 按需绑定 `System.LoggerFinder` 服务
//! （ServiceLoader + java.logging 模块的 SimpleConsoleLogger 等）。原生二进制无日志服务
//! 提供者：返回**静默 Logger**——`isLoggable` 恒 false，调用方按 JDK 惯例先判定再记录，
//! 不产生输出（与未配置日志级别的 JDK 默认观察一致：DEBUG/TRACE 级不输出）。
//! 消费方：`ObjectInputFilter$Config.<clinit>` 的 `configLog`（序列化过滤器配置日志，
//! S-66 对象序列化链首跳）；`ObjectInputStream.filterCheck` 不先判定、直接调
//! `log(Level, String, Object...)`（Logger 内部自行判级）——各 log 重载均为空操作。

use crate::prelude::*;
use super::lazy_loggers::LazyLoggers;
use crate::java::lang::{Module, System_Logger__VTable, System_Logger_Level, Throwable};
use crate::java::util::ResourceBundle;
use std::rc::Rc;

/// 静默 Logger（`System.Logger` 接口实现对象，经 `ObjectVTable::__interface` 应答）。
struct SilentLogger {
    name: String,
}

impl System_Logger__VTable for SilentLogger {
    fn getName(&self) -> Result<String> {
        Ok(Clone::clone(&self.name))
    }

    fn isLoggable(&self, _level: System_Logger_Level) -> Result<bool> {
        Ok(false)
    }

    fn log_system_logger_level_str(&self, _level: System_Logger_Level, _msg: String) -> Result<()> {
        Ok(())
    }

    fn log_system_logger_level_str_throwable(&self, _level: System_Logger_Level, _msg: String,
                                             _thrown: Throwable) -> Result<()> {
        Ok(())
    }

    fn log_system_logger_level_str_arr_obj(&self, _level: System_Logger_Level, _format: String,
                                           _params: JArray<Object>) -> Result<()> {
        Ok(())
    }

    fn log_system_logger_level_resourcebundle_str_throwable(&self, _level: System_Logger_Level,
                                                            _bundle: ResourceBundle, _msg: String,
                                                            _thrown: Throwable) -> Result<()> {
        Ok(())
    }

    fn log_system_logger_level_resourcebundle_str_arr_obj(&self, _level: System_Logger_Level,
                                                          _bundle: ResourceBundle, _format: String,
                                                          _params: JArray<Object>) -> Result<()> {
        Ok(())
    }
}

impl ObjectVTable for SilentLogger {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str { "jdk/internal/logger/LazyLoggers$JdkLazyLogger" }
    fn __obj_str(&self) -> std::string::String {
        format!("jdk.internal.logger.LazyLoggers$JdkLazyLogger@{}", self.name)
    }
    /// 接口视图查询（invokeinterface / 载体转换的运行时入口）。
    fn __interface(self: Rc<Self>, slot: &mut dyn std::any::Any) {
        if let Some(s) = slot.downcast_mut::<Option<Rc<dyn System_Logger__VTable>>>() {
            *s = Some(self);
        }
    }
}

impl LazyLoggers {
    /// `getLogger(String, Module)`：静默 Logger（见模块说明）。
    #[jvm_boundary]
    pub fn getLogger(name: String, _module: Module) -> Result<Object> {
        Ok(Object::from(SilentLogger { name }))
    }
}
