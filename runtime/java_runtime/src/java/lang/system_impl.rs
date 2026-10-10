use crate::prelude::*;
use super::*;
use crate::java::io::{BufferedInputStream, BufferedOutputStream, FileDescriptor, FileInputStream, FileOutputStream, InputStream, PrintStream};
use crate::sun::nio::cs::UTF_8;

impl System {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制的 native 方法即本文件的 Rust 函数，无事可做。
    /// System 在构建期引导映像中初始化（`[concrete.boot] init`），`System.props` 与 `VM.savedProps` 由构建期
    /// 执行的 initPhase1 写入映像，宿主相关键经 `SystemProps$Raw` 的启动重放取值（system_props_raw_impl.rs）
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    pub fn arraycopy(src: Object, src_pos: i32, dest: Object, dest_pos: i32, length: i32) -> Result<()> {
        // JVM 规范（System.arraycopy）：src 或 dest 为 null → NullPointerException；
        // src 或 dest 不是数组 → ArrayStoreException（可被 java_try 捕获，不 panic）。
        if src.0.is_jvm_null() || dest.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        // JVM arraycopy 是 memmove 语义：src 与 dest 是同一数组且区间重叠时，
        // 逐元素前向复制会把尚未读取的源元素覆盖掉（TimSort 的插入移位即此形态）。
        // 同一数组（对象标识相等）按区间方向选择复制顺序。
        macro_rules! try_copy {
            ($t:ty) => {
                if let (Some(s), Some(d)) = (
                    src.0.as_any().downcast_ref::<crate::array::__ArrayObj<$t>>(),
                    dest.try_checkcast::<JArray<$t>>(),
                ) {
                    let backward = s.identity() == d.identity() && dest_pos > src_pos;
                    let range: Box<dyn Iterator<Item = i32>> = if backward {
                        Box::new((0..length).rev())
                    } else {
                        Box::new(0..length)
                    };
                    for i in range {
                        d.set(dest_pos.wrapping_add(i), s.get(src_pos.wrapping_add(i))?)?;
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
        // 引用元素数组的其余形态（多维数组 JArray<JArray<T>>、以协变视图 / Object 擦除
        // 流转的数组）：经 `__view_into` 构造 Object 级协变视图逐元素复制（S-4）——
        // 写入走源数组的 aastore 存储检查，元素类型不兼容抛 ArrayStoreException；
        // 同一数组的重叠区间按对象标识（视图委托源数组）识别，保持 memmove 语义。
        let unused = crate::sync_model::__unused_any();
        let mut src_view: Option<JArray<Object>> = None;
        let mut dest_view: Option<JArray<Object>> = None;
        src.0.__view_into(crate::sync_model::__Shared::clone(&unused), &mut src_view);
        dest.0.__view_into(unused, &mut dest_view);
        if let (Some(s), Some(d)) = (src_view, dest_view) {
            let backward = s == d && dest_pos > src_pos;
            let range: Box<dyn Iterator<Item = i32>> = if backward {
                Box::new((0..length).rev())
            } else {
                Box::new(0..length)
            };
            for i in range {
                d.set(dest_pos.wrapping_add(i), s.get(src_pos.wrapping_add(i))?)?;
            }
            return Ok(());
        }
        Err(crate::error::JvmError::array_store(src.0.__class_name()))
    }

    #[jvm_native]
    pub fn currentTimeMillis() -> Result<i64> {
        // 挂钟（epoch 毫秒）
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
    }

    /// native `nanoTime()`：单调时钟（起点任意，JVM 同语义）。以进程首次调用为基准、
    /// 叠加 epoch 纳秒起点，数值量级与 JVM 相近且严格单调不减。
    #[jvm_native]
    pub fn nanoTime() -> Result<i64> {
        static ORIGIN: std::sync::OnceLock<(std::time::Instant, i64)> = std::sync::OnceLock::new();
        let (base, epoch) = *ORIGIN.get_or_init(|| (
            std::time::Instant::now(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos() as i64,
        ));
        Ok(epoch.saturating_add(base.elapsed().as_nanos() as i64))
    }

    /// native `identityHashCode(Object)I`：对象身份哈希（与内容无关，
    /// `Object.hashCode` 的默认语义）。JLS 未规定算法，仅要求同对象多次调用
    /// 一致、不同对象尽量不同；载体经 vtable `__identity()`（存储地址）取身份
    /// 后截断到 32 位——与 JVM 的 32 位身份哈希同宽度（碰撞行为等价级）。
    /// null 载体 → 0（JDK 语义）。
    #[jvm_native]
    pub fn identityHashCode(x: Object) -> Result<i32> {
        if x.0.is_jvm_null() {
            return Ok(0);
        }
        Ok(super::object::__object_identity_hash(&*x.0))
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
    #[jvm_native]
    pub fn out() -> Result<PrintStream> {
        Ok(std_stream(&STDOUT, 1))
    }

    #[jvm_native]
    pub fn err() -> Result<PrintStream> {
        Ok(std_stream(&STDERR, 2))
    }

    /// native `setOut0(PrintStream)`：System.setOut 的写入步（字段 final，JDK 经 native 改写）。
    #[jvm_native]
    pub fn setOut0(out: PrintStream) -> Result<()> {
        STDOUT.with(|slot| *slot.borrow_mut() = Some(out));
        Ok(())
    }

    /// native `setErr0(PrintStream)`：System.setErr 的写入步。
    #[jvm_native]
    pub fn setErr0(err: PrintStream) -> Result<()> {
        STDERR.with(|slot| *slot.borrow_mut() = Some(err));
        Ok(())
    }

    /// System.in：HotSpot 在 initPhase1 经 native setIn0 写入的静态字段（`<clinit>` 只写 null），
    /// 故与 out / err 同样由手写层提供。对象图与 JDK initPhase1 一致：
    /// `BufferedInputStream(FileInputStream(FileDescriptor.in))`，两个流类都是字节码翻译版本；
    /// 本函数只负责「native 写入静态字段」这一步（首次读取时建立，setIn0 改写）。
    #[jvm_native]
    pub fn in_() -> Result<InputStream> {
        if let Some(s) = STDIN.with(|s| s.borrow().as_ref().map(Clone::clone)) {
            return Ok(s);
        }
        let fis = FileInputStream::new_filedescriptor(FileDescriptor::in_()?)?;
        let bis: InputStream = BufferedInputStream::new_inputstream(fis.into())?.into();
        Ok(STDIN.with(|s| {
            let mut b = s.borrow_mut();
            // 并发首次读取：先写入者胜出，保持单一流身份
            Clone::clone(b.get_or_insert(bis))
        }))
    }

    /// native `setIn0(InputStream)`：System.setIn 的写入步（字段 final，JDK 经 native 改写）。
    #[jvm_native]
    pub fn setIn0(input: InputStream) -> Result<()> {
        STDIN.with(|slot| *slot.borrow_mut() = Some(input));
        Ok(())
    }

    /// native `mapLibraryName(String)`：平台本地库文件名（Linux `lib<name>.so`，
    /// macOS `lib<name>.dylib`）；null → NPE（JDK 同）。
    #[jvm_native]
    pub fn mapLibraryName(libname: String) -> Result<String> {
        if libname.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let suffix = if cfg!(target_os = "macos") { "dylib" } else { "so" };
        Ok(String::from_owned(format!("lib{}.{}", libname, suffix)))
    }
}

crate::__process_static! {
    /// System.out / System.err 的当前流：首次读取时建标准流（fd 1 / 2），setOut0 / setErr0 改写。
    static STDOUT: crate::sync_model::__RefSlot<Option<PrintStream>> = const { crate::sync_model::__RefSlot::new(None) };
    static STDERR: crate::sync_model::__RefSlot<Option<PrintStream>> = const { crate::sync_model::__RefSlot::new(None) };
    /// System.in 的当前流：首次读取时建标准输入流（fd 0），setIn0 改写。
    static STDIN: crate::sync_model::__RefSlot<Option<InputStream>> = const { crate::sync_model::__RefSlot::new(None) };
}

fn std_stream(
    slot: &'static crate::sync_model::__GilStatic<crate::sync_model::__RefSlot<Option<PrintStream>>>,
    fd: i32,
) -> PrintStream {
    if let Some(ps) = slot.with(|s| s.borrow().as_ref().map(Clone::clone)) {
        return ps;
    }
    let ps = new_std_print_stream(fd);
    slot.with(|s| {
        let mut b = s.borrow_mut();
        // 并发首次读取：先写入者胜出，保持单一流身份
        Clone::clone(b.get_or_insert(ps))
    })
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
