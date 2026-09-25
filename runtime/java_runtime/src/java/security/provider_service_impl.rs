//! `java/security/Provider$Service` 手写伴生：边界类（java/security/ 前缀），按调用链按需实现（K-2 规则）。
//!
//! K-JCA：服务描述对象由 `GetInstance.getServices` 按生成注册表（`crate::jca`）构造，字段只填
//! 类型 / 算法 / 实现类名 / provider。`newInstance` 改查注册表的构造闭包（替代按类名反射）。
//! 服务属性（SupportedModes / SupportedPaddings / SupportedKeyFormats）首版不登记：
//! `getAttribute` → null，`Cipher$Transform.supportsModePadding` 据此判 S_MAYBE、按 JDK 语义
//! 仍尝试该服务；`supportsParameter` 恒 true（JDK：无 SupportedKeyClasses/Formats 属性时同值）。

use crate::prelude::*;
use super::provider_service::Provider_Service;
use super::provider::Provider;

impl Provider_Service {
    /// 按注册表条目构造服务描述对象。
    pub fn __from_entry(entry: &crate::jca::ServiceEntry) -> Provider_Service {
        let mut s = Provider_Service::default();
        s._init_not_null();
        s.__set_type_(String::from(entry.type_));
        s.__set_algorithm(String::from(entry.algorithm));
        s.__set_className(String::from(entry.class_name.replace('/', ".").as_str()));
        s.__set_provider(Provider::__for_name(entry.provider));
        s.__set_registered(true);
        s
    }

    pub fn __impl_getType(&self) -> Result<String> {
        Ok(self.__get_type_())
    }

    pub fn __impl_getAlgorithm(&self) -> Result<String> {
        Ok(self.__get_algorithm())
    }

    pub fn __impl_getProvider(&self) -> Result<Provider> {
        Ok(self.__get_provider())
    }

    pub fn __impl_getClassName(&self) -> Result<String> {
        Ok(self.__get_className())
    }

    /// `getAttribute(String)`：属性未登记 → null（见模块说明）。
    pub fn __impl_getAttribute(&self, _name: String) -> Result<String> {
        Ok(String::default())
    }

    /// `newInstance(Object)`：注册表构造闭包（实现类的翻译无参构造器）。
    #[jvm_boundary(upcalls = "java/security/NoSuchAlgorithmException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_newInstance(&self, _constructorParameter: Object) -> Result<Object> {
        let type_ = format!("{}", self.__get_type_());
        let algo = format!("{}", self.__get_algorithm());
        let prov = format!("{}", self.__get_provider().__get_name());
        match crate::jca::find_in(&prov, &type_, &algo) {
            Some(entry) => (entry.ctor)(),
            None => {
                let msg = String::from(format!("Error constructing implementation (algorithm: {}, provider: {}, class: {})",
                                               algo, prov, self.__get_className()).as_str());
                match crate::java::security::NoSuchAlgorithmException::new_str(msg) {
                    Ok(ex) => Err(ex.into()),
                    Err(nested) => Err(nested),
                }
            }
        }
    }

    /// `supportsParameter(Object)`：恒 true（见模块说明）。
    pub fn __impl_supportsParameter(&self, _parameter: Object) -> Result<bool> {
        Ok(true)
    }
}
