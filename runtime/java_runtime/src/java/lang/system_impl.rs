use crate::prelude::*;
use super::*;
use crate::java::io::{BufferedOutputStream, FileDescriptor, FileOutputStream, PrintStream};
use crate::sun::nio::cs::UTF_8;

impl System {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制的 native 方法即本文件的 Rust 函数。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    pub fn arraycopy(src: Object, src_pos: i32, dest: Object, dest_pos: i32, length: i32) -> Result<()> {
        // JVM arraycopy 是 memmove 语义：src 与 dest 是同一数组且区间重叠时，
        // 逐元素前向复制会把尚未读取的源元素覆盖掉（TimSort 的插入移位即此形态）。
        // 同一数组（对象标识相等）按区间方向选择复制顺序。
        macro_rules! try_copy {
            ($t:ty) => {
                if let Some(s) = src.0.as_any().downcast_ref::<JArray<$t>>() {
                    let d = dest.downcast::<JArray<$t>>();
                    let backward = s == &d && dest_pos > src_pos;
                    let range: Box<dyn Iterator<Item = i32>> = if backward {
                        Box::new((0..length).rev())
                    } else {
                        Box::new(0..length)
                    };
                    for i in range {
                        d.set(dest_pos + i, s.get(src_pos + i)?)?;
                    }
                    return Ok(());
                }
            };
        }
        try_copy!(i8);
        try_copy!(i32);
        try_copy!(i64);
        try_copy!(u16);
        try_copy!(f32);
        try_copy!(f64);
        try_copy!(bool);
        try_copy!(i16);
        try_copy!(Object);
        panic!("stub: System.arraycopy: unsupported array element type")
    }

    #[jvm_native]
    pub fn currentTimeMillis() -> Result<i64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
    }

    #[jvm_native]
    pub fn nanoTime() -> Result<i64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as i64)
    }

    /// static lineSeparator：JDK 在 initPhase1 中由 line.separator 属性赋值（不经 `<clinit>`）；
    /// 原生二进制直接给出宿主平台的行分隔符。签名与宏生成的 static 访问器一致（`Result<T>`）。
    #[jvm_native]
    pub fn lineSeparator_field() -> Result<String> {
        Ok(if cfg!(windows) { String::from("\r\n") } else { String::from("\n") })
    }

    /// System.out / System.err：HotSpot 在 initPhase1 经 native setOut0 / setErr0 写入的静态字段，
    /// 不经 `<clinit>`，故由手写层提供（签名与宏生成的 static 访问器一致：`Result<T>`）。
    /// 对象图与 JDK 一致：PrintStream(BufferedOutputStream(FileOutputStream(fd), 128), autoFlush)，
    /// 三个类全部是字节码翻译版本；本函数只负责「native 写入静态字段」这一步。
    #[jvm_native(upcalls = "
        java/io/FileDescriptor.<init>:(I)V
        java/io/FileOutputStream.<init>:(Ljava/io/FileDescriptor;)V
        java/io/BufferedOutputStream.<init>:(Ljava/io/OutputStream;I)V
        java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V
        sun/nio/cs/UTF_8.INSTANCE:Lsun/nio/cs/UTF_8;
    ")]
    pub fn out() -> Result<PrintStream> {
        thread_local! {
            static STDOUT: PrintStream = new_std_print_stream(1);
        }
        Ok(STDOUT.with(Clone::clone))
    }

    #[jvm_native(upcalls = "
        java/io/FileDescriptor.<init>:(I)V
        java/io/FileOutputStream.<init>:(Ljava/io/FileDescriptor;)V
        java/io/BufferedOutputStream.<init>:(Ljava/io/OutputStream;I)V
        java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V
        sun/nio/cs/UTF_8.INSTANCE:Lsun/nio/cs/UTF_8;
    ")]
    pub fn err() -> Result<PrintStream> {
        thread_local! {
            static STDERR: PrintStream = new_std_print_stream(2);
        }
        Ok(STDERR.with(Clone::clone))
    }
}

/// 标准流的构造（对应 System.newPrintStream(new FileOutputStream(fd), enc)，enc 固定为 UTF-8）。
fn new_std_print_stream(fd: i32) -> PrintStream {
    let build = || -> Result<PrintStream> {
        let fdo = FileDescriptor::new_i(fd)?;
        let fos = FileOutputStream::new_filedescriptor(fdo)?;
        let bos = BufferedOutputStream::new_outputstream_i(fos.into(), 128)?;
        PrintStream::new_outputstream_z_charset(bos.into(), true, UTF_8::INSTANCE()?.into())
    };
    build().unwrap_or_else(|e| panic!("System 标准流初始化失败: {:?}", e))
}
