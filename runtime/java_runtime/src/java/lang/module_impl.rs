use crate::prelude::*;
use super::module_t::Module;

// java.lang.Module 伴生：单二进制运行时的模块语义。
//
// 产物无模块层（no JPMS runtime）——类全集编译期定死，全部归属**无名模块**
//（JDK 的 ALL-UNNAMED 语义）：isNamed 恒 false，ServiceLoader 的
// LazyClassPathLookupIterator 据此走 classpath 分支（资源枚举为空，
// 外部 provider 发现终止）。无名模块单例挂系统类加载器为 defining loader。

/// 全二进制唯一的无名模块（ALL-UNNAMED 对应物）。
pub fn unnamed_module() -> Module {
    crate::__process_static! {
        static UNNAMED: Module = {
            let mut m = Module::default();
            m._init_not_null();
            m.__set_loader(crate::java::lang::ClassLoader::getSystemClassLoader()
                .unwrap_or_default());
            m
        };
    }
    UNNAMED.with(Clone::clone)
}

impl Module {
    /// 无名模块恒未命名（name 为 null → isNamed false，JDK 语义）。
    #[jvm_boundary]
    pub fn isNamed(&self) -> Result<bool> {
        Ok(!_is_jnull(&Object::from(self.__get_name())))
    }

    /// `Module.canUse(Class service)`：模块是否 uses 该服务。
    /// 无名模块对全部服务恒 uses（JDK Module.addUses/implAddUses 语义的
    /// 无名模块默认——ServiceLoader 据此放行 provider 消费）。
    #[jvm_boundary]
    pub fn canUse(&self, _service: crate::java::lang::Class) -> Result<bool> {
        Ok(true)
    }
}
