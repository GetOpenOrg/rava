use crate::prelude::*;
use super::vm::VM;
use crate::java::lang::String;

impl VM {
    /// 原生二进制进入 main 时运行时已完成初始化（对应 initLevel == SYSTEM_BOOTED）。
    #[jvm_boundary]
    pub fn isBooted() -> Result<bool> {
        Ok(true)
    }

    /// `VM.getSavedProperty(key)`：启动时保存的 JVM 内部属性。
    ///
    /// 保存属性仅在指定 `-XX:` / 归档选项时存在；原生二进制无启动选项，
    /// 所有查询都未设置 → null（如 Integer$IntegerCache 的
    /// `java.lang.Integer.IntegerCache.high`，null 表示沿用默认上界 127）。
    #[jvm_boundary]
    pub fn getSavedProperty(_key: String) -> Result<String> {
        Ok(String::default())
    }

    /// `setJavaLangInvokeInited()` / `isJavaLangInvokeInited()`：java.lang.invoke
    /// 初始化完成标记（MethodHandleNatives.<clinit> 尾声置位；读者用它区分
    /// 「引导早期」与「机制就绪」）。原生二进制的引导顺序由 BFS 闭包静态
    /// 决定，标记照 JDK 语义置位/查询（线程内）。
    pub fn setJavaLangInvokeInited() -> Result<()> {
        thread_local! {
            static INVOKE_INITED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        }
        INVOKE_INITED.with(|f| f.set(true));
        Ok(())
    }

    pub fn isJavaLangInvokeInited() -> Result<bool> {
        thread_local! {
            static INVOKE_INITED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        }
        Ok(INVOKE_INITED.with(|f| f.get()))
    }

    /// `VM.isModuleSystemInited()`：模块系统初始化完成标记。原生二进制的
    /// 类型宇宙由 BFS 闭包静态组装（进入 main 前完成），模块层概念不在
    /// 运行时呈现——调用点（如 ClassLoader 的引导期分支）语义上处于
    /// 「系统模块已就绪」档位，恒真。
    #[jvm_boundary]
    pub fn isModuleSystemInited() -> Result<bool> {
        Ok(true)
    }
}
