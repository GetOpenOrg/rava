//! `jdk.internal.misc.VM` 的 native 层。
//!
//! 引导档位（`initLevel`）、停机、保存属性、`java.lang.invoke` 初始化标记等全部按字节码读写 VM 的静态字段：
//! 构建期引导映像执行 initPhase1–3（计划 2026-10-05-boot-image-evaluator），运行期从映像中的
//! `initLevel = SYSTEM_BOOTED`、`savedProps` 快照与 `javaLangInvokeInited` 开始，与 HotSpot 同为单一字段。
//! 残差步骤的构建期档位由启动序列写入同一字段（`[concrete.boot] level`）。

use crate::prelude::*;
use super::vm::VM;

impl VM {
    /// native `initialize()`：HotSpot 由 CDS 归档恢复 VM 类的静态字段（`JVM_InitializeFromArchive`）；
    /// 原生二进制无 CDS 归档，静态字段即 `<clinit>` 的字节码结果 → no-op。
    #[jvm_native]
    pub fn initialize() -> Result<()> {
        Ok(())
    }

    /// native `latestUserDefinedLoader0()`（HotSpot `JVM_LatestUserDefinedLoader`）：自调用栈顶起，首个由
    /// 非引导、非平台加载器定义的类的定义加载器；反射访问器（`MethodAccessorImpl` /
    /// `ConstructorAccessorImpl` 子类）的帧跳过。栈上无此类帧 → null（字节码 `latestUserDefinedLoader`
    /// 回落平台加载器）。消费方：`ObjectInputStream.resolveClass`。帧源见 `crate::vm_stack`。
    #[jvm_native]
    pub fn latestUserDefinedLoader0() -> Result<crate::java::lang::ClassLoader> {
        let platform = Object::from(crate::jdk::internal::loader::ClassLoaders::platformClassLoader()?);
        for f in crate::vm_stack::capture_java_frames() {
            if f.class_extends("jdk/internal/reflect/MethodAccessorImpl") || f.class_extends("jdk/internal/reflect/ConstructorAccessorImpl") {
                continue;
            }
            let c = crate::java::lang::Class::for_class(String::from(f.class));
            let loader = c.__vm_defining_loader()?.__get_classLoader();
            let lo = Object::from(Clone::clone(&loader));
            if !lo.0.is_jvm_null() && lo.0.__identity() != platform.0.__identity() {
                return Ok(loader);
            }
        }
        Ok(Default::default())
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
