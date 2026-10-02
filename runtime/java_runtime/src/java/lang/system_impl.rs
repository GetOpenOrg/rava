use crate::prelude::*;
use super::*;
use crate::java::io::{BufferedInputStream, BufferedOutputStream, FileDescriptor, FileInputStream, FileOutputStream, InputStream, PrintStream};
use crate::sun::nio::cs::UTF_8;

impl System {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制的 native 方法即本文件的 Rust 函数。
    ///
    /// 同时承担 HotSpot `System.initPhase1` 的角色：`<clinit>` 后由 VM 引导填充
    /// `System.props`。原生二进制无 -D 注入机制，属性表的键集与常量值来自闭包分析器折叠
    /// 属性读点所用的同一张表（清单 `vm_intrinsics.toml [facts.system_properties]`，经
    /// closure.json 由 java_meta 构建脚本生成，经 `crate::meta::vm_const_properties` /
    /// `vm_dynamic_properties` 读取）：
    /// - 常量键按表中值写入；
    /// - 动态键由本层取宿主值（[`host_property`]：文件系统 / 用户 / os / 编码族，
    ///   `java.home` 为嵌入资源伪值 jdk_resources::JAVA_RUNTIME_HOME，`user.timezone`
    ///   只在 TZ 存在时设），版本族由翻译的 `VersionProps.init(Map)` 写入（JDK initPhase1
    ///   同一来源），VM 族随版本族派生（[`derived_vm_property`]）；
    /// - 表外的键不写入（分析器按缺省 null 折叠），手写层无取值的动态键同样缺席。
    ///
    /// 构造不经 JDK 构造器链（Properties.<init> → Hashtable 族种子在 sig_types 载体化
    /// 上有 codegen 域缺口），按擦除字段协议直接挂后备 ConcurrentHashMap
    /// （Properties.getProperty 消费 `map` 字段）。
    ///
    /// 属性表建成后与 initPhase1 同样交 `VM.saveProperties`（翻译的字节码）保存快照。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        use crate::java::util::concurrent::ConcurrentHashMap;
        let map = ConcurrentHashMap::<Object, Object>::new()?;
        // 局部闭包不取 Java 方法名（分析器按「名字 + 实参个数」反解手写回调的实参来源）
        let store = |k: &str, v: &str| -> Result<()> {
            map.put(Object::from(String::from(k)), Object::from(String::from(v)))?;
            Ok(())
        };
        for (k, v) in crate::meta::vm_const_properties() {
            store(k, v)?;
        }
        for k in crate::meta::vm_dynamic_properties() {
            if let Some(v) = host_property(k) {
                store(k, &v)?;
            }
        }
        crate::java::lang::VersionProps::init(
            Object::from(Clone::clone(&map)).try_cast("java/util/Map")?)?;
        for k in crate::meta::vm_dynamic_properties() {
            if let Some(v) = derived_vm_property(k, &map)? {
                store(k, &v)?;
            }
        }
        // initPhase1 同序：保存属性快照（VM.saveProperties：directMemory / pageAlignDirectMemory /
        // classFileMajorVersion 等按快照取值，未指定 -XX:MaxDirectMemorySize 时取 Runtime.maxMemory()）。
        // saveProperties 只允许在 initLevel == 0 调用，本段即 initPhase1 的引导段
        let snapshot: crate::java::util::Map<Object, Object> = Object::from(Clone::clone(&map)).try_cast("java/util/Map")?;
        crate::jdk::internal::misc::VM::__vm_at_init_level(0, || {
            crate::jdk::internal::misc::VM::saveProperties(snapshot)
        })?;
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
        let unused: crate::sync_model::__AnyRef = crate::sync_model::__Shared::new(());
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
        Ok(super::object::__identity_hash(x.0.__identity()))
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

/// 动态键的宿主取值（HotSpot `SystemProps.Raw` / `os::` 的同名来源）；版本族与 VM 族不在此
/// （分别由 VersionProps.init 与 [`derived_vm_property`] 写入）→ None
fn host_property(key: &str) -> Option<std::string::String> {
    let s = std::string::String::from;
    Some(match key {
        "java.home" => s(crate::jdk_resources::JAVA_RUNTIME_HOME),
        "line.separator" => s(if cfg!(windows) { "\r\n" } else { "\n" }),
        "user.dir" => std::env::current_dir().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default(),
        "user.home" => std::env::var("HOME").unwrap_or_default(),
        "user.name" => crate::posix::current_user_name(),
        "java.io.tmpdir" => std::env::var("TMPDIR").unwrap_or_else(|_| s("/tmp")),
        "os.name" => s(crate::posix::os_name()),
        "os.arch" => s(crate::posix::os_arch()),
        "os.version" => crate::posix::os_release(),
        "sun.arch.data.model" => s(if cfg!(target_pointer_width = "64") { "64" } else { "32" }),
        "sun.cpu.endian" => s(if cfg!(target_endian = "little") { "little" } else { "big" }),
        "sun.io.unicode.encoding" => s(if cfg!(target_endian = "little") { "UnicodeLittle" } else { "UnicodeBig" }),
        "sun.boot.library.path" => format!("{}/lib", crate::jdk_resources::JAVA_RUNTIME_HOME),
        // native / jnu 编码取宿主区域的 codeset（file.encoding 与标准流编码为常量键 UTF-8，JEP 400）
        "native.encoding" | "sun.jnu.encoding" => crate::posix::native_encoding(),
        // 区域族（SystemProps.Raw：来自宿主区域环境变量）；国家缺席则不设
        "user.language" => crate::posix::locale().0,
        "user.country" => Some(crate::posix::locale().1).filter(|c| !c.is_empty())?,
        // TZ 环境变量存在才设（JDK initPhase1 同款条件），缺席留给 TimeZone/ZoneId 惰性解析
        "user.timezone" => std::env::var("TZ").ok().filter(|tz| !tz.is_empty())?,
        _ => return None,
    })
}

/// VM 族动态键（HotSpot `Arguments::init_system_properties` / `VM_Version`）：规范版本取语料
/// JDK 的特性版本，实现侧版本号与供应商随 `java.runtime.version` / `java.vendor`
/// （VersionProps 已写入 map）；其余键 → None
fn derived_vm_property(
    key: &str,
    map: &crate::java::util::concurrent::ConcurrentHashMap<Object, Object>,
) -> Result<Option<std::string::String>> {
    let from = match key {
        "java.vm.specification.version" => "java.specification.version",
        "java.vm.vendor" => "java.vendor",
        "java.vm.version" => "java.runtime.version",
        _ => return Ok(None),
    };
    let v = map.get(Object::from(String::from(from)))?;
    Ok(Some(if v.0.is_jvm_null() { std::string::String::new() } else { format!("{}", v) }))
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
