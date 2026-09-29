//! `java/util/zip/Inflater` 的 native 方法（与生成的 inflater.rs 共置）：对标 JDK Inflater.c，
//! 经系统 zlib（crate::zlib，运行期 dlopen）。Buffer 族（直接内存地址）随直接缓冲区实现，保持存根。

use crate::prelude::*;
use super::inflater::Inflater;
use crate::zlib::{self, ZStream};

fn internal_error(msg: &str) -> JvmError {
    match crate::java::lang::InternalError::new_str(String::from(msg)) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

fn lib() -> Result<&'static zlib::Zlib> {
    zlib::zlib().ok_or_else(|| internal_error("zlib not available"))
}

fn copy_in(a: &JArray<i8>, off: i32, len: i32) -> Result<Vec<u8>> {
    let mut v = Vec::with_capacity(len.max(0) as usize);
    for i in off..off + len {
        v.push(a.get(i)? as u8);
    }
    Ok(v)
}

fn copy_out(a: &JArray<i8>, off: i32, data: &[u8]) -> Result<()> {
    for (i, b) in data.iter().enumerate() {
        a.set(off + i as i32, *b as i8)?;
    }
    Ok(())
}

impl Inflater {
    /// native `initIDs()`：缓存 JNI 字段 ID——无需。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native `init(boolean nowrap)`：inflateInit2（nowrap → 原始 deflate，-MAX_WBITS）。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn init(nowrap: bool) -> Result<i64> {
        let z = lib()?;
        let addr = zlib::alloc_stream();
        let bits = if nowrap { -zlib::MAX_WBITS } else { zlib::MAX_WBITS };
        // SAFETY: addr 指向刚分配的零初始化 z_stream；版本串与结构大小取自已加载的 zlib
        let ret = unsafe { (z.inflate_init2)(zlib::stream(addr), bits, z.version, std::mem::size_of::<ZStream>() as i32) };
        match ret {
            zlib::Z_OK => Ok(addr),
            zlib::Z_MEM_ERROR => { zlib::free_stream(addr); Err(JvmError::out_of_memory("")) }
            _ => {
                // SAFETY: 同上，读取 init 失败时 zlib 写入的 msg
                let msg = unsafe { (*zlib::stream(addr)).msg() }.unwrap_or_default();
                zlib::free_stream(addr);
                Err(internal_error(&msg))
            }
        }
    }

    /// native `setDictionary(long, byte[], int, int)`：inflateSetDictionary；流状态不符 →
    /// IllegalArgumentException。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn setDictionary_l_arr_b_i_i(addr: i64, b: JArray<i8>, off: i32, len: i32) -> Result<()> {
        let z = lib()?;
        let dict = copy_in(&b, off, len)?;
        // SAFETY: addr 为 init 返回的活动流
        let ret = unsafe { (z.inflate_set_dictionary)(zlib::stream(addr), dict.as_ptr(), len as u32) };
        match ret {
            zlib::Z_OK => Ok(()),
            zlib::Z_STREAM_ERROR | zlib::Z_DATA_ERROR => Err(JvmError::illegal_argument(
                // SAFETY: 读取 msg
                &unsafe { (*zlib::stream(addr)).msg() }.unwrap_or_default())),
            _ => Err(internal_error(&unsafe { (*zlib::stream(addr)).msg() }.unwrap_or_default())),
        }
    }

    /// native `inflateBytesBytes(addr, in, inOff, inLen, out, outOff, outLen)`：inflate(Z_PARTIAL_FLUSH)；
    /// 返回值编码 inputUsed | outputUsed<<31 | finished<<62 | needDict<<63（Inflater.c
    /// checkInflateStatus）。Z_NEED_DICT / Z_DATA_ERROR 回写 inputConsumed / outputConsumed；
    /// Z_DATA_ERROR 抛 DataFormatException(strm->msg)。
    #[jvm_native(upcalls = "java/util/zip/DataFormatException.<init>:(Ljava/lang/String;)V java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn inflateBytesBytes(&self, addr: i64, input: JArray<i8>, input_off: i32, input_len: i32,
                             output: JArray<i8>, output_off: i32, output_len: i32) -> Result<i64> {
        let z = lib()?;
        let inbuf = copy_in(&input, input_off, input_len)?;
        let mut outbuf = vec![0u8; output_len.max(0) as usize];
        let strm = zlib::stream(addr);
        // SAFETY: strm 为活动流；缓冲区在调用期间有效，zlib 不保留 next_in / next_out
        let ret = unsafe {
            (*strm).next_in = inbuf.as_ptr();
            (*strm).avail_in = input_len as u32;
            (*strm).next_out = outbuf.as_mut_ptr();
            (*strm).avail_out = output_len as u32;
            (z.inflate)(strm, zlib::Z_PARTIAL_FLUSH)
        };
        // SAFETY: 读取调用后的剩余量与 msg
        let (avail_in, avail_out, msg) = unsafe { ((*strm).avail_in as i32, (*strm).avail_out as i32, (*strm).msg()) };
        let (in_used, out_used) = (input_len - avail_in, output_len - avail_out);
        copy_out(&output, output_off, &outbuf[..out_used.max(0) as usize])?;
        match ret {
            zlib::Z_STREAM_END => Ok(zlib::pack(in_used, out_used, true, false)),
            zlib::Z_OK => Ok(zlib::pack(in_used, out_used, false, false)),
            zlib::Z_NEED_DICT => {
                self.__set_inputConsumed(in_used);
                Ok(zlib::pack(in_used, out_used, false, true))
            }
            zlib::Z_BUF_ERROR => Ok(0),
            zlib::Z_DATA_ERROR => {
                self.__set_inputConsumed(in_used);
                self.__set_outputConsumed(out_used);
                Err(JvmError::from(super::DataFormatException::new_str(
                    String::from(msg.unwrap_or_default().as_str()))?))
            }
            zlib::Z_MEM_ERROR => Err(JvmError::out_of_memory("")),
            _ => Err(internal_error(&msg.unwrap_or_default())),
        }
    }

    /// native `getAdler(long)`：当前 adler32 / crc32 校验值。
    #[jvm_native]
    pub fn getAdler_l(addr: i64) -> Result<i32> {
        // SAFETY: addr 为活动流
        Ok(unsafe { (*zlib::stream(addr)).adler } as i32)
    }

    /// native `reset(long)`：inflateReset。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn reset_l(addr: i64) -> Result<()> {
        let z = lib()?;
        // SAFETY: addr 为活动流
        if unsafe { (z.inflate_reset)(zlib::stream(addr)) } != zlib::Z_OK {
            return Err(internal_error(""));
        }
        Ok(())
    }

    /// native `end(long)`：inflateEnd 并释放流。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn end_l(addr: i64) -> Result<()> {
        let z = lib()?;
        // SAFETY: addr 为活动流，end 之后 Java 侧不再使用该地址
        let ret = unsafe { (z.inflate_end)(zlib::stream(addr)) };
        zlib::free_stream(addr);
        if ret == zlib::Z_STREAM_ERROR {
            return Err(internal_error(""));
        }
        Ok(())
    }
}
