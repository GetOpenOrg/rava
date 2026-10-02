use crate::prelude::*;
use super::class_loader::ClassLoader;

// java.lang.ClassLoader 伴生：单二进制运行时的类加载器语义。
//
// 产物是静态链接的 AOT 二进制——类全集编译期定死、无动态 classpath，系统
// 类加载器的角色收敛为「API 契约的恒等对象 + 资源查空的枚举源」：
//   - getSystemClassLoader 返回进程唯一实例（name "app"，与 JDK
//     AppClassLoader 同名；构造不经 JDK 构造器链——ctor 的 parent 解析
//     会回调 getSystemClassLoader 成环，按擦除字段协议直接组装）；
//   - 模块资源（jmod 内数据文件，如 java/util/currency.data）由编译期嵌入表承载：
//     getResourceAsStream 族返回其字节流（jdk_resources::module_resources）；
//   - 其余资源族（getResource/getResources 及 static 形态）恒缺席——
//     ServiceLoader 的 LazyClassPathLookupIterator 据此枚举为空，
//     外部 provider 发现终止（TzdbZoneRulesProvider 已由 ZoneRulesProvider
//     <clinit> 的默认分支直接注册，正是 JDK 对无发现环境的回退设计）。
impl ClassLoader {
    /// native `findBootstrapClass(String name)`：引导加载器按 binary name（点分）查找已定义类，
    /// 找不到返回 null。原生镜像的类全集编译期定死、全部由引导形态承载（`Class.getClassLoader`
    /// 恒 null），故「引导加载器可见」即闭包内的类（与 `Class.forName0` 同一判定）。
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

    /// 系统类加载器单例（JDK: ClassLoader.scl，initSystemClassLoader 填充）。
    #[jvm_boundary]
    pub fn getSystemClassLoader() -> Result<ClassLoader> {
        crate::__process_static! {
            static SCL: ClassLoader = build_system_class_loader();
        }
        Ok(SCL.with(Clone::clone))
    }

    /// static `getClassLoader(Class)`：类的定义加载器（`Class.forName(String)` 按调用方类
    /// 取加载器）。与 `Class.getClassLoader` 同源（FS-C2 分层加载器落地前恒为 null，即
    /// boot 形态）；forName0 不按加载器分派，结果与 JDK 一致。
    #[jvm_boundary]
    pub fn getClassLoader(caller: super::Class) -> Result<ClassLoader> {
        if Object::from(Clone::clone(&caller)).0.is_jvm_null() {
            return Ok(ClassLoader::default());
        }
        caller.getClassLoader()
    }

    /// 父加载器（JDK 层级 app → platform → null）。app 单例的 parent 在
    /// 组装时挂 platform；其余实例（platform/boot 身份对象）parent 为 null
    ///（层级到顶，getParent 返回 null 的 JDK 语义）。
    #[jvm_boundary]
    pub fn getParent(&self) -> Result<ClassLoader> {
        Ok(Clone::clone(&self.__get_parent()))
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

/// 组装系统类加载器（不经 <init>——parent 解析与 scl 初始化成环）。
fn build_system_class_loader() -> ClassLoader {
    let mut scl = ClassLoader::default();
    scl._init_not_null();
    scl.__set_name(String::from("app"));
    scl.__set_parent(crate::jdk::internal::loader::ClassLoaders::platformClassLoader()
        .unwrap_or_default());
    scl
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
