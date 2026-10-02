use crate::prelude::*;
use super::class_loader::ClassLoader;

// java.lang.ClassLoader 伴生。
//
// 内建加载器层级（app → platform → null）、getSystemClassLoader、getParent 一律走字节码
// （ClassLoaders 整类翻译）。本文件只承载：
//   - native 方法；
//   - initPhase3 系统类加载器段的落地（`__vm_init_phase3`，准入第 ③ 类：HotSpot 在进入 main 前执行，
//     原生二进制改为首次读写 `ClassLoader.scl` / `Thread.contextClassLoader` 时执行，见
//     vm_intrinsics.toml `[vm_state.field_hooks]`）；
//   - 运行期类定义点（第 ② 类）；
//   - 资源族：模块资源（jmod 内数据文件）由编译期嵌入表承载，其余资源恒缺席——单二进制无
//     classpath 资源，ServiceLoader 的 LazyClassPathLookupIterator 据此枚举为空。
impl ClassLoader {
    /// native `registerNatives()`（<clinit> 首句）：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native `findBootstrapClass(String name)`：引导加载器按 binary name（点分）查找已定义类，
    /// 找不到返回 null。原生镜像的类全集编译期定死、类对象全部预先存在，故「引导加载器可见」即
    /// 闭包内的类（与 `Class.forName0` 同一判定；定义加载器由镜像的 `classLoader` 钩子按
    /// `defining_loader` 表给出，不影响可见性）。
    #[jvm_native]
    pub fn findBootstrapClass(name: String) -> Result<super::Class> {
        let slash = format!("{}", name).replace('.', "/");
        if slash.starts_with('[') || !super::Class::__is_known_class(&slash) {
            return Ok(super::Class::default());
        }
        Ok(super::Class::for_class(String::from(slash.as_str())))
    }

    /// native `findLoadedClass0(String name)`：本加载器作为初始加载器记录过的类。原生镜像中
    /// 全部类由引导形态定义，非引导加载器从未成为任何类的初始加载器 → 恒 null
    /// （`loadClass` 随即委派父加载器 / findBootstrapClassOrNull，与 JDK 委派模型一致）。
    #[jvm_native]
    pub fn findLoadedClass0(&self, _name: String) -> Result<super::Class> {
        Ok(super::Class::default())
    }

    /// initPhase3 的系统类加载器段（`System.initPhase3`）：
    /// ```text
    /// VM.initLevel(3);
    /// ClassLoader scl = ClassLoader.initSystemClassLoader();
    /// Thread.currentThread().setContextClassLoader(scl);   // 初始线程
    /// VM.initLevel(4);
    /// ```
    /// 两步调用都走字节码（`initSystemClassLoader` 写 `scl`，`setContextClassLoader` 写初始线程的
    /// 上下文加载器）。进程内只执行一次：其余线程在段执行期间读写钩子字段时于互斥上等待；段内
    /// 本线程对钩子字段的读写（`putstatic scl`、`putfield contextClassLoader`）直接放行。
    pub fn __vm_init_phase3() -> Result<()> {
        use std::sync::atomic::{AtomicBool, Ordering};
        static DONE: AtomicBool = AtomicBool::new(false);
        static RUNNING: parking_lot::ReentrantMutex<std::cell::Cell<bool>> =
            parking_lot::const_reentrant_mutex(std::cell::Cell::new(false));
        if DONE.load(Ordering::Acquire) {
            return Ok(());
        }
        let guard = RUNNING.lock();
        if DONE.load(Ordering::Acquire) || guard.get() {
            return Ok(());
        }
        guard.set(true);
        let r = crate::jdk::internal::misc::VM::__vm_in_init_level3(|| -> Result<()> {
            let scl = ClassLoader::initSystemClassLoader()?;
            super::thread_impl::__vm_initial_thread()?.setContextClassLoader(scl)
        });
        guard.set(false);
        if r.is_ok() {
            DONE.store(true, Ordering::Release);
        }
        r
    }

    /// 单资源查询：单二进制无 classpath 资源 → 恒 null。
    #[jvm_boundary]
    pub fn getResource(&self, name: String) -> Result<crate::java::net::URL> {
        let _ = name;
        Ok(Default::default())
    }

    /// 资源枚举：恒空枚举（消费方 ServiceLoader 迭代即终止）。
    #[jvm_boundary]
    pub fn getResources(&self, name: String) -> Result<crate::java::util::Enumeration<Object>> {
        let _ = name;
        crate::java::util::Collections::emptyEnumeration()
    }

    /// static getSystemResource：委托实例形态（恒 null）。
    #[jvm_boundary]
    pub fn getSystemResource(name: String) -> Result<crate::java::net::URL> {
        let _ = name;
        Ok(Default::default())
    }

    /// `getResourceAsStream(String)`：模块资源 → 嵌入字节的 ByteArrayInputStream；其余 → null
    ///（单二进制无 classpath 资源）。name 为 null → NPE（JDK `Objects.requireNonNull`）。
    #[jvm_boundary]
    pub fn getResourceAsStream(&self, name: String) -> Result<crate::java::io::InputStream> {
        module_resource_stream(name)
    }

    /// static `getSystemResourceAsStream(String)`：委托系统加载器（同实例形态）。
    #[jvm_boundary]
    pub fn getSystemResourceAsStream(name: String) -> Result<crate::java::io::InputStream> {
        module_resource_stream(name)
    }

    /// static getSystemResources：委托实例形态（恒空枚举）。
    #[jvm_boundary]
    pub fn getSystemResources(name: String) -> Result<crate::java::util::Enumeration<Object>> {
        crate::java::util::Collections::emptyEnumeration()
    }
}

// ── 运行期类定义点（类 2：运行模型替换）────────────────────────────────────────────────
// 原生二进制没有运行期类定义：类全集在编译期定死，字节码不再被加载执行。defineClass1 / 2 按
// HotSpot `jvm_define_class_common` → `ClassFileParser` 的检查次序给出可在不定义类的前提下判定的
// 结果（截断 / 魔数 / 主次版本，异常类型与消息与 JDK 21 实测一致）；格式合法的类文件无法定义，
// 抛 LinkageError 说明原因（Java 可捕获，不 panic）。登记：vm_intrinsics.toml 注释 / docs/plans/2026-10-02-native-gaps.md。

/// 类文件头检查；返回应抛出的异常。`name` 为调用方给出的类名（null → `<Unknown>`，点换斜线）。
fn _define_class_error(name: &String, bytes: &[u8]) -> JvmError {
    let shown = if _is_jnull(&Object::from(Clone::clone(name))) {
        "<Unknown>".to_owned()
    } else {
        format!("{}", name).replace('.', "/")
    };
    let built = if bytes.len() < 8 {
        crate::java::lang::ClassFormatError::new_str(String::from("Truncated class file")).map(Object::from)
    } else {
        let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let minor = u16::from_be_bytes([bytes[4], bytes[5]]);
        let major = u16::from_be_bytes([bytes[6], bytes[7]]);
        const MAX_MAJOR: u16 = 65; // JDK 21
        if magic != 0xCAFE_BABE {
            crate::java::lang::ClassFormatError::new_str(String::from(
                format!("Incompatible magic value {} in class file {}", magic, shown).as_str())).map(Object::from)
        } else if major > MAX_MAJOR {
            crate::java::lang::UnsupportedClassVersionError::new_str(String::from(format!(
                "{} has been compiled by a more recent version of the Java Runtime (class file version {}.{}), \
                 this version of the Java Runtime only recognizes class file versions up to {}.0",
                shown, major, minor, MAX_MAJOR).as_str())).map(Object::from)
        } else if major < 45 {
            crate::java::lang::UnsupportedClassVersionError::new_str(String::from(format!(
                "{} (class file version {}.{}) was compiled with an invalid major version", shown, major, minor).as_str()))
                .map(Object::from)
        } else if major == MAX_MAJOR && minor == 0xFFFF {
            crate::java::lang::UnsupportedClassVersionError::new_str(String::from(format!(
                "Preview features are not enabled for {} (class file version {}.{}). Try running with '--enable-preview'",
                shown, major, minor).as_str())).map(Object::from)
        } else {
            match _walk_class_file(bytes) {
                ClassFileShape::Truncated => crate::java::lang::ClassFormatError::new_str(
                    String::from("Truncated class file")).map(Object::from),
                ClassFileShape::UnknownTag(tag) => crate::java::lang::ClassFormatError::new_str(String::from(
                    format!("Unknown constant tag {} in class file {}", tag, shown).as_str())).map(Object::from),
                ClassFileShape::ExtraBytes => crate::java::lang::ClassFormatError::new_str(String::from(
                    format!("Extra bytes at the end of class file {}", shown).as_str())).map(Object::from),
                ClassFileShape::Complete => crate::java::lang::LinkageError::new_str(String::from(format!(
                    "{}: runtime class definition is not supported by the native image (class universe is fixed at build time)",
                    shown).as_str())).map(Object::from),
            }
        }
    };
    match built {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// 类文件结构走查结果（只按长度前缀走一遍，不做语义校验）。
enum ClassFileShape { Truncated, UnknownTag(u8), ExtraBytes, Complete }

/// 按 ClassFileParser 的顺序走查常量池、类头、字段、方法与属性表的长度结构：任一处读越界即
/// `Truncated class file`（HotSpot `guarantee_more`），未知常量标签即 `Unknown constant tag`，
/// 走完仍有剩余字节即 `Extra bytes at the end of class file`。
fn _walk_class_file(bytes: &[u8]) -> ClassFileShape {
    struct Cursor<'a> { b: &'a [u8], pos: usize }
    impl Cursor<'_> {
        fn skip(&mut self, n: usize) -> Option<()> {
            (self.b.len() - self.pos >= n).then(|| self.pos += n)
        }
        fn u1(&mut self) -> Option<u8> { self.skip(1).map(|_| self.b[self.pos - 1]) }
        fn u2(&mut self) -> Option<usize> {
            self.skip(2).map(|_| u16::from_be_bytes([self.b[self.pos - 2], self.b[self.pos - 1]]) as usize)
        }
        fn u4(&mut self) -> Option<usize> {
            self.skip(4).map(|_| u32::from_be_bytes(self.b[self.pos - 4..self.pos].try_into().unwrap()) as usize)
        }
        fn attributes(&mut self) -> Option<()> {
            for _ in 0..self.u2()? {
                self.skip(2)?;
                let len = self.u4()?;
                self.skip(len)?;
            }
            Some(())
        }
        fn members(&mut self) -> Option<()> {
            for _ in 0..self.u2()? {
                self.skip(6)?;
                self.attributes()?;
            }
            Some(())
        }
    }
    let mut c = Cursor { b: bytes, pos: 8 };
    let mut walk = || -> std::result::Result<(), ClassFileShape> {
        let cp_count = c.u2().ok_or(ClassFileShape::Truncated)?;
        let mut i = 1;
        while i < cp_count {
            let tag = c.u1().ok_or(ClassFileShape::Truncated)?;
            let fixed = match tag {
                1 => c.u2().ok_or(ClassFileShape::Truncated)?,
                7 | 8 | 16 | 19 | 20 => 2,
                15 => 3,
                3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => 4,
                5 | 6 => { i += 1; 8 }
                other => return Err(ClassFileShape::UnknownTag(other)),
            };
            c.skip(fixed).ok_or(ClassFileShape::Truncated)?;
            i += 1;
        }
        (|| -> Option<()> {
            c.skip(6)?;
            let interfaces = c.u2()?;
            c.skip(interfaces * 2)?;
            c.members()?;
            c.members()?;
            c.attributes()
        })().ok_or(ClassFileShape::Truncated)?;
        if c.pos != c.b.len() { Err(ClassFileShape::ExtraBytes) } else { Ok(()) }
    };
    match walk() {
        Ok(()) => ClassFileShape::Complete,
        Err(shape) => shape,
    }
}

impl ClassLoader {
    /// static native `defineClass1(loader, name, b, off, len, pd, source)`：ClassLoader.c 的
    /// 参数检查（b 为 null → NPE；len < 0 或区间越界 → ArrayIndexOutOfBoundsException）后取区间字节，
    /// 经 `_define_class_error` 判定（见上）。
    #[jvm_native]
    pub fn defineClass1(_loader: ClassLoader, name: String, b: JArray<i8>, off: i32, len: i32,
                        _pd: crate::java::security::ProtectionDomain, _source: String) -> Result<super::Class> {
        if b.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let all = b.to_vec();
        if len < 0 || off < 0 || (off as i64 + len as i64) > all.len() as i64 {
            return Err(JvmError::array_index_out_of_bounds_message(std::string::String::new()));
        }
        let bytes: Vec<u8> = all[off as usize..(off + len) as usize].iter().map(|v| *v as u8).collect();
        Err(_define_class_error(&name, &bytes))
    }

    /// static native `defineClass2(loader, name, b, off, len, pd, source)`：直接缓冲区形态
    /// （ClassLoader.defineClass(ByteBuffer) 只对 direct 缓冲区调用本 native）；地址取 Buffer.address
    /// （GetDirectBufferAddress），为 0 → NPE。
    #[jvm_native]
    pub fn defineClass2(_loader: ClassLoader, name: String, b: crate::java::nio::ByteBuffer, off: i32, len: i32,
                        _pd: crate::java::security::ProtectionDomain, _source: String) -> Result<super::Class> {
        if _is_jnull(&Object::from(Clone::clone(&b))) {
            return Err(JvmError::null_pointer());
        }
        let address = b.__get_address();
        if address == 0 {
            return Err(JvmError::null_pointer());
        }
        let mut bytes = vec![0u8; len.max(0) as usize];
        crate::native_memory::read(&Object::default(), address + off as i64, &mut bytes)?;
        Err(_define_class_error(&name, &bytes))
    }

    /// static native `retrieveDirectives()`：HotSpot `JVM_AssertionStatusDirectives`——按 `-ea` / `-da`
    /// 选项填表。原生二进制无启动选项，断言恒关（与 `Class.desiredAssertionStatus0` 同源）：
    /// 四个数组为空、deflt 为 false。
    #[jvm_native]
    pub fn retrieveDirectives() -> Result<super::AssertionStatusDirectives> {
        let mut d = super::AssertionStatusDirectives::new()?;
        d.__set_classes(JArray::from(Vec::<String>::new()));
        d.__set_classEnabled(JArray::from(Vec::<bool>::new()));
        d.__set_packages(JArray::from(Vec::<String>::new()));
        d.__set_packageEnabled(JArray::from(Vec::<bool>::new()));
        d.__set_deflt(false);
        Ok(d)
    }
}

/// 模块资源名 → 字节流（未命中 → null）。
fn module_resource_stream(name: String) -> Result<crate::java::io::InputStream> {
    if name.is_jvm_null() {
        return Err(JvmError::null_pointer());
    }
    let key = format!("{}", name);
    let Some(bytes) = crate::jdk_resources::module_resources::lookup(&key) else {
        return Ok(Default::default());
    };
    let arr = JArray::from(bytes.iter().map(|b| *b as i8).collect::<Vec<i8>>());
    let stream = crate::java::io::ByteArrayInputStream::new_arr_b(arr)?;
    Ok(<crate::java::io::InputStream as ::std::convert::From<Object>>::from(Object::from(stream)))
}
