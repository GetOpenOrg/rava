use crate::prelude::*;
use super::class_loaders::ClassLoaders;

// jdk.internal.loader.ClassLoaders 伴生：内建类加载器层级（boot → platform
// → app）的访问点。单二进制无动态加载——层级退化为恒等对象：app 加载器
// 即系统类加载器单例（ClassLoader.getSystemClassLoader 同一对象，JDK 的
// scl 就是 AppClassLoader）；platform/boot 为仅供身份比较（MethodHandles
// 的 lookupClass 加载器判定、ServiceLoader 的层级上行终止）的具名实例。
impl ClassLoaders {
    /// app 加载器 = 系统类加载器（同一单例，身份相等语义）。
    #[jvm_boundary]
    pub fn appClassLoader() -> Result<crate::java::lang::ClassLoader> {
        crate::java::lang::ClassLoader::getSystemClassLoader()
    }

    /// platform 加载器：app 之上的具名身份对象（无加载能力）。
    #[jvm_boundary]
    pub fn platformClassLoader() -> Result<crate::java::lang::ClassLoader> {
        Ok(named_loader("platform"))
    }

    /// boot 加载器：层级的 null 形态（JDK BootClassLoader 对上层即 null——
    /// ClassLoader.parent 到顶为 null；此处的具名实例只服务于需要非 null
    /// ClassLoader 值的消费面）。
    #[jvm_boundary]
    pub fn bootLoader() -> Result<crate::java::lang::ClassLoader> {
        Ok(named_loader("boot"))
    }
}

/// 具名空加载器单例（identity 用途，name 恒定）。
fn named_loader(name: &str) -> crate::java::lang::ClassLoader {
    use crate::sync_model::__RefSlot as RefCell;
    crate::__process_static! {
        static PLATFORM: RefCell<std::collections::HashMap<std::string::String, crate::java::lang::ClassLoader>> =
            RefCell::new(std::collections::HashMap::new());
    }
    PLATFORM.with(|m| {
        let mut m = m.borrow_mut();
        m.entry(std::string::String::from(name)).or_insert_with(|| {
            let mut cl = crate::java::lang::ClassLoader::default();
            cl._init_not_null();
            cl.__set_name(String::from(name));
            cl
        }).clone()
    })
}
