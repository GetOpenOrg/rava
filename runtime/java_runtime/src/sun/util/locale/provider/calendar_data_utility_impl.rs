//! `sun/util/locale/provider/CalendarDataUtility` 手写伴生（仅当
//! `calendar_data_utility.rs` 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 的日历名数据链：`LocaleServiceProviderPool.getPool(CalendarNameProvider)`
//! → CLDR 适配器 `CalendarNameProviderImpl` → `LocaleResources` →
//! `sun.util.resources.LocaleData` → **ResourceBundle 类**（jimage 内的
//! `sun/text/resources/cldr/FormatData_*`、`JavaTimeSupplementary_*`、
//! `sun/util/resources/cldr/CalendarData_*`，经模块 ResourceBundleProvider
//! 反射加载）。类形态数据无字节码语料外的独立数据文件——与 tzdb.dat 嵌入
//! 不同形态，按内部边界类规则（3b）以手写数据表承载可观察行为：
//! 名称表（ERA/MONTH/DAY_OF_WEEK/AM_PM × 全部 TextStyle）与周参数表
//! （firstDayOfWeek/minimalDaysInFirstWeek × 全部 ISO 区域）均为 JDK
//! 21.0.11（CLDR 42）经 `CalendarDataUtility` 同名 API 实测提取的金本位。
//!
//! 覆盖面（与语料消费面一致）：语言 `en` 全表 + 其余语言回落 ROOT 表
//! （JDK 资源束父链语义：无专属束的语言回落根束缩写形态）。非 en 语言的
//! 本地化名称（de/fr/ja/zh 等 jdk.localedata 内有专属束）暂以 ROOT 回落，
//! 随消费面扩大按需补表。

use crate::prelude::*;
use super::calendar_data_utility::CalendarDataUtility;
use crate::java::lang::Integer;
use crate::java::util::Locale;
use crate::java::util::HashMap;
use crate::java::util::Map;

// ── Calendar 字段序与 TextStyle→Calendar 样式位（java.util.Calendar 常量）──
const FIELD_ERA: i32 = 0;
const FIELD_MONTH: i32 = 2;
const FIELD_DAY_OF_WEEK: i32 = 7;
const FIELD_AM_PM: i32 = 9;

/// `Calendar.SHORT` 族基样式位（SHORT=1 / LONG=2；standalone 位 0x8000、
/// narrow 位 0x10000 见 `_resolve_names`）。
const STYLE_SHORT: i32 = 1;
const STYLE_LONG: i32 = 2;
const STANDALONE_BIT: i32 = 0x8000;
const NARROW_BIT: i32 = 0x10000;

// ── en 名称表（JDK 21.0.11 CLDR 实测，值域为 Calendar 字段值）──────────────
const MONTHS_EN_LONG: &[(&str, i32)] = &[
    ("January", 0), ("February", 1), ("March", 2), ("April", 3),
    ("May", 4), ("June", 5), ("July", 6), ("August", 7),
    ("September", 8), ("October", 9), ("November", 10), ("December", 11),
];
const MONTHS_SHORT: &[(&str, i32)] = &[
    ("Jan", 0), ("Feb", 1), ("Mar", 2), ("Apr", 3),
    ("May", 4), ("Jun", 5), ("Jul", 6), ("Aug", 7),
    ("Sep", 8), ("Oct", 9), ("Nov", 10), ("Dec", 11),
];
const DAYS_EN_LONG: &[(&str, i32)] = &[
    ("Sunday", 1), ("Monday", 2), ("Tuesday", 3), ("Wednesday", 4),
    ("Thursday", 5), ("Friday", 6), ("Saturday", 7),
];
const DAYS_SHORT: &[(&str, i32)] = &[
    ("Sun", 1), ("Mon", 2), ("Tue", 3), ("Wed", 4),
    ("Thu", 5), ("Fri", 6), ("Sat", 7),
];
const ERA_EN_LONG: &[(&str, i32)] = &[("Before Christ", 0), ("Anno Domini", 1)];
const ERA_EN_SHORT: &[(&str, i32)] = &[("BC", 0), ("AD", 1)];
const ERA_ROOT: &[(&str, i32)] = &[("BCE", 0), ("CE", 1)];
const AM_PM: &[(&str, i32)] = &[("AM", 0), ("PM", 1)];

/// 名称表解析：按 (语言, 字段, 样式, javatime) 归约到金本位表。
/// en：SHORT 族缩写、LONG 族全称、narrow 回落——月份/星期回落全称、纪元回落
/// 缩写（JDK 无 en narrow 专属数据时的回落形态）。ROOT：月份/星期仅缩写
/// （根束无全称），纪元 javatime 短样式为 AD/BC、其余为 BCE/CE。
/// 返回 `None` = 该字段无数据（调用方观察为 null 映射）。
fn _resolve_names(lang: &str, field: i32, style: i32, javatime: bool)
                  -> Option<&'static [(&'static str, i32)]> {
    let narrow = (style & NARROW_BIT) != 0;
    let long = !narrow && (style & !STANDALONE_BIT) == STYLE_LONG;
    let short = !narrow && !long && (style & !STANDALONE_BIT) == STYLE_SHORT;
    if !narrow && !long && !short {
        return None; // 非法样式（JDK getResourceKey 同样查不到键）
    }
    let en = lang == "en";
    match field {
        FIELD_MONTH if en =>
            Some(if long || narrow { MONTHS_EN_LONG } else { MONTHS_SHORT }),
        FIELD_DAY_OF_WEEK if en =>
            Some(if long || narrow { DAYS_EN_LONG } else { DAYS_SHORT }),
        FIELD_ERA if en =>
            Some(if long { ERA_EN_LONG } else { ERA_EN_SHORT }),
        FIELD_AM_PM if en => Some(AM_PM),
        // ROOT 回落（资源束父链终态）
        FIELD_MONTH => Some(MONTHS_SHORT),
        FIELD_DAY_OF_WEEK => Some(DAYS_SHORT),
        FIELD_ERA => Some(if javatime && short { ERA_EN_SHORT } else { ERA_ROOT }),
        FIELD_AM_PM => Some(AM_PM),
        _ => None,
    }
}

/// 语言标签（`Locale.getLanguage`，已小写）。
fn _lang_of(locale: &Locale) -> std::string::String {
    if locale.is_jvm_null() {
        return std::string::String::new();
    }
    match locale.getLanguage() {
        Ok(lang) if !lang.is_jvm_null() => format!("{}", lang),
        _ => std::string::String::new(),
    }
}

/// 周参数查找：区域 → (firstDayOfWeek, minimalDaysInFirstWeek)。
/// 无区域（语言级/ROOT locale）→ 根束值 (1, 1)；未知区域 → (2, 1)
/// （CLDR 001 缺省），与 JDK 对全部 676 个双字母区域实测一致（语言无关）。
/// 表为 JDK 21.0.11 实测中偏离缺省 (2, 1) 的 114 个区域。
fn _week_params(region: &str) -> (i32, i32) {
    const WEEK_TABLE: &[(&str, (i32, i32))] = &[
        ("AD", (2, 4)), ("AE", (7, 1)), ("AF", (7, 1)), ("AG", (1, 1)),
        ("AN", (2, 4)), ("AS", (1, 1)), ("AT", (2, 4)), ("AX", (2, 4)),
        ("BD", (1, 1)), ("BE", (2, 4)), ("BG", (2, 4)), ("BH", (7, 1)),
        ("BR", (1, 1)), ("BS", (1, 1)), ("BT", (1, 1)), ("BW", (1, 1)),
        ("BZ", (1, 1)), ("CA", (1, 1)), ("CH", (2, 4)), ("CO", (1, 1)),
        ("CZ", (2, 4)), ("DE", (2, 4)), ("DJ", (7, 1)), ("DK", (2, 4)),
        ("DM", (1, 1)), ("DO", (1, 1)), ("DZ", (7, 1)), ("EE", (2, 4)),
        ("EG", (7, 1)), ("ES", (2, 4)), ("ET", (1, 1)), ("FI", (2, 4)),
        ("FJ", (2, 4)), ("FO", (2, 4)), ("FR", (2, 4)), ("GB", (2, 4)),
        ("GF", (2, 4)), ("GG", (2, 4)), ("GI", (2, 4)), ("GP", (2, 4)),
        ("GR", (2, 4)), ("GT", (1, 1)), ("GU", (1, 1)), ("HK", (1, 1)),
        ("HN", (1, 1)), ("HU", (2, 4)), ("ID", (1, 1)), ("IE", (2, 4)),
        ("IL", (1, 1)), ("IM", (2, 4)), ("IN", (1, 1)), ("IQ", (7, 1)),
        ("IR", (7, 1)), ("IS", (2, 4)), ("IT", (2, 4)), ("JE", (2, 4)),
        ("JM", (1, 1)), ("JO", (7, 1)), ("JP", (1, 1)), ("KE", (1, 1)),
        ("KH", (1, 1)), ("KR", (1, 1)), ("KW", (7, 1)), ("LA", (1, 1)),
        ("LI", (2, 4)), ("LT", (2, 4)), ("LU", (2, 4)), ("LY", (7, 1)),
        ("MC", (2, 4)), ("MH", (1, 1)), ("MM", (1, 1)), ("MO", (1, 1)),
        ("MQ", (2, 4)), ("MT", (1, 1)), ("MV", (6, 1)), ("MX", (1, 1)),
        ("MZ", (1, 1)), ("NI", (1, 1)), ("NL", (2, 4)), ("NO", (2, 4)),
        ("NP", (1, 1)), ("OM", (7, 1)), ("PA", (1, 1)), ("PE", (1, 1)),
        ("PH", (1, 1)), ("PK", (1, 1)), ("PL", (2, 4)), ("PR", (1, 1)),
        ("PT", (1, 4)), ("PY", (1, 1)), ("QA", (7, 1)), ("RE", (2, 4)),
        ("RU", (2, 4)), ("SA", (1, 1)), ("SD", (7, 1)), ("SE", (2, 4)),
        ("SG", (1, 1)), ("SJ", (2, 4)), ("SK", (2, 4)), ("SM", (2, 4)),
        ("SV", (1, 1)), ("SY", (7, 1)), ("TH", (1, 1)), ("TT", (1, 1)),
        ("TW", (1, 1)), ("UM", (1, 1)), ("US", (1, 1)), ("VA", (2, 4)),
        ("VE", (1, 1)), ("VI", (1, 1)), ("WS", (1, 1)), ("YE", (1, 1)),
        ("ZA", (1, 1)), ("ZW", (1, 1)),
    ];
    if region.is_empty() {
        return (1, 1);
    }
    WEEK_TABLE.iter()
        .find(|(k, _)| *k == region)
        .map(|(_, v)| *v)
        .unwrap_or((2, 1))
}

/// 名称表 → `Map<String, Integer>`（JDK `getDisplayNamesImpl` 的可观察
/// 映射；TreeMap+LengthBasedComparator 的序不进消费契约——java.time 侧
/// 建存时即转 HashMap 按值键化）。无数据字段返回 null 映射。
fn _names_map(lang: &str, field: i32, style: i32, javatime: bool)
              -> Result<Map<Object, Object>> {
    let Some(table) = _resolve_names(lang, field, style, javatime) else {
        return Ok(Default::default());
    };
    let map = HashMap::<Object, Object>::new()?;
    for (name, value) in table {
        let _ = Object::from_any(map.put(
            Object::from(String::from(*name)),
            Object::from(Integer::new_i(*value)?),
        )?);
    }
    Ok(<Map<Object, Object> as ::std::convert::From<_>>::from(Object::from(map)))
}

/// 单名查找：字段值 → 名称（无数据或值越界返回 null）。
fn _field_name(lang: &str, field: i32, value: i32, style: i32, javatime: bool)
               -> Result<String> {
    let Some(table) = _resolve_names(lang, field, style, javatime) else {
        return Ok(Default::default());
    };
    for (name, v) in table {
        if *v == value {
            return Ok(String::from(*name));
        }
    }
    Ok(Default::default())
}

impl CalendarDataUtility {
    /// `retrieveFirstDayOfWeek(Locale)`：`fw` Unicode 扩展（mon..sun → 2..1）
    /// 优先；否则按 `rg` 区域覆盖后的国家查周参数表。
    #[jvm_boundary]
    pub fn retrieveFirstDayOfWeek(locale: Locale) -> Result<i32> {
        if !locale.is_jvm_null() && locale.hasExtensions()? {
            let fw = locale.getUnicodeLocaleType(String::from("fw"))?;
            if !fw.is_jvm_null() {
                let v = match format!("{}", fw).as_str() {
                    "mon" => 2, "tue" => 3, "wed" => 4,
                    "thu" => 5, "fri" => 6, "sat" => 7, "sun" => 1,
                    _ => -1,
                };
                if v > 0 {
                    return Ok(v);
                }
            }
        }
        let effective = Self::findRegionOverride(Clone::clone(&locale))?;
        let region = if effective.is_jvm_null() {
            std::string::String::new()
        } else {
            match effective.getCountry() {
                Ok(r) if !r.is_jvm_null() => format!("{}", r),
                _ => std::string::String::new(),
            }
        };
        let (fdw, _) = _week_params(&region);
        Ok(fdw)
    }

    /// `retrieveMinimalDaysInFirstWeek(Locale)`：按 `rg` 区域覆盖后的国家
    /// 查周参数表。
    #[jvm_boundary]
    pub fn retrieveMinimalDaysInFirstWeek(locale: Locale) -> Result<i32> {
        let effective = Self::findRegionOverride(Clone::clone(&locale))?;
        let region = if effective.is_jvm_null() {
            std::string::String::new()
        } else {
            match effective.getCountry() {
                Ok(r) if !r.is_jvm_null() => format!("{}", r),
                _ => std::string::String::new(),
            }
        };
        let (_, mdifw) = _week_params(&region);
        Ok(mdifw)
    }

    /// `retrieveFieldValueName(String,int,int,int,Locale)`（java.util.Calendar
    /// 侧单名查询，javatime=false）。
    #[jvm_boundary]
    pub fn retrieveFieldValueName(id: String, field: i32, value: i32, style: i32,
                                  locale: Locale) -> Result<String> {
        let _ = id;
        _field_name(&_lang_of(&locale), field, value, style, false)
    }

    /// `retrieveJavaTimeFieldValueName(String,int,int,int,Locale)`（java.time
    /// 侧单名查询，javatime=true）。
    #[jvm_boundary]
    pub fn retrieveJavaTimeFieldValueName(id: String, field: i32, value: i32,
                                          style: i32, locale: Locale) -> Result<String> {
        let _ = id;
        _field_name(&_lang_of(&locale), field, value, style, true)
    }

    /// `retrieveFieldValueNames(String,int,int,Locale)`（javatime=false）。
    #[jvm_boundary]
    pub fn retrieveFieldValueNames(id: String, field: i32, style: i32,
                                   locale: Locale) -> Result<Map<Object, Object>> {
        let _ = id;
        _names_map(&_lang_of(&locale), field, style, false)
    }

    /// `retrieveJavaTimeFieldValueNames(String,int,int,Locale)`（javatime=true）
    /// ——TestDateTimeFormat 的 EEEE/MMMM 文本链消费点。
    #[jvm_boundary]
    pub fn retrieveJavaTimeFieldValueNames(id: String, field: i32, style: i32,
                                           locale: Locale) -> Result<Map<Object, Object>> {
        let _ = id;
        _names_map(&_lang_of(&locale), field, style, true)
    }

    /// `findRegionOverride(Locale)`：`rg` 扩展形如 `xxZZZZ`（xx 两位大写字母）
    /// 时以 xx 重建 locale（区域覆盖，语言/文字/变体/扩展保持），否则原样返回。
    /// 重建走 `Locale.getInstance` 五参工厂（翻译字节码；`Locale$Builder` 在
    /// 消费闭包外，setLocale/build 为存根不可用）。
    #[jvm_boundary]
    pub fn findRegionOverride(l: Locale) -> Result<Locale> {
        let rg = if l.is_jvm_null() {
            String::default()
        } else {
            l.getUnicodeLocaleType(String::from("rg"))?
        };
        if !rg.is_jvm_null() {
            let upper = format!("{}", rg).to_uppercase();
            let bytes = upper.as_bytes();
            if bytes.len() == 6
                && bytes[0].is_ascii_uppercase()
                && bytes[1].is_ascii_uppercase()
                && &upper[2..] == "ZZZZ"
            {
                let base = l.__get_baseLocale();
                return Locale::getInstance_str_str_str_str_localeextensions(
                    base.getLanguage()?,
                    base.getScript()?,
                    String::from(&upper[..2]),
                    base.getVariant()?,
                    l.__get_localeExtensions(),
                );
            }
        }
        Ok(l)
    }
}
