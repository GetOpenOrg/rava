use crate::prelude::*;
use super::file_output_stream::FileOutputStream;
use super::FileDescriptor_1;

impl FileOutputStream {
    /// static final FD_ACCESS：Java 里由 <clinit> 经 SharedSecrets 取得
    /// FileDescriptor 的匿名访问器实例；此处直接构造该（字节码翻译的）匿名类。
    #[jvm_native(upcalls = "java/io/FileDescriptor$1.<init>:()V")]
    pub fn FD_ACCESS() -> Object {
        let access = FileDescriptor_1::new()
            .unwrap_or_else(|e| panic!("FileDescriptor 访问器初始化失败: {:?}", e));
        Object::from(access)
    }

    /// native writeBytes(byte[], int, int, boolean)：写入底层文件描述符。
    /// 与 write(2) 一致不做用户态缓冲：写完立即 flush。
    #[jvm_native]
    pub fn writeBytes(&self, b: JArray<i8>, off: i32, len: i32, _append: bool) -> Result<()> {
        use std::io::Write;
        let bytes: Vec<u8> = (off..off + len).map(|i| b.get(i) as u8).collect();
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
        io_result.map_err(|e| JvmError::Custom(format!("java.io.IOException: {}", e)))
    }
}
