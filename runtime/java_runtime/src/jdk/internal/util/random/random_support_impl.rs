//! `jdk/internal/util/random/RandomSupport` 手写伴生：内部边界类，按调用链
//! 按需实现（K-2 规则），其余保持 panic 存根。

use crate::prelude::*;
use crate::java::lang::IllegalArgumentException;
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

    /// `checkBound(int)V`：非正边界抛 `IllegalArgumentException("bound must be
    /// positive")`（按 OpenJDK 21 字节码逐指令还原）。消费方：有界随机数
    /// 入口（`nextInt(bound)` / `nextLong(bound)` 等）的参数前置校验。
    #[jvm_boundary(upcalls = "java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V")]
    pub fn checkBound_i(bound: i32) -> Result<()> {
        if bound <= 0 {
            return Err(JvmError::from(IllegalArgumentException::new_str(
                String::from("bound must be positive"))?));
        }
        Ok(())
    }

    /// `boundedNextInt(RandomGenerator, int)I`：有界nextInt 的均匀化核心，按
    /// OpenJDK 21 字节码逐指令还原——`r = rng.nextInt()`；`m = bound - 1`；
    /// bound 为 2 的幂 → `r &= m`；否则拒绝采样循环
    /// （`u = r >>> 1`；`u + m - (r = u % bound) < 0` 时 `u = rng.nextInt() >>> 1`
    /// 重来，加减按 JVM 二进制补码回绕）。`rng.nextInt()` 经接口载体分派
    /// （itable 语义，不枚举实现类）。
    #[jvm_boundary(upcalls = "java/util/random/RandomGenerator.nextInt:()I")]
    pub fn boundedNextInt_randomgenerator_i(rng: Object, bound: i32) -> Result<i32> {
        let next_int = || -> Result<i32> {
            Into::<crate::java::util::random::RandomGenerator>::into(Clone::clone(&rng)).nextInt()
        };
        let m = bound.wrapping_sub(1);
        let mut r = next_int()?;
        if (bound & m) == 0 {
            r &= m;
        } else {
            let mut u = ((r as u32) >> 1) as i32;
            loop {
                r = irem(u, bound)?;
                if u.wrapping_add(m).wrapping_sub(r) >= 0 {
                    break;
                }
                u = ((next_int()? as u32) >> 1) as i32;
            }
        }
        Ok(r)
    }
}
