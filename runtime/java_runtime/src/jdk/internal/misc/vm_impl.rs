use crate::prelude::*;
use super::vm::VM;
use crate::java::lang::String;

/// 停机标记（进程级）。
static SHUTDOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

impl VM {
    /// 原生二进制进入 main 时运行时已完成初始化（对应 initLevel == SYSTEM_BOOTED）。
    #[jvm_boundary]
    pub fn isBooted() -> Result<bool> {
        Ok(true)
    }

    /// `shutdown()` / `isShutdown()`：停机标记（JDK 语义为 initLevel 置 SYSTEM_SHUTDOWN）。
    /// Shutdown.runHooks 据此保证 hook 只运行一次（System.exit 与 DestroyJavaVM 并发时）。
    #[jvm_boundary]
    pub fn shutdown() -> Result<()> {
        SHUTDOWN.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    #[jvm_boundary]
    pub fn isShutdown() -> Result<bool> {
        Ok(SHUTDOWN.load(std::sync::atomic::Ordering::SeqCst))
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
        crate::__process_static! {
            static INVOKE_INITED: crate::sync_model::__PrimCell<bool> = const { crate::sync_model::__PrimCell::new(false) };
        }
        INVOKE_INITED.with(|f| f.set(true));
        Ok(())
    }

    pub fn isJavaLangInvokeInited() -> Result<bool> {
        crate::__process_static! {
            static INVOKE_INITED: crate::sync_model::__PrimCell<bool> = const { crate::sync_model::__PrimCell::new(false) };
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

    /// `latestUserDefinedLoader()`：调用栈上最近的用户定义类加载器。原生二进制无类加载器
    /// 层级（全部类静态链接进同一镜像）→ null（bootstrap 视图）；消费方
    /// `ObjectInputStream.resolveClass` 据此以 `Class.forName(name, false, null)` 按名解析。
    #[jvm_boundary]
    pub fn latestUserDefinedLoader() -> Result<crate::java::lang::ClassLoader> {
        Ok(Default::default())
    }

    /// `isSystemDomainLoader(ClassLoader)`：boot / platform 加载器判定。原生单二进制只有一层
    /// 类宇宙（全部等价于系统域）→ 恒真。消费方：MethodType 的缓存保活判定（MH-native）。
    #[jvm_boundary]
    pub fn isSystemDomainLoader(_loader: crate::java::lang::ClassLoader) -> Result<bool> {
        Ok(true)
    }
}
