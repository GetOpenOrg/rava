use crate::prelude::*;
use super::random_access_file::RandomAccessFile;

// RandomAccessFile 的 native 层（gap_scan 缺口批次 1）：对标 HotSpot RandomAccessFile.c /
// io_util.c / io_util_md.c。fd 存 OS 原生描述符（FileDescriptor.fd），生命周期由
// FileDescriptor.close0 管理；读写 / 定位按 std::fs::File 的借出视图执行。
// open0 的 mode 位即 RandomAccessFile 的 O_RDONLY(1) / O_RDWR(2) / O_SYNC(4) / O_DSYNC(8)。

const O_RDONLY: i32 = 1;
const O_SYNC: i32 = 4;
const O_DSYNC: i32 = 8;

fn borrow_file(fd: i32) -> std::mem::ManuallyDrop<std::fs::File> {
    use std::os::fd::FromRawFd;
    std::mem::ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(fd) })
}

/// OS 错误文案（JDK strerror 形态：去掉 ` (os error N)` 后缀）。
fn os_error_text(e: &std::io::Error) -> std::string::String {
    let s = format!("{}", e);
    match s.find(" (os error ") {
        Some(i) => s[..i].to_owned(),
        None => s,
    }
}

fn io_err(e: std::io::Error) -> JvmError {
    JvmError::from(super::IOException::new_str(String::from(os_error_text(&e))).unwrap())
}

fn closed() -> JvmError {
    JvmError::from(super::IOException::new_str(String::from("Stream Closed")).unwrap())
}

impl RandomAccessFile {
    fn __raw_fd(&self) -> Result<i32> {
        let fd = self.__get_fd().__get_fd();
        if fd < 0 { Err(closed()) } else { Ok(fd) }
    }

    /// native initIDs：HotSpot 缓存 JNI 字段 ID；原生二进制无此需要。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native open0(String, int)：只读或读写（不存在则创建）打开；O_SYNC / O_DSYNC 同步写。
    /// 失败抛 FileNotFoundException（消息 `path (strerror)`）。
    #[jvm_native(upcalls = "java/io/FileNotFoundException.<init>:(Ljava/lang/String;)V")]
    pub fn open0(&self, name: String, mode: i32) -> Result<()> {
        use std::os::unix::fs::OpenOptionsExt;
        let path = format!("{}", name);
        let mut opts = std::fs::OpenOptions::new();
        opts.read(true);
        if mode & O_RDONLY == 0 {
            opts.write(true).create(true);
            if mode & O_SYNC != 0 {
                opts.custom_flags(libc::O_SYNC);
            } else if mode & O_DSYNC != 0 {
                opts.custom_flags(libc::O_DSYNC);
            }
        }
        match opts.open(&path) {
            Ok(f) => {
                use std::os::fd::IntoRawFd;
                self.__get_fd().__set_fd(f.into_raw_fd());
                Ok(())
            }
            Err(e) => Err(JvmError::from(super::FileNotFoundException::new_str(
                String::from(format!("{} ({})", path, os_error_text(&e))))?)),
        }
    }

    /// native read0()：读单字节 0-255；EOF 返回 -1。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn read0(&self) -> Result<i32> {
        use std::io::Read;
        let mut f = borrow_file(self.__raw_fd()?);
        let mut buf = [0u8; 1];
        match f.read(&mut buf) {
            Ok(0) => Ok(-1),
            Ok(_) => Ok(buf[0] as i32),
            Err(e) => Err(io_err(e)),
        }
    }

    /// native readBytes0(byte[], int, int)：越界抛 IndexOutOfBoundsException；len 为 0
    /// 返回 0；EOF 返回 -1；否则返回实读字节数（部分读语义）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V java/lang/IndexOutOfBoundsException.<init>:()V")]
    pub fn readBytes0(&self, b: JArray<i8>, off: i32, len: i32) -> Result<i32> {
        use std::io::Read;
        if off < 0 || len < 0 || off as i64 + len as i64 > b.len()? as i64 {
            return Err(JvmError::from(crate::java::lang::IndexOutOfBoundsException::new()?));
        }
        if len == 0 {
            return Ok(0);
        }
        let mut f = borrow_file(self.__raw_fd()?);
        let mut buf = vec![0u8; len as usize];
        match f.read(&mut buf) {
            Ok(0) => Ok(-1),
            Ok(n) => {
                for (i, v) in buf[..n].iter().enumerate() {
                    b.set(off + i as i32, *v as i8)?;
                }
                Ok(n as i32)
            }
            Err(e) => Err(io_err(e)),
        }
    }

    /// native write0(int)：写单字节（低 8 位）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn write0(&self, b: i32) -> Result<()> {
        use std::io::Write;
        let mut f = borrow_file(self.__raw_fd()?);
        f.write_all(&[b as u8]).map_err(io_err)
    }

    /// native writeBytes0(byte[], int, int)：越界抛 IndexOutOfBoundsException，全部写出。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V java/lang/IndexOutOfBoundsException.<init>:()V")]
    pub fn writeBytes0(&self, b: JArray<i8>, off: i32, len: i32) -> Result<()> {
        use std::io::Write;
        if off < 0 || len < 0 || off as i64 + len as i64 > b.len()? as i64 {
            return Err(JvmError::from(crate::java::lang::IndexOutOfBoundsException::new()?));
        }
        let mut buf = Vec::with_capacity(len as usize);
        for i in off..off + len {
            buf.push(b.get(i)? as u8);
        }
        let mut f = borrow_file(self.__raw_fd()?);
        f.write_all(&buf).map_err(io_err)
    }

    /// native getFilePointer()：当前偏移（lseek(fd, 0, SEEK_CUR)）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn getFilePointer(&self) -> Result<i64> {
        use std::io::Seek;
        let mut f = borrow_file(self.__raw_fd()?);
        f.stream_position().map(|p| p as i64).map_err(io_err)
    }

    /// native seek0(long)：定位到绝对偏移（负值已由 Java 侧 seek 拦截）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn seek0(&self, pos: i64) -> Result<()> {
        use std::io::{Seek, SeekFrom};
        let mut f = borrow_file(self.__raw_fd()?);
        f.seek(SeekFrom::Start(pos as u64)).map(|_| ()).map_err(io_err)
    }

    /// native length0()：文件长度（fstat）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn length0(&self) -> Result<i64> {
        let f = borrow_file(self.__raw_fd()?);
        f.metadata().map(|m| m.len() as i64).map_err(io_err)
    }

    /// native setLength0(long)：ftruncate；原偏移超过新长度时移到新长度（JDK 同序）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn setLength0(&self, new_length: i64) -> Result<()> {
        use std::io::{Seek, SeekFrom};
        let mut f = borrow_file(self.__raw_fd()?);
        let cur = f.stream_position().map_err(io_err)?;
        f.set_len(new_length as u64).map_err(io_err)?;
        if cur as i64 > new_length {
            f.seek(SeekFrom::Start(new_length as u64)).map_err(io_err)?;
        }
        Ok(())
    }
}
