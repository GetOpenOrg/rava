//! `java/security/SecureRandom` 手写伴生：边界类（java/security/ 前缀），按调用链按需实现（K-2 规则）。
//!
//! K-JCA：JDK 的 `new SecureRandom()` 经 provider 列表选缺省 PRNG（Linux 上 SUN 的
//! `NativePRNG`：读 `/dev/urandom`，并以 SHA1PRNG 混合）。原生二进制直接取操作系统熵源
//! （`/dev/urandom`）——与 NativePRNG 的「非阻塞读内核熵池」语义一致，不经 provider 查找、
//! 不引入 NativePRNG / DRBG 的实现闭包。`getAlgorithm` / `getProvider` 报告 JDK 缺省值
//! （NativePRNG / SUN）。`next(bits)` 按 JDK 字节码语义由 `nextBytes` 组装；`Random` 的
//! nextInt / nextLong / nextDouble 等经虚调用 `next` 走本实现。`setSeed` 为补充熵，
//! 对内核熵源无影响（JDK NativePRNG 同样不因 setSeed 变为确定序列）。

use crate::prelude::*;
use super::secure_random::implref::SecureRandom;
use super::provider::implref::Provider;
use std::io::Read;

/// 以操作系统熵源填满缓冲；读失败 → `ProviderException("nextBytes() failed")`
///（JDK NativePRNG.RandomIO.implNextBytes 同一异常形态）。
fn fill_os_random(buf: &mut [u8]) -> Result<()> {
    let ok = std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(buf)).is_ok();
    if ok {
        return Ok(());
    }
    match super::ProviderException::new_str(String::from("nextBytes() failed")) {
        Ok(ex) => Err(ex.into()),
        Err(nested) => Err(nested),
    }
}


impl SecureRandom {
    /// `<init>()`：缺省 PRNG（见模块说明）。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Self::__init_on(this)
    }

    #[doc(hidden)]
    pub fn __init_on(this: Self) -> Result<Self> {
        this.__set_algorithm(String::from("NativePRNG"));
        this.__set_provider(Provider::__for_name("SUN"));
        this.__set_threadSafe(true);
        Ok(this)
    }

    /// `<init>(byte[] seed)`：种子仅作补充熵（见模块说明）。
    #[jvm_boundary]
    pub fn new_arr_b(_seed: JArray<i8>) -> Result<Self> {
        Self::new()
    }

    pub fn __impl_getAlgorithm(&self) -> Result<String> {
        Ok(self.__get_algorithm())
    }

    /// `nextBytes(byte[])`：操作系统熵源。
    #[jvm_boundary(upcalls = "java/security/ProviderException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_nextBytes_arr_b(&self, bytes: JArray<i8>) -> Result<()> {
        let n = bytes.len()?;
        let mut buf = vec![0u8; n as usize];
        fill_os_random(&mut buf)?;
        for (i, b) in buf.iter().enumerate() {
            bytes.set(i as i32, *b as i8)?;
        }
        Ok(())
    }

    /// `next(int numBits)`：JDK 字节码语义——取 ⌈numBits/8⌉ 个随机字节大端拼接，右移去掉多余位。
    #[jvm_boundary(upcalls = "java/security/ProviderException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_next(&self, numBits: i32) -> Result<i32> {
        let num_bytes = ((numBits + 7) / 8) as usize;
        let mut buf = vec![0u8; num_bytes];
        fill_os_random(&mut buf)?;
        let mut next: i32 = 0;
        for b in &buf {
            next = next.wrapping_shl(8).wrapping_add(*b as i32);
        }
        Ok(((next as u32) >> (num_bytes as u32 * 8 - numBits as u32)) as i32)
    }

    /// `setSeed(long)` / `setSeed(byte[])`：补充熵，对内核熵源无可观测影响。
    pub fn __impl_setSeed_l(&self, _seed: i64) -> Result<()> {
        Ok(())
    }

    pub fn __impl_setSeed_arr_b(&self, _seed: JArray<i8>) -> Result<()> {
        Ok(())
    }

    /// `generateSeed(int)`：操作系统熵源。
    #[jvm_boundary(upcalls = "java/security/ProviderException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_generateSeed(&self, numBytes: i32) -> Result<JArray<i8>> {
        let mut buf = vec![0u8; numBytes.max(0) as usize];
        fill_os_random(&mut buf)?;
        Ok(JArray::from(buf.into_iter().map(|b| b as i8).collect::<Vec<i8>>()))
    }
}
