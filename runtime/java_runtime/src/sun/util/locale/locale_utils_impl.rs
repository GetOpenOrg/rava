use crate::prelude::*;
use super::*;

// 内部边界类 sun.util.locale.LocaleUtils：ASCII 范围内的大小写 / 字符类判定工具。
// 按 java.util.Locale 调用链按需实现，其余方法保持 panic 存根。

fn text(s: &String) -> std::string::String {
    if s.is_jvm_null() { std::string::String::new() } else { s.to_string() }
}

impl LocaleUtils {
    /// 仅按 ASCII 忽略大小写比较（与 JDK 一致，不受区域设置影响）
    #[jvm_boundary]
    pub fn caseIgnoreMatch(s1: String, s2: String) -> Result<bool> {
        if s1.is_jvm_null() || s2.is_jvm_null() {
            return Ok(s1.is_jvm_null() && s2.is_jvm_null());
        }
        Ok(text(&s1).eq_ignore_ascii_case(&text(&s2)))
    }

    #[jvm_boundary]
    pub fn toLowerString(s: String) -> Result<String> {
        Ok(String::from(text(&s).to_ascii_lowercase().as_str()))
    }

    #[jvm_boundary]
    pub fn toUpperString(s: String) -> Result<String> {
        Ok(String::from(text(&s).to_ascii_uppercase().as_str()))
    }

    #[jvm_boundary]
    pub fn isAlphaNumericString(s: String) -> Result<bool> {
        Ok(text(&s).chars().all(|c| c.is_ascii_alphanumeric()))
    }

    #[jvm_boundary]
    pub fn isEmpty_str(str: String) -> Result<bool> {
        Ok(str.is_jvm_null() || text(&str).is_empty())
    }
}
