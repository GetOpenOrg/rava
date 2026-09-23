use crate::prelude::*;
use super::file_input_stream::FileInputStream;
use crate::jdk_resources;

// FileInputStream 的 native 层：文件读取的单一资源决策点。
//
// open0 按路径特征分流：命中嵌入 JDK 资源（<JAVA_RUNTIME_HOME>/lib/…，
// 见 jdk_resources 模块）→ 虚拟 fd（负数句柄，光标态在全局注册表）；
// 未命中 → std::fs 打开宿主真实文件，fd 存 OS 原生描述符（非负 i32，
// 与 Unix JDK 的 FileDescriptor.fd 同义）。read0/readBytes/skip0/
// available0/length0/position0 按 fd 符号分派到两个后端，语义与
// HotSpot io_util.c / io_util_md.c 对齐（read 部分读返回实读数、EOF
// 返回 -1、skip 夹取到 EOF、available 为 FIONREAD 的剩余字节数）。

/// 以 `File` 重建 OS fd 的只读视图（不取得所有权——fd 的生命周期由
/// FileDescriptor.close0 管理）。
fn borrow_file(fd: i32) -> std::mem::ManuallyDrop<std::fs::File> {
    use std::os::fd::FromRawFd;
    std::mem::ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(fd) })
}

fn io_err(e: std::io::Error) -> JvmError {
    JvmError::from(super::IOException::new_str(String::from(format!("{}", e))).unwrap())
}

impl FileInputStream {
    /// native initIDs：HotSpot 缓存 JNI 字段 ID；原生二进制无此需要。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native open0(String)：打开 name 指向的字节源并写入 this.fd。
    /// 资源重定向决策点：嵌入 JDK 数据文件命中 → 虚拟 fd；否则宿主文件。
    /// 失败按 JDK 语义抛 FileNotFoundException（消息 `path (errno 文案)`）。
    #[jvm_native(upcalls = "java/io/FileNotFoundException.<init>:(Ljava/lang/String;)V")]
    pub fn open0(&self, name: String) -> Result<()> {
        let path = format!("{}", name);
        match jdk_resources::open_embedded(&path) {
            Some(vfd) => self.__get_fd().__set_fd(vfd),
            None => match std::fs::File::open(&path) {
                Ok(f) => {
                    use std::os::fd::AsRawFd;
                    // fd 所有权转入 FileDescriptor（close0 关闭）；借出视图立即丢弃
                    let raw = f.as_raw_fd();
                    std::mem::forget(f);
                    self.__get_fd().__set_fd(raw);
                }
                Err(e) => {
                    let msg = String::from(format!("{} ({})", path, e));
                    return Err(JvmError::from(
                        super::FileNotFoundException::new_str(msg)?));
                }
            },
        }
        Ok(())
    }

    /// native read0()：读单字节，0-255；EOF 返回 -1。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn read0(&self) -> Result<i32> {
        let fd = self.__get_fd().__get_fd();
        if fd <= -2 {
            return Ok(jdk_resources::virtual_read_byte(fd).map(|b| b as i32).unwrap_or(-1));
        }
        if fd < 0 {
            return Err(io_err(std::io::Error::last_os_error()));
        }
        let mut one = [0u8; 1];
        use std::io::Read;
        let n = borrow_file(fd).read(&mut one).map_err(io_err)?;
        Ok(if n == 1 { one[0] as i32 } else { -1 })
    }

    /// native readBytes(byte[], off, len)：批量读，返回实读数；EOF 返回 -1；
    /// len == 0 返回 0。越界按 io_util.c 抛 ArrayIndexOutOfBoundsException。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn readBytes(&self, b: JArray<i8>, off: i32, len: i32) -> Result<i32> {
        let fd = self.__get_fd().__get_fd();
        let blen = b.len()?;
        if off < 0 || len < 0 || len > blen - off {
            let msg = String::from(format!("Array index out of range: {}", off));
            return Err(JvmError::from(
                crate::java::lang::ArrayIndexOutOfBoundsException::new_str(msg)?));
        }
        if len == 0 {
            return Ok(0);
        }
        let mut buf = vec![0u8; len as usize];
        let n: usize = if fd <= -2 {
            jdk_resources::virtual_read(fd, &mut buf)
        } else if fd < 0 {
            return Err(io_err(std::io::Error::last_os_error()));
        } else {
            use std::io::Read;
            borrow_file(fd).read(&mut buf).map_err(io_err)?
        };
        if n == 0 {
            return Ok(-1);
        }
        for i in 0..n {
            b.set(off + i as i32, buf[i] as i8)?;
        }
        Ok(n as i32)
    }

    /// native skip0(long)：前跳至多 n 字节（夹取到 EOF），返回实跳数。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn skip0(&self, n: i64) -> Result<i64> {
        if n <= 0 {
            return Ok(0);
        }
        let fd = self.__get_fd().__get_fd();
        if fd <= -2 {
            return Ok(jdk_resources::virtual_skip(fd, n));
        }
        if fd < 0 {
            return Err(io_err(std::io::Error::last_os_error()));
        }
        use std::io::Seek;
        let mut f = borrow_file(fd);
        let cur = f.stream_position().map_err(io_err)?;
        let end = f.seek(std::io::SeekFrom::End(0)).map_err(io_err)?;
        let target = (cur + n as u64).min(end);
        f.seek(std::io::SeekFrom::Start(target)).map_err(io_err)?;
        Ok((target - cur) as i64)
    }

    /// native available0()：不经阻塞可读字节数（常规文件 = 剩余字节）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn available0(&self) -> Result<i32> {
        let fd = self.__get_fd().__get_fd();
        if fd <= -2 {
            return Ok(jdk_resources::virtual_available(fd).min(i32::MAX as i64) as i32);
        }
        if fd < 0 {
            return Err(io_err(std::io::Error::last_os_error()));
        }
        use std::io::Seek;
        let mut f = borrow_file(fd);
        let len = f.metadata().map_err(io_err)?.len();
        let pos = f.stream_position().map_err(io_err)?;
        Ok((len.saturating_sub(pos)).min(i32::MAX as u64) as i32)
    }

    /// native length0()：文件总长（transferTo 快路径探测）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn length0(&self) -> Result<i64> {
        let fd = self.__get_fd().__get_fd();
        if fd <= -2 {
            return Ok(jdk_resources::virtual_length(fd));
        }
        if fd < 0 {
            return Err(io_err(std::io::Error::last_os_error()));
        }
        Ok(borrow_file(fd).metadata().map_err(io_err)?.len() as i64)
    }

    /// native position0()：当前文件偏移。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn position0(&self) -> Result<i64> {
        let fd = self.__get_fd().__get_fd();
        if fd <= -2 {
            return Ok(jdk_resources::virtual_position(fd));
        }
        if fd < 0 {
            return Err(io_err(std::io::Error::last_os_error()));
        }
        use std::io::Seek;
        Ok(borrow_file(fd).stream_position().map_err(io_err)? as i64)
    }
}
