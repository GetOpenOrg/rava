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

    /// `CDS.getRandomSeedForDumping:()J`：CDS 归档转储期（-Xshare:dump）用的随机
    /// 种子。HotSpot JVM_GetRandomSeedForDumping 仅在转储期返回 os 种子，
    /// **正常运行路径恒返回 0**。唯一消费方 ImmutableCollections.<clinit> 在
    /// seed==0 时回落 System.nanoTime()——与无归档 JVM 的常规路径逐字一致，
    /// 该值不进入任何可观测输出（compact strings / SALT 散列种子均为内部状态）。
    #[jvm_native]
    pub fn getRandomSeedForDumping() -> Result<i64> {
        Ok(0)
    }
}
