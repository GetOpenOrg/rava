use crate::prelude::*;
use super::*;
use crate::java::io::{BufferedOutputStream, FileDescriptor, FileOutputStream, PrintStream};
use crate::sun::nio::cs::UTF_8;

impl System {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制的 native 方法即本文件的 Rust 函数。
    ///
    /// 同时承担 HotSpot `System.initPhase1` 的角色：`<clinit>` 后由 VM 引导填充
    /// `System.props`（JDK 里来自 VM 快照属性）。原生二进制无 -D 注入机制，
    /// 属性表填充为宿主导出的 VM 快照子集：文件系统分隔符 / 用户目录 / 临时
    /// 目录 / os 标识来自宿主 POSIX 环境，`java.home` 为嵌入资源伪值
    /// （jdk_resources::JAVA_RUNTIME_HOME，与 StaticProperty.javaHome 同源），
    /// `user.timezone` 来自 TZ（缺席则不设——JDK 惰性解析语义）。构造不经
    /// JDK 构造器链（Properties.<init> → Hashtable 族种子在 sig_types 载体化
    /// 上有 codegen 域缺口），按擦除字段协议直接挂后备 ConcurrentHashMap
    /// （Properties.getProperty 消费 `map` 字段）。
    /// 消费链：ZoneRulesProvider.<clinit> 的 doPrivileged 回调
    /// ZoneRulesProvider$1.run → System.getProperty（属性缺席 → 走
    /// TzdbZoneRulesProvider 默认分支）；UnixFileSystem.<init> →
    /// GetPropertyAction.privilegedGetProperties → file.separator /
    /// path.separator（缺席会 NPE）。
    #[jvm_native(upcalls = "
        java/util/concurrent/ConcurrentHashMap.<init>:()V
        java/util/concurrent/ConcurrentHashMap.put:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    ")]
    pub fn registerNatives() -> Result<()> {
        use crate::java::util::concurrent::ConcurrentHashMap;
        let map = ConcurrentHashMap::<Object, Object>::new()?;
        for (k, v) in vm_snapshot_properties() {
            map.put(Object::from(String::from(k)), Object::from(String::from(v)))?;
        }
        let mut p = crate::java::util::Properties::default();
        p._init_not_null();
        p.__set_map(map);
        System::set_props(p)?;
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
                    src.0.as_any().downcast_ref::<JArray<$t>>(),
                    dest.try_checkcast::<JArray<$t>>(),
                ) {
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
        // 引用元素数组的其余形态（多维数组 JArray<JArray<T>>、以协变视图 / Object 擦除
        // 流转的数组）：经 `__view_into` 构造 Object 级协变视图逐元素复制（S-4）——
        // 写入走源数组的 aastore 存储检查，元素类型不兼容抛 ArrayStoreException；
        // 同一数组的重叠区间按对象标识（视图委托源数组）识别，保持 memmove 语义。
        let unused: std::rc::Rc<dyn std::any::Any> = std::rc::Rc::new(());
        let mut src_view: Option<JArray<Object>> = None;
        let mut dest_view: Option<JArray<Object>> = None;
        src.0.__view_into(std::rc::Rc::clone(&unused), &mut src_view);
        dest.0.__view_into(unused, &mut dest_view);
        if let (Some(s), Some(d)) = (src_view, dest_view) {
            let backward = s == d && dest_pos > src_pos;
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
        Err(crate::error::JvmError::array_store(src.0.__class_name()))
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
        Ok(x.0.__identity() as i32)
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

/// VM 快照属性子集（initPhase1 对应物）：键集 = 库代码在类初始化/常规路径
/// 上会读取的标准属性。宿主 POSIX 环境导出；`java.home` 恒为嵌入资源伪值。
fn vm_snapshot_properties() -> Vec<(&'static str, std::string::String)> {
    let mut props = vec![
        ("java.home", crate::jdk_resources::JAVA_RUNTIME_HOME.to_owned()),
        ("file.separator", std::string::String::from("/")),
        ("path.separator", std::string::String::from(":")),
        ("line.separator", std::string::String::from(if cfg!(windows) { "\r\n" } else { "\n" })),
        ("user.dir", std::env::current_dir().map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default()),
        ("user.home", std::env::var("HOME").unwrap_or_default()),
        ("user.name", std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .unwrap_or_default()),
        ("java.io.tmpdir", std::env::var("TMPDIR").unwrap_or_else(|_| std::string::String::from("/tmp"))),
        ("os.name", std::string::String::from(if cfg!(target_os = "macos") { "Mac OS X" }
            else if cfg!(target_os = "linux") { "Linux" }
            else { std::env::consts::OS })),
        ("os.arch", std::string::String::from(if cfg!(target_arch = "aarch64") { "aarch64" }
            else if cfg!(target_arch = "x86_64") { "amd64" }
            else { std::env::consts::ARCH })),
    ];
    // user.timezone：TZ 环境变量存在才设（JDK initPhase1 同款条件），
    // 缺席留给 TimeZone/ZoneId 惰性解析
    if let Ok(tz) = std::env::var("TZ") {
        if !tz.is_empty() {
            props.push(("user.timezone", tz));
        }
    }
    props
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
