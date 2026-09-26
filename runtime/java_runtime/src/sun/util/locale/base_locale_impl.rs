use crate::prelude::*;
use super::*;

// 内部边界类 sun.util.locale.BaseLocale：Locale 的 (language, script, region, variant) 四元组。
// struct 由字节码生成；此处按 java.util.Locale 调用链按需实现，其余方法保持 panic 存根。
// 与 JDK 的差异：不做软引用实例缓存（getInstance 每次构造新实例，equals/hashCode 按值比较，
// 语义等价）。

// JDK BaseLocale.<clinit> 中 constantBaseLocales 的内容，下标即 Locale.createConstant(byte)
// 的实参（Locale.<clinit> 翻译自字节码，按所在 JDK 的下标取用）。**下标顺序随 JDK 版本
// 变化**（javap 实测）：JDK 21 以 ENGLISH=0 起、ROOT 居末；JDK 25 以 ROOT=0 起、US=2、
// GERMANY=12。按语料版本（crate::jdk_feature()）选表——错表会让 Locale.US 拿到 de 的
// 格式（TestFormatLocale 实证）。22–24 未取语料核对，按 25 的新顺序处理。
const CONSTANTS_JDK21: [(&str, &str); 19] = [
    ("en", ""), ("fr", ""), ("de", ""), ("it", ""), ("ja", ""), ("ko", ""), ("zh", ""),
    ("zh", "CN"), ("zh", "TW"), ("fr", "FR"), ("de", "DE"), ("it", "IT"), ("ja", "JP"),
    ("ko", "KR"), ("en", "GB"), ("en", "US"), ("en", "CA"), ("fr", "CA"), ("", ""),
];

// JDK 25：javap -c sun.util.locale.BaseLocale <clinit> 的 aastore 下标序
const CONSTANTS_JDK25: [(&str, &str); 19] = [
    ("", ""), ("en", ""), ("en", "US"), ("fr", ""), ("de", ""), ("it", ""), ("ja", ""),
    ("ko", ""), ("zh", ""), ("zh", "CN"), ("zh", "TW"), ("fr", "FR"), ("de", "DE"),
    ("it", "IT"), ("ja", "JP"), ("ko", "KR"), ("en", "GB"), ("en", "CA"), ("fr", "CA"),
];

fn constants() -> &'static [(&'static str, &'static str); 19] {
    if crate::jdk_feature() >= 22 { &CONSTANTS_JDK25 } else { &CONSTANTS_JDK21 }
}

crate::__process_static! {
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
                JArray::from(constants().iter()
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

    /// BaseLocale.hashCode()I 的方法体（虚方法：声明在生成的宏块内，经 vtable 分派到此）。
    /// 与 JDK 一致：h = language.hashCode(); h = 31*h + script/region/variant.hashCode()
    #[jvm_boundary]
    pub fn __impl_hashCode(&self) -> Result<i32> {
        let mut h: i32 = 0;
        for part in [self.__get_language(), self.__get_script(), self.__get_region(), self.__get_variant()] {
            h = h.wrapping_mul(31).wrapping_add(string_hash(&text(&part)));
        }
        Ok(h)
    }

    /// BaseLocale.equals(Object)Z 的方法体：四元组按值比较
    #[jvm_boundary]
    pub fn __impl_equals(&self, obj: Object) -> Result<bool> {
        if !obj.is_instance_of(Self::BINARY_NAME) {
            return Ok(false);
        }
        let other = BaseLocale::from(obj);
        Ok(text(&self.__get_language()) == text(&other.__get_language())
            && text(&self.__get_script()) == text(&other.__get_script())
            && text(&self.__get_region()) == text(&other.__get_region())
            && text(&self.__get_variant()) == text(&other.__get_variant()))
    }
}

/// java.lang.String.hashCode 的取值：s[0]*31^(n-1) + ... + s[n-1]（UTF-16 码元）
fn string_hash(s: &str) -> i32 {
    s.encode_utf16().fold(0i32, |h, unit| h.wrapping_mul(31).wrapping_add(unit as i32))
}
