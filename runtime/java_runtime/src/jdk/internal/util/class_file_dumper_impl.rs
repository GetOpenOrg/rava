//! `jdk/internal/util/ClassFileDumper` 手写伴生：内部边界类，按调用链按需
//! 实现（K-2 规则），其余保持 panic 存根。
//!
//! 消费链：`MethodHandles$Lookup.<clinit>` → `getInstance("jdk.invoke.
//! MethodHandle.dumpClassFiles", "DUMP_CLASS_FILES")`——HotSpot 按 key 读
//! 系统属性决定是否转储生成的字节码；原生二进制没有「转储 .class 文件」
//! 的对应物（无运行时字节码），属性在原生环境永未设置——恒返回禁用实例
//! （enabled=false），`isEnabled` 恒 false，dumpClass 族永不被调用（调用方
//! 按 isEnabled 门控），转储路径保持 stub。

use crate::prelude::*;
use super::class_file_dumper::ClassFileDumper;
use crate::java::util::concurrent::atomic::AtomicInteger;

impl ClassFileDumper {
    /// static `getInstance(String key, String path)`：无系统属性机制 →
    /// 禁用实例（JDK 属性缺席路径的等价物：`new ClassFileDumper(key, null)`，
    /// enabled=false）。同一 key 返回新实例即可（JDK 有 DUMPER_MAP 缓存，
    /// 消费方只存不比对，身份语义不进可观察输出）。
    #[jvm_boundary(upcalls = "java/util/concurrent/atomic/AtomicInteger.<init>:(I)V")]
    pub fn getInstance(_key: String, _path: String) -> Result<ClassFileDumper> {
        let mut d = ClassFileDumper::default();
        d._init_not_null();
        d.__set_key(_key);
        d.__set_dumpDir(String::default());
        d.__set_enabled(false);
        d.__set_counter(AtomicInteger::new_i(0)?);
        Ok(d)
    }

    /// `isEnabled()`：转储开关——原生二进制无字节码可转储，恒 false。
    pub fn isEnabled(&self) -> Result<bool> {
        Ok(false)
    }
}
