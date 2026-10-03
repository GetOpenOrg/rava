use crate::prelude::*;
use super::vm::VM;
use crate::java::lang::String;

/// 停机标记（进程级）。
static SHUTDOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

impl VM {
    /// native `initialize()`：HotSpot 由 CDS 归档恢复 VM 类的静态字段（`JVM_InitializeFromArchive`）；
    /// 原生二进制无 CDS 归档，静态字段即 `<clinit>` 的字节码结果 → no-op。
    #[jvm_native]
    pub fn initialize() -> Result<()> {
        Ok(())
    }

    /// 引导阶段 `initLevel()`（VM 注入状态，准入第 ③ 类；HotSpot 由 initPhase1~3 经 `initLevel(int)` 写入）。
    /// 原生二进制进入 main 时为 SYSTEM_BOOTED（4）；只有正在执行引导段的线程在段内读到该段的档位：
    /// initPhase1 的属性快照段（`System::registerNatives` → `VM.saveProperties`）为 0，initPhase3 的系统类
    /// 加载器段（`ClassLoader::__vm_init_phase3`）为 SYSTEM_LOADER_INITIALIZING（3）——其余线程此时读
    /// 对应状态会在该段的互斥上等待结束，与 JVM「引导段先于任何应用线程」的时序一致。
    #[jvm_boundary]
    pub fn initLevel() -> Result<i32> {
        Ok(crate::exec_context::state().boot_level.get().unwrap_or(4))
    }

    /// 在 initLevel == `level` 下执行引导段 `f`（仅本线程可见；嵌套时退出后恢复外层档位）。
    pub fn __vm_at_init_level<R>(level: i32, f: impl FnOnce() -> R) -> R {
        let outer = crate::exec_context::state().boot_level.replace(Some(level));
        let r = f();
        crate::exec_context::state().boot_level.set(outer);
        r
    }

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

    /// `setJavaLangInvokeInited()` / `isJavaLangInvokeInited()`：java.lang.invoke 初始化完成标记。
    /// HotSpot 在 create_vm 期间即初始化 JSR 292 核心类（initialize_jsr292_core_classes：
    /// MethodHandle / MemberName / MethodHandleNatives …，后者 <clinit> 尾声置位），用户代码运行时
    /// 恒已置位。原生二进制进入 main 前处于同一「引导完成」档位（同 isModuleSystemInited）：
    /// 查询恒真，置位为 no-op。消费方：MethodHandleAccessorFactory（反射访问器，FS-R R2a）。
    pub fn setJavaLangInvokeInited() -> Result<()> {
        Ok(())
    }

    pub fn isJavaLangInvokeInited() -> Result<bool> {
        Ok(true)
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

    /// native `getNanoTimeAdjustment(long offsetInSeconds)`（Instant.now / Clock.systemUTC 的时基）：
    /// 当前 CLOCK_REALTIME 相对 offset 的纳秒差；秒差超出 ±2^32 返回 -1（HotSpot
    /// JVM_GetNanoTimeAdjustment 同阈值，调用方据此重取 offset）。
    #[jvm_native]
    pub fn getNanoTimeAdjustment(offset_in_seconds: i64) -> Result<i64> {
        const MAX_DIFF: i64 = 0x1_0000_0000;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
        let diff = now.as_secs() as i64 - offset_in_seconds;
        if diff >= MAX_DIFF || diff <= -MAX_DIFF {
            return Ok(-1);
        }
        Ok(diff * 1_000_000_000 + now.subsec_nanos() as i64)
    }
}
