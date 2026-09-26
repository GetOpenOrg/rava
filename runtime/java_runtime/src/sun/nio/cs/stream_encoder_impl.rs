use crate::prelude::*;
use super::stream_encoder::StreamEncoder;
use crate::java::io::OutputStream;
use crate::java::nio::charset::Charset;

/// 内部边界类 sun.nio.cs.StreamEncoder：UTF-16 → UTF-8 编码后写入下游 OutputStream。
/// 编码结果不在本层缓冲（直接写下游），因此 flushBuffer 无待写数据。
impl StreamEncoder {
    #[jvm_boundary]
    pub fn forOutputStreamWriter_outputstream_obj_charset(out: OutputStream, _lock: Object, cs: Charset) -> Result<StreamEncoder> {
        let mut se = StreamEncoder::default();
        se._init_not_null();
        se.__set_out(out);
        se.__set_cs(cs);
        Ok(se)
    }

    #[jvm_boundary(upcalls = "java/io/OutputStream.write:([BII)V")]
    pub fn write_i(&self, c: i32) -> Result<()> {
        self.encode_units(&[c as u16])
    }

    #[jvm_boundary(upcalls = "java/io/OutputStream.write:([BII)V")]
    pub fn write_arr_c_i_i(&self, cbuf: JArray<u16>, off: i32, len: i32) -> Result<()> {
        let mut units: Vec<u16> = Vec::with_capacity(len.max(0) as usize);
        for i in off..off + len {
            units.push(cbuf.get(i)?);
        }
        self.encode_units(&units)
    }

    #[jvm_boundary(upcalls = "java/io/OutputStream.write:([BII)V java/lang/String.charAt:(I)C")]
    pub fn write_str_i_i(&self, s: String, off: i32, len: i32) -> Result<()> {
        let mut units: Vec<u16> = Vec::with_capacity(len.max(0) as usize);
        for i in off..off + len {
            units.push(s.charAt(i)?);
        }
        self.encode_units(&units)
    }

    #[jvm_boundary]
    pub fn flushBuffer(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary(upcalls = "java/io/OutputStream.flush:()V")]
    pub fn flush(&self) -> Result<()> {
        self.__get_out().flush()
    }

    /// `close()`：JDK `implClose` 语义——残留的孤立高代理项按编码器替换字节（`?`）写出，
    /// 再关闭底层 OutputStream；重复关闭无操作（closed 位）。
    /// 消费方：`PrintStream(out, autoFlush, charsetName).close()` → BufferedWriter →
    /// OutputStreamWriter → 本类。
    #[jvm_boundary(upcalls = "java/io/OutputStream.close:()V java/io/OutputStream.write:([BII)V")]
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
        let mut bytes: Vec<i8> = Vec::with_capacity(pending.len());
        let mut utf8 = [0u8; 4];
        for decoded in char::decode_utf16(pending.iter().copied()) {
            // 无法配对的代理项按 CodingErrorAction.REPLACE 输出 '?'
            let ch = decoded.unwrap_or('?');
            bytes.extend(ch.encode_utf8(&mut utf8).as_bytes().iter().map(|b| *b as i8));
        }
        if bytes.is_empty() {
            return Ok(());
        }
        let n = bytes.len() as i32;
        self.__get_out().write_arr_b_i_i(JArray::from(bytes), 0, n)
    }
}
