use crate::prelude::*;
use super::cds::CDS;
use crate::java::lang::Class;

// 内部边界类 jdk.internal.misc.CDS：按调用链按需实现，其余保持 panic 存根。

impl CDS {
    /// `CDS.initializeFromArchive(cls)`：从 CDS 归档恢复类的静态字段。
    ///
    /// 原生二进制不携带归档，无任何可恢复的静态状态 → no-op。
    /// 调用方（如 Integer$IntegerCache）随后检查归档字段仍为 null，
    /// 走常规初始化路径，与无归档启动的 JVM 行为一致。
    #[jvm_boundary]
    pub fn initializeFromArchive(_arg0: Class) -> Result<()> {
        Ok(())
    }
}
