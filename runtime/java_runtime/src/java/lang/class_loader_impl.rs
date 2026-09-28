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
    #[jvm_boundary(upcalls = "java/util/Collections.emptyEnumeration:()Ljava/util/Enumeration;")]
    pub fn getResources(&self, name: String) -> Result<Object> {
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
    #[jvm_boundary(upcalls = "java/io/ByteArrayInputStream.<init>:([B)V")]
    pub fn getResourceAsStream(&self, name: String) -> Result<crate::java::io::InputStream> {
        module_resource_stream(name)
    }

    /// static `getSystemResourceAsStream(String)`：委托系统加载器（同实例形态）。
    #[jvm_boundary(upcalls = "java/io/ByteArrayInputStream.<init>:([B)V")]
    pub fn getSystemResourceAsStream(name: String) -> Result<crate::java::io::InputStream> {
        module_resource_stream(name)
    }

    /// static getSystemResources：委托实例形态（恒空枚举）。
    #[jvm_boundary(upcalls = "java/util/Collections.emptyEnumeration:()Ljava/util/Enumeration;")]
    pub fn getSystemResources(name: String) -> Result<Object> {
        crate::java::util::Collections::emptyEnumeration()
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
