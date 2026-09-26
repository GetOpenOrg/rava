use crate::prelude::*;
use super::class_loader::ClassLoader;

// java.lang.ClassLoader 伴生：单二进制运行时的类加载器语义。
//
// 产物是静态链接的 AOT 二进制——类全集编译期定死、无动态 classpath，系统
// 类加载器的角色收敛为「API 契约的恒等对象 + 资源查空的枚举源」：
//   - getSystemClassLoader 返回进程唯一实例（name "app"，与 JDK
//     AppClassLoader 同名；构造不经 JDK 构造器链——ctor 的 parent 解析
//     会回调 getSystemClassLoader 成环，按擦除字段协议直接组装）；
//   - 资源族（getResource/getResources 及 static 形态）恒缺席——
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
