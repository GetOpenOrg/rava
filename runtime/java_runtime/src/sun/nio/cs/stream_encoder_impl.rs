use crate::prelude::*;
use super::stream_encoder::StreamEncoder;
use crate::java::io::OutputStream;
use crate::java::nio::charset::Charset;

/// `encodingName()`：HistoricallyNamedCharset 的历史名（九个标准字符集中 UTF-16 / UTF-32 族
/// 历史名即规范名），其余取规范名——JVM 实测对照。
fn historical_name(cs: &Charset) -> String {
    let name = format!("{}", cs.__get_name());
    String::from(match name.as_str() {
        "UTF-8" => "UTF8",
        "ISO-8859-1" => "ISO8859_1",
        "US-ASCII" => "ASCII",
        "UTF-16BE" => "UnicodeBigUnmarked",
        "UTF-16LE" => "UnicodeLittleUnmarked",
        other => other,
    })
}

/// `Charset.forName(name)`；IllegalCharsetNameException / UnsupportedCharsetException →
/// `UnsupportedEncodingException(name)`（按字符集名构造的前置，JDK 同形）。
fn charset_for_name(name: String) -> Result<Charset> {
    match Charset::forName_str(Clone::clone(&name)) {
        Err(e) if e.is_instance_of("java/nio/charset/IllegalCharsetNameException")
            || e.is_instance_of("java/nio/charset/UnsupportedCharsetException") =>
        {
            Err(JvmError::from(crate::java::io::UnsupportedEncodingException::new_str(name)?))
        }
        r => r,
    }
}

/// 内部边界类 sun.nio.cs.StreamEncoder：UTF-16 码元按构造时的 charset 编码后写入下游
/// OutputStream（九个标准 charset，名字取自各 charset 构造器登记的规范名——与
/// StandardCharsets 边界同一真源）。编码结果不在本层缓冲（直接写下游），flushBuffer 无待写数据。
///
/// 编码规则与 JDK 编码器一致：8 位编码（ISO-8859-1 / US-ASCII）不可映射字符 → `?`（代理对
/// 按一个字符计）；`UTF-16` 首次写出 BOM（FE FF）后按大端，`UTF-32` 大端无 BOM；孤立代理项
/// 按 CodingErrorAction.REPLACE（8 位 / UTF-8 → `?`，UTF-16/32 → U+FFFD）。
impl StreamEncoder {
    #[jvm_boundary]
    pub fn forOutputStreamWriter_outputstream_obj_charset(out: OutputStream, _lock: Object, cs: Charset) -> Result<StreamEncoder> {
        let mut se = StreamEncoder::default();
        se._init_not_null();
        se.__set_out(out);
        se.__set_cs(cs);
        Ok(se)
    }

    /// `getEncoding()`：打开时返回字符集历史名，关闭后 null（JDK：`isOpen() ? encodingName() : null`）。
    #[jvm_boundary]
    pub fn getEncoding(&self) -> Result<String> {
        if self.__get_closed() {
            return Ok(String::default());
        }
        Ok(historical_name(&self.__get_cs()))
    }

    /// `forOutputStreamWriter(OutputStream, Object, String)`：按字符集名查找，名字非法 / 不支持 →
    /// `UnsupportedEncodingException(charsetName)`（JDK 同形）。
    #[jvm_boundary]
    pub fn forOutputStreamWriter_outputstream_obj_str(out: OutputStream, lock: Object, charset_name: String) -> Result<StreamEncoder> {
        let cs = charset_for_name(charset_name)?;
        Self::forOutputStreamWriter_outputstream_obj_charset(out, lock, cs)
    }

    #[jvm_boundary]
    pub fn write_i(&self, c: i32) -> Result<()> {
        self.encode_units(&[c as u16])
    }

    #[jvm_boundary]
    pub fn write_arr_c_i_i(&self, cbuf: JArray<u16>, off: i32, len: i32) -> Result<()> {
        let mut units: Vec<u16> = Vec::with_capacity(len.max(0) as usize);
        for i in off..off.saturating_add(len) {
            units.push(cbuf.get(i)?);
        }
        self.encode_units(&units)
    }

    #[jvm_boundary]
    pub fn write_str_i_i(&self, s: String, off: i32, len: i32) -> Result<()> {
        let mut units: Vec<u16> = Vec::with_capacity(len.max(0) as usize);
        for i in off..off.saturating_add(len) {
            units.push(s.charAt(i)?);
        }
        self.encode_units(&units)
    }

    #[jvm_boundary]
    pub fn flushBuffer(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn flush(&self) -> Result<()> {
        self.__get_out().flush()
    }

    /// `close()`：JDK `implClose` 语义——残留的孤立高代理项按编码器替换字节（`?`）写出，
    /// 再关闭底层 OutputStream；重复关闭无操作（closed 位）。
    /// 消费方：`PrintStream(out, autoFlush, charsetName).close()` → BufferedWriter →
    /// OutputStreamWriter → 本类。
    #[jvm_boundary]
    pub fn close(&self) -> Result<()> {
        if self.__get_closed() {
            return Ok(());
        }
        if self.__get_haveLeftoverChar() {
            self.__set_haveLeftoverChar(false);
            self.__get_out().write_arr_b_i_i(JArray::from(vec![b'?' as i8]), 0, 1)?;
        }
        self.__set_closed(true);
        self.__get_out().close()
    }
}

impl StreamEncoder {
    /// 编码一段 UTF-16 码元；段尾的孤立高代理项留到下一次写入配对（leftoverChar 语义）。
    fn encode_units(&self, units: &[u16]) -> Result<()> {
        let mut pending: Vec<u16> = Vec::with_capacity(units.len() + 1);
        if self.__get_haveLeftoverChar() {
            pending.push(self.__get_leftoverChar());
            self.__set_haveLeftoverChar(false);
        }
        pending.extend_from_slice(units);
        if let Some(&last) = pending.last() {
            if (0xD800..0xDC00).contains(&last) {
                pending.pop();
                self.__set_leftoverChar(last);
                self.__set_haveLeftoverChar(true);
            }
        }
        let cs_name = format!("{}", self.__get_cs().__get_name());
        let mut out: Vec<u8> = Vec::with_capacity(pending.len() * 2);
        if cs_name == "UTF-16" && !pending.is_empty() && self.take_bom() {
            out.extend_from_slice(&[0xFE, 0xFF]);
        }
        for decoded in char::decode_utf16(pending.iter().copied()) {
            encode_char(&cs_name, decoded.ok(), &mut out);
        }
        let bytes: Vec<i8> = out.into_iter().map(|b| b as i8).collect();
        if bytes.is_empty() {
            return Ok(());
        }
        let n = bytes.len() as i32;
        self.__get_out().write_arr_b_i_i(JArray::from(bytes), 0, n)
    }
}

impl StreamEncoder {
    /// UTF-16 编码器的 BOM 只在流首写出一次（JDK UnicodeEncoder 的 needsMark 状态）：
    /// 首次调用返回 true 并登记本编码器身份。
    fn take_bom(&self) -> bool {
        crate::__process_static! {
            static BOM_WRITTEN: crate::sync_model::__RefSlot<std::collections::HashSet<usize>> =
                crate::sync_model::__RefSlot::new(std::collections::HashSet::new());
        }
        let id = Object::from(Clone::clone(self)).0.__identity() as usize;
        BOM_WRITTEN.with(|s| s.borrow_mut().insert(id))
    }
}

/// 单个字符（None = 孤立代理项）按 charset 规范名编码追加到 `out`。
fn encode_char(cs: &str, ch: Option<char>, out: &mut Vec<u8>) {
    match cs {
        "ISO-8859-1" | "US-ASCII" => {
            let limit = if cs == "US-ASCII" { 0x80 } else { 0x100 };
            match ch {
                Some(c) if (c as u32) < limit => out.push(c as u32 as u8),
                _ => out.push(b'?'),
            }
        }
        "UTF-16" | "UTF-16BE" | "UTF-16LE" => {
            let c = ch.unwrap_or('\u{FFFD}');
            let mut buf = [0u16; 2];
            for unit in c.encode_utf16(&mut buf).iter() {
                if cs == "UTF-16LE" {
                    out.extend_from_slice(&unit.to_le_bytes());
                } else {
                    out.extend_from_slice(&unit.to_be_bytes());
                }
            }
        }
        "UTF-32" | "UTF-32BE" | "UTF-32LE" => {
            let cp = ch.unwrap_or('\u{FFFD}') as u32;
            if cs == "UTF-32LE" {
                out.extend_from_slice(&cp.to_le_bytes());
            } else {
                out.extend_from_slice(&cp.to_be_bytes());
            }
        }
        _ => {
            // UTF-8（缺省 charset）
            let c = ch.unwrap_or('?');
            let mut utf8 = [0u8; 4];
            out.extend_from_slice(c.encode_utf8(&mut utf8).as_bytes());
        }
    }
}
