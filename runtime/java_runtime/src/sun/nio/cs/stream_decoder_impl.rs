//! `sun/nio/cs/StreamDecoder` 手写伴生：POSIX 原生族档 A（切入序 6）。
//!
//! readAllLines 的解码层。档 A 落差（如实记录）：JDK 经 CharsetDecoder 的
//! decodeLoop 机器（ByteBuffer→CharBuffer + CoderResult 协议）解码；本层以
//! 直连解码器承载——按 cs 名分族（UTF-8 严格增量解码 / ISO-8859-1 / US-ASCII），
//! 字节状态保存在 bb 的 [position, limit) 区间（JDK 同款），跨 read 的残留
//! 序列与 read0 的代理对拆分（leftoverChar）语义与 JDK 一致。错码以
//! IOException 报文呈现（MalformedInputException 类型面留档 B）。socket
//! 通道形态（ch 分支）不在语料面。

use crate::prelude::*;
use super::stream_decoder::StreamDecoder;
use crate::java::io::InputStream;
use crate::java::io::IOException;
use crate::java::lang::String;
use crate::java::nio::ByteBuffer;
use crate::java::nio::charset::Charset;
use crate::java::nio::charset::CharsetDecoder;

/// `ByteBuffer.allocate(8192) + flip()` 的初始态（hb 就绪、区间空）。
fn empty_heap_bb() -> ByteBuffer {
    let mut bb = ByteBuffer::default();
    bb._init_not_null();
    bb.__set_hb(JArray::from(vec![0i8; 8192]));
    bb.__set_offset(0);
    bb.__set_position(0);
    bb.__set_limit(0);
    bb.__set_capacity(8192);
    bb.into()
}

/// bb 的未消费字节快照（[position, limit)）。
fn bb_take(bb: &ByteBuffer) -> Vec<u8> {
    let pos = bb.__get_position();
    let lim = bb.__get_limit();
    let hb = bb.__get_hb();
    (pos..lim).map(|i| hb.get(i).unwrap_or(0) as u8).collect()
}

/// 未消费字节回存（[0, n) + position=0/limit=n——compact+读入的同构终态）。
fn bb_put(bb: &ByteBuffer, pend: &[u8]) {
    let hb = bb.__get_hb();
    for (i, b) in pend.iter().enumerate() {
        let _ = hb.set(i as i32, *b as i8);
    }
    bb.__set_position(0);
    bb.__set_limit(pend.len() as i32);
}

/// 解码族：按 charset 名分派（语料消费面 UTF-8 为主）。
enum Family {
    Utf8,
    Latin1,
    Ascii,
}

fn family_of(cs: &Charset) -> Result<Family> {
    let name = format!("{}", cs.__get_name());
    match name.as_str() {
        "UTF-8" | "UTF8" | "unicode-1-1-utf-8" => Ok(Family::Utf8),
        "ISO-8859-1" | "latin1" | "ISO8859-1" => Ok(Family::Latin1),
        "US-ASCII" | "ASCII" => Ok(Family::Ascii),
        _ => Err(JvmError::from(IOException::new_str(String::from(format!(
            "stub: sun/nio/cs/StreamDecoder (charset {} 未消费)",
            name
        )))?)),
    }
}

/// 增量解码一步：从 pend 尽可能多地解码字符进 out（容量 out_cap）。
/// 返回是否遇到不完整序列尾部（需要补字节）。
fn decode_step(fam: &Family, pend: &mut Vec<u8>, out: &mut Vec<u16>, out_cap: usize) -> Result<bool> {
    match fam {
        Family::Latin1 | Family::Ascii => {
            while out.len() < out_cap && !pend.is_empty() {
                let b = pend[0];
                if matches!(fam, Family::Ascii) && b > 0x7F {
                    return Err(JvmError::from(IOException::new_str(String::from(
                        "Malformed input: byte > 0x7F in US-ASCII",
                    ))?));
                }
                out.push(b as u16);
                pend.remove(0);
            }
            Ok(false) // 单字节族无跨读残留
        }
        Family::Utf8 => {
            loop {
                if out.len() >= out_cap || pend.is_empty() {
                    return Ok(false);
                }
                let b0 = pend[0];
                let need = match b0 {
                    0x00..=0x7F => 1,
                    0xC2..=0xDF => 2,
                    0xE0..=0xEF => 3,
                    0xF0..=0xF4 => 4,
                    _ => {
                        return Err(JvmError::from(IOException::new_str(String::from(
                            "Malformed input: invalid UTF-8 leading byte",
                        ))?))
                    }
                };
                if pend.len() < need {
                    return Ok(true); // 不完整序列：补字节
                }
                let mut scalar: u32 = match need {
                    1 => (b0 as u32) & 0x7F,
                    2 => ((b0 as u32) & 0x1F) << 6,
                    3 => ((b0 as u32) & 0x0F) << 12,
                    _ => ((b0 as u32) & 0x07) << 18,
                };
                for k in 1..need {
                    let b = pend[k];
                    if b & 0xC0 != 0x80 {
                        return Err(JvmError::from(IOException::new_str(String::from(
                            "Malformed input: invalid UTF-8 continuation byte",
                        ))?));
                    }
                    scalar |= ((b as u32) & 0x3F) << (6 * (need - 1 - k));
                }
                // 过长编码 / 代理区 / 超平面拒绝（UTF_8$Decoder 的严格面）
                let valid = match need {
                    1 => true,
                    2 => (0x80..0x800).contains(&scalar),
                    3 => (0x800..0x10000).contains(&scalar) && !(0xD800..0xE000).contains(&scalar),
                    _ => (0x10000..=0x10FFFF).contains(&scalar),
                };
                if !valid {
                    return Err(JvmError::from(IOException::new_str(String::from(
                        "Malformed input: malformed UTF-8 sequence",
                    ))?));
                }
                let c = char::from_u32(scalar).unwrap_or('\u{FFFD}');
                let mut buf = [0u16; 2];
                out.extend_from_slice(c.encode_utf16(&mut buf));
                pend.drain(..need);
            }
        }
    }
}

impl StreamDecoder {
    /// `forInputStreamReader(InputStream, Object, Charset)`。
    #[jvm_boundary]
    pub fn forInputStreamReader_inputstream_obj_charset(
        in_: InputStream,
        lock: Object,
        cs: Charset,
    ) -> Result<StreamDecoder> {
        let mut sd = StreamDecoder::default();
        sd._init_not_null();
        sd.__set_in_(in_);
        sd.__set_lock(lock);
        sd.__set_cs(cs);
        sd.__set_bb(empty_heap_bb());
        Ok(sd)
    }

    /// `forInputStreamReader(InputStream, Object, CharsetDecoder)`：
    /// cs 取 dec.charset()（构造时快照，JDK 同源）。
    #[jvm_boundary]
    pub fn forInputStreamReader_inputstream_obj_charsetdecoder(
        in_: InputStream,
        lock: Object,
        dec: CharsetDecoder,
    ) -> Result<StreamDecoder> {
        let mut sd = StreamDecoder::default();
        sd._init_not_null();
        sd.__set_in_(in_);
        sd.__set_lock(lock);
        sd.__set_decoder(dec);
        let cs = sd.__get_decoder().__get_charset();
        sd.__set_cs(cs);
        sd.__set_bb(empty_heap_bb());
        Ok(sd)
    }

    /// `read()`：read0 语义——leftover 优先；否则 2 char 读 + 拆分。
    #[jvm_boundary]
    pub fn __impl_read(&self) -> Result<i32> {
        if self.__get_haveLeftoverChar() {
            self.__set_haveLeftoverChar(false);
            return Ok(self.__get_leftoverChar() as i32);
        }
        let cb = JArray::<u16>::try_new(2)?;
        let n = self.read_arr_c_i_i(Clone::clone(&cb), 0, 2)?;
        match n {
            -1 => Ok(-1),
            2 => {
                self.__set_leftoverChar(cb.get(1)?);
                self.__set_haveLeftoverChar(true);
                Ok(cb.get(0)? as i32)
            }
            1 => Ok(cb.get(0)? as i32),
            _ => Ok(-1),
        }
    }

    /// `read(char[], int, int)`：lockedRead 语义——leftover 回填、len==1 走
    /// read0、其余 implRead（≥2 char，代理对安全）。
    #[jvm_boundary]
    pub fn __impl_read_arr_c_i_i(
        &self,
        cbuf: JArray<u16>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        if self.__get_closed() {
            return Err(JvmError::from(IOException::new_str(String::from(
                "Stream closed",
            ))?));
        }
        if offset < 0 || length < 0 || offset + length > cbuf.len()? {
            return Err(JvmError::from(
                crate::java::lang::IndexOutOfBoundsException::new()?,
            ));
        }
        if length == 0 {
            return Ok(0);
        }
        let mut off = offset;
        let mut len = length;
        let mut n = 0;
        if self.__get_haveLeftoverChar() {
            cbuf.set(off, self.__get_leftoverChar())?;
            off += 1;
            len -= 1;
            self.__set_haveLeftoverChar(false);
            n = 1;
            if len == 0 || !self.ready()? {
                return Ok(n);
            }
        }
        if len == 1 {
            let c = self.read()?;
            if c == -1 {
                return Ok(if n == 0 { -1 } else { n });
            }
            cbuf.set(off, c as u16)?;
            return Ok(n + 1);
        }
        let nr = self.impl_read(&cbuf, off, off + len)?;
        if nr < 0 {
            Ok(if n == 1 { 1 } else { nr })
        } else {
            Ok(n + nr)
        }
    }

    /// `implRead(char[], int, int)`（包私有核心）：解码 ≥1 字符；EOF 且无产出
    /// 返回 -1；不完整序列 EOF 视为错码（REPORT 语义）。
    fn impl_read(&self, cbuf: &JArray<u16>, off: i32, end: i32) -> Result<i32> {
        let fam = family_of(&self.__get_cs())?;
        let cap = (end - off) as usize;
        let mut pend = bb_take(&self.__get_bb());
        let mut out: Vec<u16> = Vec::with_capacity(cap + 1);
        let mut eof = false;
        loop {
            let _incomplete = decode_step(&fam, &mut pend, &mut out, cap)?;
            if out.len() >= cap {
                break;
            }
            if eof {
                if !pend.is_empty() {
                    return Err(JvmError::from(IOException::new_str(String::from(
                        "Malformed input: truncated UTF-8 sequence at EOF",
                    ))?));
                }
                break;
            }
            // 补字节（readBytes 语义：in.read(hb, 0, 8192)）
            let hb = self.__get_bb().__get_hb();
            let n = self.__get_in_().read_arr_b_i_i(Clone::clone(&hb), 0, 8192)?;
            if n < 0 {
                eof = true;
                continue;
            }
            if n == 0 {
                return Err(JvmError::from(IOException::new_str(String::from(
                    "Underlying input stream returned zero bytes",
                ))?));
            }
            pend.extend((0..n).map(|i| hb.get(i).unwrap_or(0) as u8));
        }
        bb_put(&self.__get_bb(), &pend);
        for (i, c) in out.iter().enumerate() {
            cbuf.set(off + i as i32, *c)?;
        }
        if out.is_empty() {
            if eof {
                return Ok(-1);
            }
            return Ok(-1);
        }
        Ok(out.len() as i32)
    }

    /// `ready()`：leftover 或 bb 有未消费字节（档 A 不触 available()——
    /// 語料面 InputStream.available 为存根；BufferedReader 阻塞读不依赖）。
    #[jvm_boundary]
    pub fn __impl_ready(&self) -> Result<bool> {
        if self.__get_closed() {
            return Err(JvmError::from(IOException::new_str(String::from(
                "Stream closed",
            ))?));
        }
        Ok(self.__get_haveLeftoverChar()
            || self.__get_bb().__get_position() < self.__get_bb().__get_limit())
    }

    /// `close()`：幂等；in.close()（ChannelInputStream → FileChannelImpl）。
    #[jvm_boundary]
    pub fn __impl_close(&self) -> Result<()> {
        if self.__get_closed() {
            return Ok(());
        }
        self.__set_closed(true);
        self.__get_in_().close()
    }
}
