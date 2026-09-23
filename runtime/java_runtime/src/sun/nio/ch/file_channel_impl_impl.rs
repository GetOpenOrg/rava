//! `sun/nio/ch/FileChannelImpl` 手写伴生：POSIX 原生族档 A（切入序 6）。
//!
//! 用例面（writeString/readString/readAllLines/mismatch 的数据落地）：
//! 构造 + read/write(ByteBuffer) + size/position + close。fd 为 i32 裸 fd
//! （UnixChannelFactory.open 的 open(2) 结果），读写以
//! `ManuallyDrop<File::from_raw_fd>` 承载（不取得所有权，close 时回收一次）。
//! ByteBuffer 的堆形态（wrap/allocate 产物）经平铺字段直访（hb/offset/
//! position/limit——宏生成的访问器，不依赖 java.nio 方法体的调用链收录）。
//! 分散/聚集读写、锁、mmap、direct I/O 保持 panic 存根（档 A 纪律）。

use crate::prelude::*;
use super::file_channel_impl::FileChannelImpl;
use crate::java::io::FileDescriptor;
use crate::java::io::IOException;
use crate::java::lang::String;
use crate::java::nio::ByteBuffer;

/// fd → 借用视图（不取得所有权：ManuallyDrop 包裹，离开作用域不关 fd）。
fn borrow_file(fd: i32) -> std::mem::ManuallyDrop<std::fs::File> {
    use std::os::fd::FromRawFd;
    std::mem::ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(fd) })
}

/// io::Error → IOException 的 JvmError（read/write/seek/metadata 的失败面）。
fn io_err(e: std::io::Error) -> JvmError {
    match IOException::new_str(String::from(format!("{}", e))) {
        Ok(x) => JvmError::from(x),
        Err(err) => err,
    }
}

/// 堆 ByteBuffer 的字节视图参数（hb + offset + position/limit）。
struct HeapView {
    arr: JArray<i8>,
    start: i32, // hb 内的读写起点（offset + position）
    len: i32,   // remaining
    pos: i32,   // buffer 当前 position（推进量回写用）
}

fn heap_view(bb: &ByteBuffer) -> Result<HeapView> {
    let arr = bb.__get_hb();
    let off = bb.__get_offset();
    let pos = bb.__get_position();
    let lim = bb.__get_limit();
    Ok(HeapView {
        arr,
        start: off + pos,
        len: (lim - pos).max(0),
        pos,
    })
}

impl FileChannelImpl {
    /// `<init>(FileDescriptor, String, boolean, boolean, boolean, Closeable)`：
    /// 档 A 落差：threads/positionLock（单线程协作调度无中断语义）、alignment
    /// （direct 恒 false）、closer（无 Cleaner 池——fd 由 close 显式回收）不承载。
    #[jvm_boundary]
    pub fn new(
        fd: FileDescriptor,
        path: String,
        readable: bool,
        writable: bool,
        direct: bool,
        parent: Object,
    ) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        // super()：AbstractInterruptibleChannel.<init> 的字段语义
        // （closeLock = new Object()；closed 缺省 false）
        this.__set_closeLock(Object::new()?);
        this.__set_fd(fd);
        this.__set_path(path);
        this.__set_readable(readable);
        this.__set_writable(writable);
        this.__set_direct(direct);
        this.__set_parent(parent);
        this.__set_alignment(-1);
        Ok(this)
    }

    /// `<clinit>`：JDK 侧初始化 fdAccess/nd 静态字段（native 调度层）——档 A
    /// 的读写不经过 FileDispatcher（std 直连 fd），静态初始化为 no-op。
    #[jvm_boundary]
    pub fn __clinit() -> Result<()> {
        Ok(())
    }

    /// `open(FileDescriptor, String, boolean, boolean, boolean, Closeable)`：
    /// 静态工厂（UnixChannelFactory 的落点）。
    #[jvm_boundary(upcalls = "java/nio/channels/FileChannel.<clinit>:()V")]
    pub fn open_filedescriptor_str_z_z_z_closeable(
        fd: FileDescriptor,
        path: String,
        readable: bool,
        writable: bool,
        direct: bool,
        parent: Object,
    ) -> Result<crate::java::nio::channels::FileChannel> {
        Ok(Self::new(fd, path, readable, writable, direct, parent)?.into())
    }

    /// `setUninterruptible()`：阻塞不可中断标记（无中断语义，仅置位）。
    #[jvm_boundary]
    pub fn setUninterruptible(&self) -> Result<()> {
        self.__set_uninterruptible(true);
        Ok(())
    }

    /// `read(ByteBuffer)`：单次 read(2) 到 dst 的剩余空间；EOF 归一化为 -1
    /// （IOStatus.normalize 语义）；position 前移 n。不可读通道 /
    /// 已关闭通道按 JDK 抛 NonReadableChannelException / ClosedChannelException。
    #[jvm_boundary(upcalls = "java/nio/channels/NonReadableChannelException.<init>:()V java/nio/channels/ClosedChannelException.<init>:()V")]
    pub fn __impl_read_bytebuffer(&self, dst: ByteBuffer) -> Result<i32> {
        use std::io::Read;
        if self.__get_closed() {
            return Err(JvmError::from(
                crate::java::nio::channels::ClosedChannelException::new()?,
            ));
        }
        if !self.__get_readable() {
            return Err(JvmError::from(
                crate::java::nio::channels::NonReadableChannelException::new()?,
            ));
        }
        let fd = self.__get_fd().__get_fd();
        let mut f = borrow_file(fd);
        let view = heap_view(&dst)?;
        if view.len == 0 {
            return Ok(0);
        }
        let n = view.arr.with_vec(|v| {
            let buf: &mut [u8] = unsafe {
                std::slice::from_raw_parts_mut(
                    v.as_mut_ptr().add(view.start as usize) as *mut u8,
                    view.len as usize,
                )
            };
            std::io::Read::read(&mut *f, buf).map_err(io_err)
        })??;
        match n {
            0 => Ok(-1), // EOF
            n => {
                dst.__set_position(view.pos + n as i32);
                Ok(n as i32)
            }
        }
    }

    /// `write(ByteBuffer)`：单次 write(2) 自 src 的剩余区（部分写允许——调用方
    /// writeFully 循环，JDK 同语义）；position 前移 n。
    #[jvm_boundary(upcalls = "java/nio/channels/NonWritableChannelException.<init>:()V java/nio/channels/ClosedChannelException.<init>:()V")]
    pub fn __impl_write_bytebuffer(&self, src: ByteBuffer) -> Result<i32> {
        use std::io::Write;
        if self.__get_closed() {
            return Err(JvmError::from(
                crate::java::nio::channels::ClosedChannelException::new()?,
            ));
        }
        if !self.__get_writable() {
            return Err(JvmError::from(
                crate::java::nio::channels::NonWritableChannelException::new()?,
            ));
        }
        let fd = self.__get_fd().__get_fd();
        let mut f = borrow_file(fd);
        let view = heap_view(&src)?;
        if view.len == 0 {
            return Ok(0);
        }
        let n = view.arr.with_vec(|v| {
            let buf: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    v.as_ptr().add(view.start as usize) as *const u8,
                    view.len as usize,
                )
            };
            std::io::Write::write(&mut *f, buf).map_err(io_err)
        })??;
        src.__set_position(view.pos + n as i32);
        Ok(n as i32)
    }

    /// `position()`：fd 当前偏移（lseek(cur)）。
    #[jvm_boundary]
    pub fn __impl_position(&self) -> Result<i64> {
        use std::io::Seek;
        let fd = self.__get_fd().__get_fd();
        let mut f = borrow_file(fd);
        let pos = std::io::Seek::seek(&mut *f, std::io::SeekFrom::Current(0)).map_err(io_err)?;
        Ok(pos as i64)
    }

    /// `size()`：fstat 的 st_size。
    #[jvm_boundary]
    pub fn __impl_size(&self) -> Result<i64> {
        let fd = self.__get_fd().__get_fd();
        let f = borrow_file(fd);
        let len = f.metadata().map_err(io_err)?.len();
        Ok(len as i64)
    }

    /// `implCloseChannel()`：置 closed + close(fd)（AbstractInterruptibleChannel.
    /// close 翻译体的 this 虚调落点）。
    #[jvm_boundary]
    pub fn __impl_implCloseChannel(&self) -> Result<()> {
        self.__set_closed(true);
        let fd = self.__get_fd().__get_fd();
        super::super::fs::unix_native_dispatcher::UnixNativeDispatcher::close(fd)?;
        Ok(())
    }
}
