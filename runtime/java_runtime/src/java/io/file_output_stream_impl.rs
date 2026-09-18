use crate::prelude::*;
use super::file_output_stream::FileOutputStream;

impl FileOutputStream {
    /// native initIDs：HotSpot 缓存 JNI 字段 ID；原生二进制无此需要。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native writeBytes(byte[], int, int, boolean)：写入底层文件描述符。
    /// 与 write(2) 一致不做用户态缓冲：写完立即 flush。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn writeBytes(&self, b: JArray<i8>, off: i32, len: i32, _append: bool) -> Result<()> {
        use std::io::Write;
        let mut bytes: Vec<u8> = Vec::with_capacity(len.max(0) as usize);
        for i in off..off + len {
            bytes.push(b.get(i)? as u8);
        }
        let fd = self.__get_fd().__get_fd();
        let io_result = match fd {
            1 => {
                let mut h = std::io::stdout().lock();
                h.write_all(&bytes).and_then(|_| h.flush())
            }
            2 => {
                let mut h = std::io::stderr().lock();
                h.write_all(&bytes).and_then(|_| h.flush())
            }
            _ => panic!("stub: java/io/FileOutputStream.writeBytes:([BIIZ)V (fd={})", fd),
        };
        match io_result {
            Ok(()) => Ok(()),
            Err(e) => Err(super::IOException::new_str(String::from(format!("{}", e)))?.into()),
        }
    }
}
