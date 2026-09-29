//! `java/util/zip/Deflater` 的 native 方法（与生成的 deflater.rs 共置）：对标 JDK Deflater.c，
//! 经系统 zlib（crate::zlib，运行期 dlopen）——压缩输出与 JVM 逐字节一致。Buffer 族（直接内存
//! 地址）随直接缓冲区实现，保持存根。

use crate::prelude::*;
use super::deflater::Deflater;
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

impl Deflater {
    /// native `init(int level, int strategy, boolean nowrap)`：deflateInit2（Z_DEFLATED、
    /// DEF_MEM_LEVEL 8、nowrap → -MAX_WBITS）。参数非法 → IllegalArgumentException。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn init(level: i32, strategy: i32, nowrap: bool) -> Result<i64> {
        let z = lib()?;
        let addr = zlib::alloc_stream();
        let bits = if nowrap { -zlib::MAX_WBITS } else { zlib::MAX_WBITS };
        // SAFETY: addr 指向刚分配的零初始化 z_stream
        let ret = unsafe {
            (z.deflate_init2)(zlib::stream(addr), level, zlib::Z_DEFLATED, bits, zlib::DEF_MEM_LEVEL,
                              strategy, z.version, std::mem::size_of::<ZStream>() as i32)
        };
        match ret {
            zlib::Z_OK => Ok(addr),
            zlib::Z_MEM_ERROR => { zlib::free_stream(addr); Err(JvmError::out_of_memory("")) }
            zlib::Z_STREAM_ERROR => { zlib::free_stream(addr); Err(JvmError::illegal_argument("")) }
            _ => {
                // SAFETY: 读取 init 失败时的 msg
                let msg = unsafe { (*zlib::stream(addr)).msg() }.unwrap_or_default();
                zlib::free_stream(addr);
                Err(internal_error(&msg))
            }
        }
    }

    /// native `setDictionary(long, byte[], int, int)`：deflateSetDictionary。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn setDictionary_l_arr_b_i_i(addr: i64, b: JArray<i8>, off: i32, len: i32) -> Result<()> {
        let z = lib()?;
        let dict = copy_in(&b, off, len)?;
        // SAFETY: addr 为活动流
        let ret = unsafe { (z.deflate_set_dictionary)(zlib::stream(addr), dict.as_ptr(), len as u32) };
        match ret {
            zlib::Z_OK => Ok(()),
            zlib::Z_STREAM_ERROR => Err(JvmError::illegal_argument("")),
            _ => Err(internal_error(&unsafe { (*zlib::stream(addr)).msg() }.unwrap_or_default())),
        }
    }

    /// native `deflateBytesBytes(addr, in, inOff, inLen, out, outOff, outLen, flush, params)`：
    /// params 低位为 1 时先 deflateParams(level = params>>3, strategy = (params>>1)&3)，否则
    /// deflate(flush)。返回 inputUsed | outputUsed<<31 | finished<<62 | setParams<<63
    /// （Deflater.c checkDeflateStatus；deflateParams 返回 Z_OK 即清 setParams 位）。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn deflateBytesBytes(&self, addr: i64, input: JArray<i8>, input_off: i32, input_len: i32,
                             output: JArray<i8>, output_off: i32, output_len: i32,
                             flush: i32, params: i32) -> Result<i64> {
        let z = lib()?;
        let inbuf = copy_in(&input, input_off, input_len)?;
        let mut outbuf = vec![0u8; output_len.max(0) as usize];
        let strm = zlib::stream(addr);
        let set_params = params & 1 != 0;
        // SAFETY: strm 为活动流；缓冲区在调用期间有效
        let ret = unsafe {
            (*strm).next_in = inbuf.as_ptr();
            (*strm).avail_in = input_len as u32;
            (*strm).next_out = outbuf.as_mut_ptr();
            (*strm).avail_out = output_len as u32;
            if set_params {
                (z.deflate_params)(strm, params >> 3, (params >> 1) & 3)
            } else {
                (z.deflate)(strm, flush)
            }
        };
        // SAFETY: 读取调用后的剩余量与 msg
        let (avail_in, avail_out, msg) = unsafe { ((*strm).avail_in as i32, (*strm).avail_out as i32, (*strm).msg()) };
        let (in_used, out_used) = (input_len - avail_in, output_len - avail_out);
        for (i, b) in outbuf[..out_used.max(0) as usize].iter().enumerate() {
            output.set(output_off + i as i32, *b as i8)?;
        }
        if set_params {
            return match ret {
                zlib::Z_OK => Ok(zlib::pack(in_used, out_used, false, false)),
                zlib::Z_BUF_ERROR => Ok(zlib::pack(in_used, out_used, false, true)),
                _ => Err(internal_error("deflateParams failed")),
            };
        }
        match ret {
            zlib::Z_STREAM_END => Ok(zlib::pack(in_used, out_used, true, false)),
            zlib::Z_OK => Ok(zlib::pack(in_used, out_used, false, false)),
            zlib::Z_BUF_ERROR => Ok(0),
            _ => Err(internal_error(&msg.unwrap_or_default())),
        }
    }

    /// native `getAdler(long)`：当前 adler32 / crc32 校验值。
    #[jvm_native]
    pub fn getAdler_l(addr: i64) -> Result<i32> {
        // SAFETY: addr 为活动流
        Ok(unsafe { (*zlib::stream(addr)).adler } as i32)
    }

    /// native `reset(long)`：deflateReset。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn reset_l(addr: i64) -> Result<()> {
        let z = lib()?;
        // SAFETY: addr 为活动流
        if unsafe { (z.deflate_reset)(zlib::stream(addr)) } != zlib::Z_OK {
            return Err(internal_error(""));
        }
        Ok(())
    }

    /// native `end(long)`：deflateEnd 并释放流。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn end_l(addr: i64) -> Result<()> {
        let z = lib()?;
        // SAFETY: addr 为活动流，end 之后 Java 侧不再使用该地址
        let ret = unsafe { (z.deflate_end)(zlib::stream(addr)) };
        zlib::free_stream(addr);
        if ret == zlib::Z_STREAM_ERROR {
            return Err(internal_error(""));
        }
        Ok(())
    }
}
