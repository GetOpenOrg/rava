//! `sun/nio/ch/ChannelInputStream` 手写伴生：POSIX 原生族档 A（切入序 6）。
//! 读通道流外壳：read 委托通道的 ByteBuffer 读写（对 FileChannelImpl 精确
//! try_cast 后直调——档 A 语料面的唯一通道形态；JDK 经 ReadableByteChannel
//! 接口分派，语义等价，socket 通道族留档 B）。ByteBuffer 以堆形态构造
//! （hb/offset/position/limit 平铺字段直设，等价 ByteBuffer.wrap(bs, off, len)）。

use crate::prelude::*;
use super::channel_input_stream::ChannelInputStream;
use super::file_channel_impl::FileChannelImpl;
use crate::java::nio::ByteBuffer;

/// 堆 ByteBuffer 构造（`ByteBuffer.wrap(bs, off, len)` 等价）——通道流与
/// FileChannelImpl 共用的内部工具。
pub(crate) fn heap_bb(bs: &JArray<i8>, off: i32, len: i32) -> Result<ByteBuffer> {
    // 堆形态以基类字段直设承载（HeapByteBuffer 具体类不必入闭包——档 A 的
    // ByteBuffer 只经字段直访消费）
    let mut bb = ByteBuffer::default();
    bb._init_not_null();
    bb.__set_hb(Clone::clone(bs));
    bb.__set_offset(0);
    bb.__set_position(off);
    bb.__set_limit(off + len);
    bb.__set_capacity(bs.len()?);
    Ok(bb.into())
}

/// 通道引用 → FileChannelImpl 视图（档 A 唯一通道形态）。
pub(crate) fn as_file_channel(ch: &Object) -> Result<FileChannelImpl> {
    Clone::clone(ch).try_cast::<FileChannelImpl>("sun/nio/ch/FileChannelImpl")
}

/// 索引三元组边界检查（`Objects.checkFromIndexSize` 语义）。
pub(crate) fn check_from_index_size(off: i32, len: i32, size: i32) -> Result<()> {
    if off < 0 || len < 0 || (off as i64 + len as i64) > size as i64 {
        return Err(JvmError::from(
            crate::java::lang::IndexOutOfBoundsException::new()?,
        ));
    }
    Ok(())
}

impl ChannelInputStream {
    /// `<init>(ReadableByteChannel)`。
    #[jvm_boundary]
    pub fn new(ch: Object) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_ch(ch);
        Ok(this)
    }

    /// `read(byte[], int, int)`：单次通道读（ByteBuffer.wrap + ch.read 的
    /// 精确直调形态）；返回 -1 表示 EOF。
    #[jvm_boundary(upcalls = "java/lang/IndexOutOfBoundsException.<init>:()V")]
    pub fn __impl_read_arr_b_i_i(&self, bs: JArray<i8>, off: i32, len: i32) -> Result<i32> {
        check_from_index_size(off, len, bs.len()?)?;
        if len == 0 {
            return Ok(0);
        }
        let fc = as_file_channel(&self.__get_ch())?;
        let bb = heap_bb(&bs, off, len)?;
        fc.__impl_read_bytebuffer(bb)
    }

    /// `read()`：单字节读（JDK：b1 缓冲 + read(b1)，EOF → -1）。
    #[jvm_boundary]
    pub fn __impl_read(&self) -> Result<i32> {
        let b1 = JArray::from(vec![0i8; 1]);
        let n = self.read_arr_b_i_i(Clone::clone(&b1), 0, 1)?;
        if n == 1 {
            Ok((b1.get(0)? as u8) as i32)
        } else {
            Ok(-1)
        }
    }

    /// `close()`：ch.close()（AbstractInterruptibleChannel.close 的翻译体：
    /// 置 closed + implCloseChannel）。
    #[jvm_boundary]
    pub fn __impl_close(&self) -> Result<()> {
        let fc = as_file_channel(&self.__get_ch())?;
        fc.close()
    }
}
