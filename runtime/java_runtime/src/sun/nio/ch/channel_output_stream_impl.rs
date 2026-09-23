//! `sun/nio/ch/ChannelOutputStream` 手写伴生：POSIX 原生族档 A（切入序 6）。
//! 写通道流外壳：write 的 writeFully 循环（JDK 同款：部分写循环直至写满）。
//! 通道访问与 ByteBuffer 构造的工具见 channel_input_stream_impl。

use crate::prelude::*;
use super::channel_output_stream::ChannelOutputStream;
use super::channel_input_stream_impl::{as_file_channel, check_from_index_size, heap_bb};
use crate::java::lang::String;

impl ChannelOutputStream {
    /// `<init>(WritableByteChannel)`。
    #[jvm_boundary]
    pub fn new(ch: Object) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_ch(ch);
        Ok(this)
    }

    /// `write(byte[], int, int)`：writeFully 循环（n <= 0 视为异常，JDK 同款）。
    #[jvm_boundary(upcalls = "java/lang/IndexOutOfBoundsException.<init>:()V java/lang/RuntimeException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_write_arr_b_i_i(&self, bs: JArray<i8>, off: i32, len: i32) -> Result<()> {
        check_from_index_size(off, len, bs.len()?)?;
        if len == 0 {
            return Ok(());
        }
        let fc = as_file_channel(&self.__get_ch())?;
        let bb = heap_bb(&bs, off, len)?;
        while bb.__get_position() < bb.__get_limit() {
            let n = fc.__impl_write_bytebuffer(Clone::clone(&bb))?;
            if n <= 0 {
                return Err(JvmError::from(
                    crate::java::lang::RuntimeException::new_str(String::from(
                        "no bytes written",
                    ))?),
                );
            }
        }
        Ok(())
    }

    /// `close()`：ch.close()。
    #[jvm_boundary]
    pub fn __impl_close(&self) -> Result<()> {
        let fc = as_file_channel(&self.__get_ch())?;
        fc.close()
    }
}
