//! 缓存键指纹：128 位非密码学流式哈希（两条独立的 64 位乘法混合通道，按 16 字节块吞入，小端定序）。
//!
//! 只用于缓存命中判定，不抗恶意构造；两条通道各自 64 位，偶然碰撞概率约 2^-128。
//! 输入按「标签 + 长度 + 内容」分帧（[`Fp::field`]），相邻字段的边界不会互相吞并。

const P1: u64 = 0x9E37_79B1_85EB_CA87;
const P2: u64 = 0xC2B2_AE3D_27D4_EB4F;
const P3: u64 = 0x1656_67B1_9E37_79F9;
const P4: u64 = 0x85EB_CA77_C2B2_AE63;

#[derive(Clone, Copy)]
pub struct Fp {
    a: u64,
    b: u64,
    len: u64,
    buf: [u8; 16],
    blen: usize,
}

impl Default for Fp {
    fn default() -> Self {
        Fp { a: 0x243F_6A88_85A3_08D3, b: 0x1319_8A2E_0370_7344, len: 0, buf: [0; 16], blen: 0 }
    }
}

#[inline]
fn fmix(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    k ^= k >> 33;
    k = k.wrapping_mul(0xC4CE_B9FE_1A85_EC53);
    k ^ (k >> 33)
}

impl Fp {
    #[inline]
    fn block(&mut self, b: &[u8; 16]) {
        let (w0, w1) = b.split_at(8);
        let w0 = u64::from_le_bytes(w0.try_into().unwrap_or_default());
        let w1 = u64::from_le_bytes(w1.try_into().unwrap_or_default());
        self.a = (self.a ^ w0).wrapping_mul(P1).rotate_left(31).wrapping_mul(P2);
        self.b = (self.b ^ w1).wrapping_mul(P3).rotate_left(27).wrapping_mul(P4);
    }

    /// 吞入原始字节（流式，分多次调用与一次调用结果相同）
    pub fn update(&mut self, mut data: &[u8]) {
        self.len += data.len() as u64;
        if self.blen > 0 {
            let take = (16 - self.blen).min(data.len());
            self.buf[self.blen..self.blen + take].copy_from_slice(&data[..take]);
            self.blen += take;
            data = &data[take..];
            if self.blen < 16 {
                return;
            }
            let b = self.buf;
            self.block(&b);
            self.blen = 0;
        }
        let mut chunks = data.chunks_exact(16);
        for c in &mut chunks {
            let mut b = [0u8; 16];
            b.copy_from_slice(c);
            self.block(&b);
        }
        let r = chunks.remainder();
        self.buf[..r.len()].copy_from_slice(r);
        self.blen = r.len();
    }

    /// 分帧字段：标签、内容长度、内容
    pub fn field(&mut self, tag: &str, data: &[u8]) {
        self.update(&(tag.len() as u64).to_le_bytes());
        self.update(tag.as_bytes());
        self.update(&(data.len() as u64).to_le_bytes());
        self.update(data);
    }

    /// 32 位十六进制摘要
    pub fn hex(&self) -> String {
        let mut t = *self;
        let mut b = [0u8; 16];
        b[..t.blen].copy_from_slice(&t.buf[..t.blen]);
        b[15] = t.blen as u8;
        t.block(&b);
        let a = fmix(t.a ^ t.len ^ t.b.rotate_left(17));
        let c = fmix(t.b ^ t.len.rotate_left(32) ^ a);
        format!("{a:016x}{c:016x}")
    }
}

/// 一次性求字节串的摘要
pub fn hex_of(data: &[u8]) -> String {
    let mut f = Fp::default();
    f.update(data);
    f.hex()
}
