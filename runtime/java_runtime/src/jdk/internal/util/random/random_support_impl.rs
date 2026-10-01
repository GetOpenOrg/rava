//! `jdk/internal/util/random/RandomSupport` 手写伴生：本类经 closure.toml [release] 按字节码翻译，
//! 仅 initialSeed 手写（secureRandomSeed 分支经 SecureRandom.getSeed 拉入整条 JCA 链）。

use crate::prelude::*;
use super::random_support::RandomSupport;

impl RandomSupport {
    /// `initialSeed()J`：默认种子的非确定性来源。`useSecureRandomSeed` 分支
    /// （VM 属性 java.util.secureRandomSeed）在原生二进制无 VM 属性面，恒走
    /// JDK 默认路径：`mixStafford13(currentTimeMillis) ^ mixStafford13(nanoTime)`
    /// ——值不进可观察输出（消费方为 SplittableRandom defaultGen 等）。
    #[jvm_boundary]
    pub fn initialSeed() -> Result<i64> {
        let t = crate::java::lang::System::currentTimeMillis()?;
        let n = crate::java::lang::System::nanoTime()?;
        Ok(Self::mixStafford13(t)? ^ Self::mixStafford13(n)?)
    }
}
