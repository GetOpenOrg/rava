//! 大端字节读取器（JVMS §4 的 u1/u2/u4 等）。

use crate::Error;

pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Reader { data, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    fn need(&self, n: usize) -> Result<(), Error> {
        if self.pos + n > self.data.len() {
            return Err(Error::Truncated(self.pos));
        }
        Ok(())
    }

    pub fn u1(&mut self) -> Result<u8, Error> {
        self.need(1)?;
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }

    pub fn u2(&mut self) -> Result<u16, Error> {
        self.need(2)?;
        let v = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(v)
    }

    pub fn u4(&mut self) -> Result<u32, Error> {
        self.need(4)?;
        let b = &self.data[self.pos..self.pos + 4];
        self.pos += 4;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub fn i1(&mut self) -> Result<i8, Error> {
        Ok(self.u1()? as i8)
    }

    pub fn i2(&mut self) -> Result<i16, Error> {
        Ok(self.u2()? as i16)
    }

    pub fn i4(&mut self) -> Result<i32, Error> {
        Ok(self.u4()? as i32)
    }

    pub fn u8_(&mut self) -> Result<u64, Error> {
        let hi = self.u4()? as u64;
        let lo = self.u4()? as u64;
        Ok((hi << 32) | lo)
    }

    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8], Error> {
        self.need(n)?;
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    pub fn skip(&mut self, n: usize) -> Result<(), Error> {
        self.need(n)?;
        self.pos += n;
        Ok(())
    }
}

/// Modified UTF-8（JVMS §4.4.7）解码为 UTF-16 码元：`C0 80` 为 NUL，补充平面字符以代理对编码，
/// 孤立代理项原样保留。残缺字节序列替换为 U+FFFD（同 Python 侧）。
pub fn decode_mutf8_units(b: &[u8]) -> Vec<u16> {
    let mut units: Vec<u16> = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c & 0x80 == 0 {
            units.push(c as u16);
            i += 1;
        } else if c & 0xE0 == 0xC0 && i + 1 < b.len() && b[i + 1] & 0xC0 == 0x80 {
            units.push((((c & 0x1F) as u16) << 6) | (b[i + 1] & 0x3F) as u16);
            i += 2;
        } else if c & 0xF0 == 0xE0 && i + 2 < b.len() && b[i + 1] & 0xC0 == 0x80 && b[i + 2] & 0xC0 == 0x80 {
            units.push(
                (((c & 0x0F) as u16) << 12) | (((b[i + 1] & 0x3F) as u16) << 6) | (b[i + 2] & 0x3F) as u16,
            );
            i += 3;
        } else {
            units.push(0xFFFD);
            i += 1;
        }
    }
    units
}

/// Modified UTF-8 解码为 Rust String 及（仅当含孤立代理项时）无损的 UTF-16 码元。
/// 孤立代理项放不进 Rust String，文本侧替换为 U+FFFD——成员名 / 类名匹配只用文本
/// （孤立代理不构成合法名字）；字符串常量（ldc / ConstantValue / 拼接配方）取码元，保值。
pub fn decode_mutf8(b: &[u8]) -> (String, Option<Box<[u16]>>) {
    let units = decode_mutf8_units(b);
    match String::from_utf16(&units) {
        Ok(s) => (s, None),
        Err(_) => (String::from_utf16_lossy(&units), Some(units.into_boxed_slice())),
    }
}

#[cfg(test)]
mod mutf8_tests {
    use super::*;

    #[test]
    fn lone_surrogate_keeps_units() {
        // "a" + U+D83D（孤立高代理，三字节形态 ED A0 BD）
        let (s, units) = decode_mutf8(&[0x61, 0xED, 0xA0, 0xBD]);
        assert_eq!(s, "a\u{FFFD}");
        assert_eq!(units.as_deref(), Some(&[0x61u16, 0xD83D][..]));
        // 成对代理（六字节形态）无损，不携带码元
        let (s, units) = decode_mutf8(&[0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80]);
        assert_eq!(s, "\u{1F600}");
        assert!(units.is_none());
        // NUL 两字节形态
        assert_eq!(decode_mutf8(&[0xC0, 0x80]).0, "\0");
    }
}
