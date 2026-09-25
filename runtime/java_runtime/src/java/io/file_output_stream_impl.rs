use crate::prelude::*;
use super::file_output_stream::FileOutputStream;

/// 借出 fd 的 File 视图（不接管所有权：关闭由 FileDescriptor.close0 负责）。
fn borrow_file(fd: i32) -> std::mem::ManuallyDrop<std::fs::File> {
    use std::os::fd::FromRawFd;
    std::mem::ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(fd) })
}

/// OS 错误文案（JDK 的 strerror 形态：去掉 Rust Display 追加的 ` (os error N)`）。
fn os_error_text(e: &std::io::Error) -> std::string::String {
    let s = format!("{}", e);
    match s.find(" (os error ") {
        Some(i) => s[..i].to_owned(),
        None => s,
    }
}

fn io_err(e: std::io::Error) -> JvmError {
    match super::IOException::new_str(String::from(os_error_text(&e))) {
        Ok(x) => JvmError::from(x),
        Err(e) => e,
    }
}

impl FileOutputStream {
    /// native initIDs：HotSpot 缓存 JNI 字段 ID；原生二进制无此需要。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native open0(String, boolean)：按 JDK io_util_md.c 语义打开写入目标——
    /// `O_WRONLY | O_CREAT | (append ? O_APPEND : O_TRUNC)`，权限 0666（受 umask）。
    /// 失败抛 FileNotFoundException，消息 `path (strerror)`。
    #[jvm_native(upcalls = "java/io/FileNotFoundException.<init>:(Ljava/lang/String;)V")]
    pub fn open0(&self, name: String, append: bool) -> Result<()> {
        use std::os::unix::fs::OpenOptionsExt;
        let path = format!("{}", name);
        let mut opts = std::fs::OpenOptions::new();
        opts.write(true).create(true).mode(0o666);
        if append {
            opts.append(true);
        } else {
            opts.truncate(true);
        }
        match opts.open(&path) {
            Ok(f) => {
                use std::os::fd::IntoRawFd;
                // fd 所有权转入 FileDescriptor（close0 关闭）
                self.__get_fd().__set_fd(f.into_raw_fd());
                Ok(())
            }
            Err(e) => {
                let msg = String::from(format!("{} ({})", path, os_error_text(&e)));
                Err(JvmError::from(super::FileNotFoundException::new_str(msg)?))
            }
        }
    }

    /// native write(int, boolean)：写单字节（取低 8 位）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn write(&self, b: i32, _append: bool) -> Result<()> {
        self.write_all_fd(&[b as u8])
    }

    /// native writeBytes(byte[], int, int, boolean)：写入底层文件描述符。
    /// 与 write(2) 一致不做用户态缓冲：写完立即 flush。越界按 io_util.c 抛
    /// ArrayIndexOutOfBoundsException（经 JArray::get）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn writeBytes(&self, b: JArray<i8>, off: i32, len: i32, _append: bool) -> Result<()> {
        let mut bytes: Vec<u8> = Vec::with_capacity(len.max(0) as usize);
        for i in off..off + len {
            bytes.push(b.get(i)? as u8);
        }
        self.write_all_fd(&bytes)
    }

    fn write_all_fd(&self, bytes: &[u8]) -> Result<()> {
        use std::io::Write;
        let fd = self.__get_fd().__get_fd();
        let io_result = match fd {
            1 => {
                let mut h = std::io::stdout().lock();
                h.write_all(bytes).and_then(|_| h.flush())
            }
            2 => {
                let mut h = std::io::stderr().lock();
                h.write_all(bytes).and_then(|_| h.flush())
            }
            fd if fd >= 0 => borrow_file(fd).write_all(bytes),
            // 已关闭（-1）或只读虚拟 fd（嵌入资源）：JDK 为 IOException("Stream Closed")
            _ => return Err(io_err(std::io::Error::other("Stream Closed"))),
        };
        io_result.map_err(io_err)
    }
}
