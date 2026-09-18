use crate::prelude::*;
use super::static_property::StaticProperty;

// 内部边界类 jdk.internal.util.StaticProperty：VM 启动时快照的系统属性。
// 按 java.util.Locale 调用链按需实现默认区域相关属性，其余保持 panic 存根。
// 取值与 JDK 的 System.initPhase1 一致：user.language / user.country 来自进程环境
// （LC_ALL / LC_MESSAGES / LANG，形如 `en_US.UTF-8`），缺省为 en / 空；
// *_DISPLAY / *_FORMAT 未单独设置时回落到基础属性。

fn posix_locale() -> (std::string::String, std::string::String) {
    let raw = ["LC_ALL", "LC_MESSAGES", "LANG"].iter()
        .filter_map(|name| std::env::var(name).ok())
        .find(|value| !value.is_empty())
        .unwrap_or_default();
    let tag = raw.split(['.', '@']).next().unwrap_or("");
    if tag.is_empty() || tag == "C" || tag == "POSIX" {
        return ("en".to_owned(), std::string::String::new());
    }
    let mut parts = tag.splitn(2, '_');
    let language = parts.next().unwrap_or("en").to_owned();
    let country = parts.next().unwrap_or("").to_owned();
    (language, country)
}

impl StaticProperty {
    #[jvm_boundary]
    pub fn USER_LANGUAGE() -> Result<String> { Ok(String::from(posix_locale().0.as_str())) }
    #[jvm_boundary]
    pub fn USER_LANGUAGE_DISPLAY() -> Result<String> { Self::USER_LANGUAGE() }
    #[jvm_boundary]
    pub fn USER_LANGUAGE_FORMAT() -> Result<String> { Self::USER_LANGUAGE() }

    #[jvm_boundary]
    pub fn USER_SCRIPT() -> Result<String> { Ok(String::from("")) }
    #[jvm_boundary]
    pub fn USER_SCRIPT_DISPLAY() -> Result<String> { Self::USER_SCRIPT() }
    #[jvm_boundary]
    pub fn USER_SCRIPT_FORMAT() -> Result<String> { Self::USER_SCRIPT() }

    #[jvm_boundary]
    pub fn USER_COUNTRY() -> Result<String> { Ok(String::from(posix_locale().1.as_str())) }
    #[jvm_boundary]
    pub fn USER_COUNTRY_DISPLAY() -> Result<String> { Self::USER_COUNTRY() }
    #[jvm_boundary]
    pub fn USER_COUNTRY_FORMAT() -> Result<String> { Self::USER_COUNTRY() }

    #[jvm_boundary]
    pub fn USER_VARIANT() -> Result<String> { Ok(String::from("")) }
    #[jvm_boundary]
    pub fn USER_VARIANT_DISPLAY() -> Result<String> { Self::USER_VARIANT() }
    #[jvm_boundary]
    pub fn USER_VARIANT_FORMAT() -> Result<String> { Self::USER_VARIANT() }

    #[jvm_boundary]
    pub fn USER_EXTENSIONS() -> Result<String> { Ok(String::from("")) }
    #[jvm_boundary]
    pub fn USER_EXTENSIONS_DISPLAY() -> Result<String> { Self::USER_EXTENSIONS() }
    #[jvm_boundary]
    pub fn USER_EXTENSIONS_FORMAT() -> Result<String> { Self::USER_EXTENSIONS() }
}
