//! `java/util/zip/Inflater` 的 native 方法（与生成的 inflater.rs 共置）：对标 JDK Inflater.c，
//! 经系统 zlib（crate::zlib，运行期 dlopen）。Bytes / Buffer 各变体（Buffer = 直接缓冲区的直接内存
//! 地址）共用 inflate_raw / inflate_status。

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
    for i in off..off.saturating_add(len) {
        v.push(a.get(i)? as u8);
    }
    Ok(v)
}

fn copy_out(a: &JArray<i8>, off: i32, data: &[u8]) -> Result<()> {
    for (i, b) in data.iter().enumerate() {
        a.set(off.wrapping_add(i as i32), *b as i8)?;
    }
    Ok(())
}

/// 一次 inflate 调用的结果（原始指针形态，Bytes / Buffer 各变体共用）。
struct InflateRun {
    ret: i32,
    in_used: i32,
    out_used: i32,
    msg: Option<std::string::String>,
}

fn inflate_raw(addr: i64, input: *const u8, input_len: i32, output: *mut u8, output_len: i32) -> Result<InflateRun> {
    let z = lib()?;
    let strm = zlib::stream(addr);
    // SAFETY: strm 为活动流；input / output 在调用期间有效（堆副本或直接内存），zlib 不保留指针
    let ret = unsafe {
        (*strm).next_in = input;
        (*strm).avail_in = input_len as u32;
        (*strm).next_out = output;
        (*strm).avail_out = output_len as u32;
        (z.inflate)(strm, zlib::Z_PARTIAL_FLUSH)
    };
    // SAFETY: 读取调用后的剩余量与 msg
    let (avail_in, avail_out, msg) = unsafe { ((*strm).avail_in as i32, (*strm).avail_out as i32, (*strm).msg()) };
    Ok(InflateRun { ret, in_used: input_len - avail_in, out_used: output_len - avail_out, msg })
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
        let inbuf = copy_in(&input, input_off, input_len)?;
        let mut outbuf = vec![0u8; output_len.max(0) as usize];
        let r = inflate_raw(addr, inbuf.as_ptr(), input_len, outbuf.as_mut_ptr(), output_len)?;
        copy_out(&output, output_off, &outbuf[..r.out_used.max(0) as usize])?;
        self.inflate_status(r)
    }

    /// native `inflateBytesBuffer(addr, in, inOff, inLen, outAddr, outLen)`：输出为直接缓冲区。
    #[jvm_native(upcalls = "java/util/zip/DataFormatException.<init>:(Ljava/lang/String;)V java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn inflateBytesBuffer(&self, addr: i64, input: JArray<i8>, input_off: i32, input_len: i32,
                              output_addr: i64, output_len: i32) -> Result<i64> {
        let inbuf = copy_in(&input, input_off, input_len)?;
        let r = inflate_raw(addr, inbuf.as_ptr(), input_len, output_addr as *mut u8, output_len)?;
        self.inflate_status(r)
    }

    /// native `inflateBufferBytes(addr, inAddr, inLen, out, outOff, outLen)`：输入为直接缓冲区。
    #[jvm_native(upcalls = "java/util/zip/DataFormatException.<init>:(Ljava/lang/String;)V java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn inflateBufferBytes(&self, addr: i64, input_addr: i64, input_len: i32,
                              output: JArray<i8>, output_off: i32, output_len: i32) -> Result<i64> {
        let mut outbuf = vec![0u8; output_len.max(0) as usize];
        let r = inflate_raw(addr, input_addr as *const u8, input_len, outbuf.as_mut_ptr(), output_len)?;
        copy_out(&output, output_off, &outbuf[..r.out_used.max(0) as usize])?;
        self.inflate_status(r)
    }

    /// native `inflateBufferBuffer(addr, inAddr, inLen, outAddr, outLen)`：输入输出均为直接缓冲区。
    #[jvm_native(upcalls = "java/util/zip/DataFormatException.<init>:(Ljava/lang/String;)V java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn inflateBufferBuffer(&self, addr: i64, input_addr: i64, input_len: i32,
                               output_addr: i64, output_len: i32) -> Result<i64> {
        let r = inflate_raw(addr, input_addr as *const u8, input_len, output_addr as *mut u8, output_len)?;
        self.inflate_status(r)
    }

    /// 状态映射（Inflater.c checkInflateStatus）；Z_NEED_DICT / Z_DATA_ERROR 回写消费量字段。
    fn inflate_status(&self, r: InflateRun) -> Result<i64> {
        match r.ret {
            zlib::Z_STREAM_END => Ok(zlib::pack(r.in_used, r.out_used, true, false)),
            zlib::Z_OK => Ok(zlib::pack(r.in_used, r.out_used, false, false)),
            zlib::Z_NEED_DICT => {
                self.__set_inputConsumed(r.in_used);
                Ok(zlib::pack(r.in_used, r.out_used, false, true))
            }
            zlib::Z_BUF_ERROR => Ok(0),
            zlib::Z_DATA_ERROR => {
                self.__set_inputConsumed(r.in_used);
                self.__set_outputConsumed(r.out_used);
                Err(JvmError::from(super::DataFormatException::new_str(
                    String::from(r.msg.unwrap_or_default().as_str()))?))
            }
            zlib::Z_MEM_ERROR => Err(JvmError::out_of_memory("")),
            _ => Err(internal_error(&r.msg.unwrap_or_default())),
        }
    }

    /// native `setDictionaryBuffer(long addr, long bufAddress, int len)`：字典来自直接缓冲区。
    #[jvm_native(upcalls = "java/lang/InternalError.<init>:(Ljava/lang/String;)V")]
    pub fn setDictionaryBuffer(addr: i64, buf_addr: i64, len: i32) -> Result<()> {
        let z = lib()?;
        // SAFETY: addr 为活动流；buf_addr 为直接内存地址，len 已由 Java 侧界检查
        let ret = unsafe { (z.inflate_set_dictionary)(zlib::stream(addr), buf_addr as *const u8, len as u32) };
        match ret {
            zlib::Z_OK => Ok(()),
            zlib::Z_STREAM_ERROR | zlib::Z_DATA_ERROR => Err(JvmError::illegal_argument(
                // SAFETY: 读取 msg
                &unsafe { (*zlib::stream(addr)).msg() }.unwrap_or_default())),
            _ => Err(internal_error(&unsafe { (*zlib::stream(addr)).msg() }.unwrap_or_default())),
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
