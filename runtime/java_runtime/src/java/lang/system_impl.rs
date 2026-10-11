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

    /// native `arraycopy`：HotSpot `JVM_ArrayCopy` → `Klass::copy_array` 同序同消息——
    /// null → NPE；源非数组 / 目标非数组 / 元素种类不符 → ArrayStoreException；
    /// 负下标 / 负长度 → ArrayIndexOutOfBoundsException；区间越界（无符号和）→ 同上；
    /// 全部校验通过才复制（目标不被部分写入），同一数组的重叠区间按 memmove 语义。
    /// 引用数组逐元素存储检查失败时，失败元素之前的已复制（HotSpot checkcast 复制同）。
    #[jvm_native]
    pub fn arraycopy(src: Object, src_pos: i32, dest: Object, dest_pos: i32, length: i32) -> Result<()> {
        if src.0.is_jvm_null() || dest.0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let Some(s_len) = src.0.__array_len() else {
            return Err(arraycopy_store(format!("arraycopy: source type {} is not an array", external_name(&src)?)));
        };
        let s_len = s_len?;
        let s_kind = elem_kind(&src);
        let Some(d_len) = dest.0.__array_len() else {
            return Err(arraycopy_store(format!("arraycopy: destination type {} is not an array", external_name(&dest)?)));
        };
        let d_len = d_len?;
        let d_kind = elem_kind(&dest);
        if s_kind != d_kind {
            return Err(arraycopy_store(format!("arraycopy: type mismatch: can not copy {}[] into {}[]",
                                               kind_name(s_kind), kind_name(d_kind))));
        }
        let kind = kind_name(s_kind);
        if src_pos < 0 || dest_pos < 0 || length < 0 {
            let msg = if src_pos < 0 {
                format!("arraycopy: source index {src_pos} out of bounds for {kind}[{s_len}]")
            } else if dest_pos < 0 {
                format!("arraycopy: destination index {dest_pos} out of bounds for {kind}[{d_len}]")
            } else {
                format!("arraycopy: length {length} is negative")
            };
            return Err(JvmError::array_index_out_of_bounds_message(msg));
        }
        let (last_src, last_dest) = ((length as u32).wrapping_add(src_pos as u32), (length as u32).wrapping_add(dest_pos as u32));
        if last_src > s_len as u32 {
            return Err(JvmError::array_index_out_of_bounds_message(format!(
                "arraycopy: last source index {last_src} out of bounds for {kind}[{s_len}]")));
        }
        if last_dest > d_len as u32 {
            return Err(JvmError::array_index_out_of_bounds_message(format!(
                "arraycopy: last destination index {last_dest} out of bounds for {kind}[{d_len}]")));
        }
        if length == 0 {
            return Ok(());
        }
        // 校验后逐元素复制：同一数组（对象标识相等）且目标在源之后时倒序（memmove）
        let order = |same: bool| -> Box<dyn Iterator<Item = i32>> {
            if same && dest_pos > src_pos { Box::new((0..length).rev()) } else { Box::new(0..length) }
        };
        macro_rules! copy_prim {
            ($t:ty) => {
                if let (Some(s), Some(d)) = (
                    src.0.as_any().downcast_ref::<crate::array::__ArrayObj<$t>>(),
                    dest.try_checkcast::<JArray<$t>>(),
                ) {
                    for i in order(s.identity() == d.identity()) {
                        d.set(dest_pos + i, s.get(src_pos + i)?)?;
                    }
                    return Ok(());
                }
            };
        }
        match s_kind {
            Some(_) => {
                copy_prim!(i8);
                copy_prim!(i32);
                copy_prim!(i64);
                copy_prim!(u16);
                copy_prim!(f32);
                copy_prim!(f64);
                copy_prim!(bool);
                copy_prim!(i16);
            }
            None => {
                // 引用元素数组：经 `__view_into` 取 Object 级协变视图（多维数组、Object 擦除流转的数组同径），
                // 写入走目标数组的 aastore 存储检查；视图委托源数组，同一数组按对象标识识别
                let unused = crate::sync_model::__unused_any();
                let mut src_view: Option<JArray<Object>> = None;
                let mut dest_view: Option<JArray<Object>> = None;
                src.0.__view_into(crate::sync_model::__Shared::clone(&unused), &mut src_view);
                dest.0.__view_into(unused, &mut dest_view);
                if let (Some(s), Some(d)) = (src_view, dest_view) {
                    for i in order(s == d) {
                        if d.set(dest_pos + i, s.get(src_pos + i)?).is_err() {
                            return Err(element_store_error(&src, &dest)?);
                        }
                    }
                    return Ok(());
                }
            }
        }
        Err(JvmError::array_store(&external_name(&src)?))
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
        STDOUT.set(Some(out));
        Ok(())
    }

    /// native `setErr0(PrintStream)`：System.setErr 的写入步。
    #[jvm_native]
    pub fn setErr0(err: PrintStream) -> Result<()> {
        STDERR.set(Some(err));
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
        // 并发首次读取：先写入者胜出，保持单一流身份；落选的流在锁外释放
        Ok(STDIN.with(|s| s.get_or_insert(bis)))
    }

    /// native `setIn0(InputStream)`：System.setIn 的写入步（字段 final，JDK 经 native 改写）。
    #[jvm_native]
    pub fn setIn0(input: InputStream) -> Result<()> {
        STDIN.set(Some(input));
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
    // 并发首次读取：先写入者胜出，保持单一流身份；落选的流在锁外释放
    slot.with(|s| s.get_or_insert(ps))
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

/// 数组元素种类：基本类型名（HotSpot `type2name_tab`）；引用元素数组 → None（消息记作 `object array`）。
/// 调用方已确认对象是数组。
fn elem_kind(a: &Object) -> Option<&'static str> {
    macro_rules! probe {
        ($($t:ty => $n:literal),*) => {
            $(if a.0.as_any().downcast_ref::<crate::array::__ArrayObj<$t>>().is_some() { return Some($n); })*
        };
    }
    probe!(i8 => "byte", i32 => "int", i64 => "long", u16 => "char", f32 => "float", f64 => "double",
           bool => "boolean", i16 => "short");
    None
}

fn kind_name(k: Option<&'static str>) -> &'static str {
    k.unwrap_or("object array")
}

/// 运行时类的外部名（`Klass::external_name`：`java.lang.String`、`[I`、`[Ljava.lang.String;`）
fn external_name(o: &Object) -> Result<std::string::String> {
    Ok(format!("{}", o.0.getClass()?.__get_name()))
}

fn arraycopy_store(msg: std::string::String) -> JvmError {
    match crate::java::lang::ArrayStoreException::new_str(String::from(msg.as_str())) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// 引用数组的元素存储检查失败（`ObjArrayKlass::do_copy`）：目标元素类型 bound 不是源元素类型 stype
/// 的子类型 → `type mismatch: can not copy S[] into B[]`，否则 → `element type mismatch: can not cast one of
/// the elements of S[] to the type of the destination array, B`（JDK 21 实测）。
fn element_store_error(src: &Object, dest: &Object) -> Result<JvmError> {
    let component = |o: &Object| -> Result<std::string::String> {
        let n = external_name(o)?;
        let c = &n[1..];
        Ok(c.strip_prefix('L').and_then(|x| x.strip_suffix(';')).unwrap_or(c).to_string())
    };
    let (stype, bound) = (component(src)?, component(dest)?);
    let class_of = |n: &str| Class::for_class(String::from(n.replace('.', "/").as_str()));
    let msg = if class_of(&stype).isAssignableFrom(class_of(&bound))? {
        format!("arraycopy: element type mismatch: can not cast one of the elements of {stype}[] to the type of the destination array, {bound}")
    } else {
        format!("arraycopy: type mismatch: can not copy {stype}[] into {bound}[]")
    };
    Ok(arraycopy_store(msg))
}
