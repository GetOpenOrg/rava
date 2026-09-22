//! `jdk/internal/util/random/RandomSupport` 手写伴生：内部边界类，按调用链
//! 按需实现（K-2 规则），其余保持 panic 存根。

use crate::prelude::*;
use super::random_support::RandomSupport;

impl RandomSupport {
    /// `mixMurmur64(long z)J`：Murmur3 的 64 位 finalizer（两轮
    /// `(z ^ (z >>> 33)) * K` 再异或折叠）。按 OpenJDK 21 字节码逐指令还原
    /// （javap 常量 -49064778989728563 = 0xff51afd7ed558ccd、
    /// -4265267296055464877 = 0xc4ceb9fe1a85ec53）。消费方：
    /// `ThreadLocalRandom.<clinit>` 的 seeder 种子混合、`initialSeed` 等。
    #[jvm_boundary]
    pub fn mixMurmur64(mut z: i64) -> Result<i64> {
        z = (z ^ ((z as u64 >> 33) as i64)).wrapping_mul(-49064778989728563i64);
        z = (z ^ ((z as u64 >> 33) as i64)).wrapping_mul(-4265267296055464877i64);
        Ok(z ^ ((z as u64 >> 33) as i64))
    }

    /// `mixStafford13(long z)J`：Stafford13 混合（splitmix64 finalizer，三轮
    /// `z ^= z >>> s` 交错乘法）。按 OpenJDK 21 字节码逐指令还原（javap 常量
    /// -4658895280553007687 = 0xbf58476d1ce4e5b9、
    /// -7723592293110705685 = 0x94d049bb133111eb）。消费方：`initialSeed`、
    /// `SplittableRandom` 的默认种子。
    #[jvm_boundary]
    pub fn mixStafford13(mut z: i64) -> Result<i64> {
        z = (z ^ ((z as u64 >> 30) as i64)).wrapping_mul(-4658895280553007687i64);
        z = (z ^ ((z as u64 >> 27) as i64)).wrapping_mul(-7723592293110705685i64);
        Ok(z ^ ((z as u64 >> 31) as i64))
    }

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
