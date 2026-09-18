use crate::prelude::*;
use super::*;

// 内部边界类 sun.util.locale.BaseLocale：Locale 的 (language, script, region, variant) 四元组。
// struct 由字节码生成；此处按 java.util.Locale 调用链按需实现，其余方法保持 panic 存根。
// 与 JDK 的差异：不做软引用实例缓存（getInstance 每次构造新实例，equals/hashCode 按值比较，
// 语义等价）。

// JDK BaseLocale.<clinit> 中 constantBaseLocales 的内容，下标即 BaseLocale.ENGLISH..ROOT 常量
const CONSTANTS: [(&str, &str); 19] = [
    ("en", ""), ("fr", ""), ("de", ""), ("it", ""), ("ja", ""), ("ko", ""), ("zh", ""),
    ("zh", "CN"), ("zh", "TW"), ("fr", "FR"), ("de", "DE"), ("it", "IT"), ("ja", "JP"),
    ("ko", "KR"), ("en", "GB"), ("en", "US"), ("en", "CA"), ("fr", "CA"), ("", ""),
];

thread_local! {
    static CONSTANT_BASE_LOCALES: std::cell::OnceCell<JArray<BaseLocale>> = std::cell::OnceCell::new();
}

fn make(language: &str, script: &str, region: &str, variant: &str) -> BaseLocale {
    let mut this = BaseLocale::default();
    this._init_not_null();
    this.__set_language(String::from(language));
    this.__set_script(String::from(script));
    this.__set_region(String::from(region));
    this.__set_variant(String::from(variant));
    this
}

fn text(s: &String) -> std::string::String {
    if s.is_jvm_null() { std::string::String::new() } else { s.to_string() }
}

/// 旧 ISO 639 语言码 → 新码（java.locale.useOldISOCodes 默认 false）
fn convert_old_iso_codes(language: &str) -> &str {
    match language {
        "iw" => "he",
        "in" => "id",
        "ji" => "yi",
        other => other,
    }
}

fn title_case(s: &str) -> std::string::String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase(),
        None => std::string::String::new(),
    }
}

impl BaseLocale {
    #[jvm_boundary]
    pub fn constantBaseLocales() -> Result<JArray<BaseLocale>> {
        Ok(CONSTANT_BASE_LOCALES.with(|cell| {
            Clone::clone(cell.get_or_init(|| {
                JArray::from(CONSTANTS.iter()
                    .map(|(language, region)| make(language, "", region, ""))
                    .collect::<Vec<BaseLocale>>())
            }))
        }))
    }

    #[jvm_boundary]
    pub fn getInstance(language: String, script: String, region: String, variant: String) -> Result<BaseLocale> {
        // 与 JDK 一致的规范化：language 小写（并转换旧 ISO 码）、script 首字母大写、region 大写
        let language = text(&language).to_ascii_lowercase();
        Ok(make(
            convert_old_iso_codes(&language),
            &title_case(&text(&script)),
            &text(&region).to_ascii_uppercase(),
            &text(&variant),
        ))
    }

    #[jvm_boundary]
    pub fn convertOldISOCodes(language: String) -> Result<String> {
        let language = text(&language).to_ascii_lowercase();
        Ok(String::from(convert_old_iso_codes(&language)))
    }

    #[jvm_boundary]
    pub fn getLanguage(&self) -> Result<String> { Ok(self.__get_language()) }

    #[jvm_boundary]
    pub fn getScript(&self) -> Result<String> { Ok(self.__get_script()) }

    #[jvm_boundary]
    pub fn getRegion(&self) -> Result<String> { Ok(self.__get_region()) }

    #[jvm_boundary]
    pub fn getVariant(&self) -> Result<String> { Ok(self.__get_variant()) }
}
