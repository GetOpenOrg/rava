//! `sun/util/locale/provider/LocaleResources` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! L-1（`docs/plans/2026-09-25-l1-cldr-locale-data.md` §六）：数字格式本地化数据改由 CLDR
//! 资源束的**翻译字节码**供给。本类只承担 JDK 链上「装载」一跳的截断——
//! `LocaleData.getNumberFormatData(locale)` 的 `ResourceBundle.getBundle` 类名反射，改为按
//! 候选链（fr_FR → fr → ROOT）查生成注册表（`crate::data_bundles`）实例化翻译束类，并以
//! `setParent` 串成 JDK 同形的父链；键查找、父链回退、`getContents` 数据全部走翻译字节码。
//!
//! `getNumberStrings` / `getDecimalFormatSymbolsData` / `getNumberPatterns` 按 JDK 21/25 字节码
//! 语义（javap 实测两版同形）逐步复刻；JDK 的 SoftReference 缓存以线程内按 locale 缓存束链替代
//! （束构造即 `getContents` 全量字面量数组，逐次重建代价高）。

use crate::prelude::*;
use super::locale_resources::LocaleResources;
use crate::java::util::{Locale, ResourceBundle};
use std::collections::HashMap;

/// ROOT / en 束位于 java.base 包，其余 locale 束位于 jdk.localedata 的 ext 子包。
const FORMAT_DATA_BASE: &str = "sun/text/resources/cldr/FormatData";
const FORMAT_DATA_EXT: &str = "sun/text/resources/cldr/ext/FormatData";

/// CLDR 货币名束（L-2）：ROOT 位于 java.base，其余 locale 束位于 jdk.localedata 的 ext 子包。
const CURRENCY_NAMES_BASE: &str = "sun/util/resources/cldr/CurrencyNames";
const CURRENCY_NAMES_EXT: &str = "sun/util/resources/cldr/ext/CurrencyNames";

crate::__process_static! {
    /// locale 候选键 → 已串好父链的最具体 CurrencyNames 束。
    static CURRENCY_NAMES: crate::sync_model::__RefSlot<HashMap<std::string::String, ResourceBundle>> =
        crate::sync_model::__RefSlot::new(HashMap::new());
}

crate::__process_static! {
    /// locale 候选键（`lang_Script_REGION_variant`）→ 已串好父链的最具体束。
    static NUMBER_FORMAT_DATA: crate::sync_model::__RefSlot<HashMap<std::string::String, ResourceBundle>> =
        crate::sync_model::__RefSlot::new(HashMap::new());
}

/// ResourceBundle 候选后缀（由具体到一般，不含 ROOT）——与闭包分析器 `generator/crates/closure/src/seeds/locale.rs`
/// 的 `parent_chain` 同一顺序（生成侧按它选束，运行侧按它查束）。
fn parent_chain(lang: &str, script: &str, region: &str, variant: &str) -> Vec<std::string::String> {
    let mut out: Vec<std::string::String> = Vec::new();
    if lang.is_empty() {
        return out;
    }
    let mut push = |s: std::string::String| {
        if !out.contains(&s) {
            out.push(s);
        }
    };
    if !script.is_empty() {
        if !region.is_empty() && !variant.is_empty() {
            push(format!("{lang}_{script}_{region}_{variant}"));
        }
        if !region.is_empty() {
            push(format!("{lang}_{script}_{region}"));
        }
        push(format!("{lang}_{script}"));
    }
    if !region.is_empty() && !variant.is_empty() {
        push(format!("{lang}_{region}_{variant}"));
    }
    if !region.is_empty() {
        push(format!("{lang}_{region}"));
    }
    push(lang.to_owned());
    out
}

fn instantiate(name: &str) -> Result<Option<ResourceBundle>> {
    match crate::data_bundles::lookup(name) {
        Some(ctor) => Ok(Some(ctor()?.try_cast::<ResourceBundle>("java/util/ResourceBundle")?)),
        None => Ok(None),
    }
}

impl LocaleResources {
    /// `<init>(ResourceBundleBasedAdapter, Locale)`：记录 locale（数据按其候选链装载）。
    #[jvm_boundary]
    pub fn new(adapter: Object, locale: Locale) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Self::__init_on(this, adapter, locale)
    }

    #[doc(hidden)]
    pub fn __init_on(this: Self, _adapter: Object, locale: Locale) -> Result<Self> {
        this.__set_locale(locale);
        Ok(this)
    }

    /// `LocaleData.getNumberFormatData(locale)` 的截断点：候选链上已登记的束逐个实例化，
    /// 以 `setParent` 由具体到一般串接（ROOT 恒为链尾），返回最具体束。
    #[doc(hidden)]
    pub fn __number_format_data(&self) -> Result<ResourceBundle> {
        self.__bundle_chain(FORMAT_DATA_BASE, FORMAT_DATA_EXT, &NUMBER_FORMAT_DATA)
    }

    /// 候选链上已登记的 `<base>_<后缀>` / `<ext>_<后缀>` 束逐个实例化，以 `setParent` 由具体到
    /// 一般串接（ROOT 束 `<base>` 恒为链尾），按 locale 候选键缓存，返回最具体束
    ///（`LocaleData.getBundle` 的截断点；键查找与父链回退走翻译字节码）。
    fn __bundle_chain(&self, base_name: &str, ext_name: &str,
                      cache: &'static crate::sync_model::__GilStatic<crate::sync_model::__RefSlot<HashMap<std::string::String, ResourceBundle>>>)
                      -> Result<ResourceBundle> {
        let base = self.__get_locale().__get_baseLocale();
        let (lang, script, region, variant) = if base.is_jvm_null() {
            Default::default()
        } else {
            (format!("{}", base.__get_language()), format!("{}", base.__get_script()),
             format!("{}", base.__get_region()), format!("{}", base.__get_variant()))
        };
        let key = format!("{lang}_{script}_{region}_{variant}");
        if let Some(rb) = cache.with(|c| c.borrow().get(&key).map(Clone::clone)) {
            return Ok(rb);
        }
        let mut chain: Vec<ResourceBundle> = Vec::new();
        for suffix in parent_chain(&lang, &script, &region, &variant) {
            for pkg in [base_name, ext_name] {
                if let Some(rb) = instantiate(&format!("{pkg}_{suffix}"))? {
                    chain.push(rb);
                    break;
                }
            }
        }
        match instantiate(base_name)? {
            Some(root) => chain.push(root),
            None => panic!("stub: data bundle {base_name} not registered (locale seeds)"),
        }
        for i in 0..chain.len() - 1 {
            chain[i].setParent(Clone::clone(&chain[i + 1]))?;
        }
        let rb = Clone::clone(&chain[0]);
        cache.with(|c| c.borrow_mut().insert(key, Clone::clone(&rb)));
        Ok(rb)
    }

    /// private `getNumberStrings(ResourceBundle, String)`（字节码语义；仅手写体内调用，
    /// 回调边声明在两个公开入口上）：
    /// `<nu 扩展>.<type>` → `<DefaultNumberingSystem>.<type>` → `<type>`，首个存在的键胜出。
    #[doc(hidden)]
    pub fn getNumberStrings(&self, rb: ResourceBundle, type_: String) -> Result<JArray<String>> {
        let num_sys = self.__get_locale().getUnicodeLocaleType(String::from("nu"))?;
        if !num_sys.is_jvm_null() {
            let key = String::from_owned(format!("{num_sys}.{type_}"));
            if rb.containsKey(Clone::clone(&key))? {
                return rb.getStringArray(key);
            }
        }
        if rb.containsKey(String::from("DefaultNumberingSystem"))? {
            let default_sys = rb.getString(String::from("DefaultNumberingSystem"))?;
            let key = String::from_owned(format!("{default_sys}.{type_}"));
            if rb.containsKey(Clone::clone(&key))? {
                return rb.getStringArray(key);
            }
        }
        rb.getStringArray(type_)
    }

    /// `getDecimalFormatSymbolsData()`：`Object[3]`，[0] = NumberElements；[1]/[2] 为
    /// DecimalFormatSymbols 的货币缓存槽（JDK 由 initializeCurrency 回填，初值 null）。
    ///
    /// 回调边（手写体 → 翻译层）：束链串接与键查找的 ResourceBundle / Locale 成员。
    #[jvm_boundary(upcalls = "java/util/ResourceBundle.setParent:(Ljava/util/ResourceBundle;)V java/util/Locale.getUnicodeLocaleType:(Ljava/lang/String;)Ljava/lang/String; java/util/ResourceBundle.containsKey:(Ljava/lang/String;)Z java/util/ResourceBundle.getStringArray:(Ljava/lang/String;)[Ljava/lang/String; java/util/ResourceBundle.getString:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn __impl_getDecimalFormatSymbolsData(&self) -> Result<JArray<Object>> {
        let rb = self.__number_format_data()?;
        let data: JArray<Object> = JArray::new(3);
        data.set(0, Object::from(self.getNumberStrings(rb, String::from("NumberElements"))?))?;
        Ok(data)
    }

    /// `getCurrencyName(String key)`（L-2）：CurrencyNames 束链上 `containsKey(key)` 则取值，否则
    /// null——键为大写货币代码（符号）或小写货币代码（显示名），与 JDK `CurrencyNameProviderImpl`
    /// 的取键约定一致。束链串接同 `getNumberFormatData`，查表 / 父链回退走翻译字节码。
    #[jvm_boundary(upcalls = "java/util/ResourceBundle.setParent:(Ljava/util/ResourceBundle;)V java/util/ResourceBundle.containsKey:(Ljava/lang/String;)Z java/util/ResourceBundle.getString:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn __impl_getCurrencyName(&self, key: String) -> Result<String> {
        let rb = self.__bundle_chain(CURRENCY_NAMES_BASE, CURRENCY_NAMES_EXT, &CURRENCY_NAMES)?;
        if rb.containsKey(Clone::clone(&key))? {
            return rb.getString(key);
        }
        Ok(String::default())
    }

    /// `getNumberPatterns()`：NumberPatterns（number / currency / percent / accounting）。
    #[jvm_boundary(upcalls = "java/util/ResourceBundle.setParent:(Ljava/util/ResourceBundle;)V java/util/Locale.getUnicodeLocaleType:(Ljava/lang/String;)Ljava/lang/String; java/util/ResourceBundle.containsKey:(Ljava/lang/String;)Z java/util/ResourceBundle.getStringArray:(Ljava/lang/String;)[Ljava/lang/String; java/util/ResourceBundle.getString:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn __impl_getNumberPatterns(&self) -> Result<JArray<String>> {
        let rb = self.__number_format_data()?;
        self.getNumberStrings(rb, String::from("NumberPatterns"))
    }

    /// `getCNPatterns(NumberFormat.Style)`：`<short|long>.CompactNumberPatterns`（FormatData 束链，
    /// 字节码语义：LONG → "long"，其余 → "short"；键经 getObject 取 String[]）。
    #[jvm_boundary(upcalls = "java/util/ResourceBundle.setParent:(Ljava/util/ResourceBundle;)V java/util/ResourceBundle.getStringArray:(Ljava/lang/String;)[Ljava/lang/String; java/lang/Enum.name:()Ljava/lang/String;")]
    pub fn __impl_getCNPatterns(&self, style: crate::java::text::NumberFormat_Style) -> Result<JArray<String>> {
        let prefix = if format!("{}", style.name()?) == "LONG" { "long" } else { "short" };
        let rb = self.__number_format_data()?;
        rb.getStringArray(String::from_owned(format!("{prefix}.CompactNumberPatterns")))
    }

    /// `getRules()`：`String[2]` = [PluralRules, DayPeriodRules]（FormatData 束链——CLDR 的
    /// DateFormatData 与 NumberFormatData 同为 FormatData 束；键缺席取空串，字节码语义）。
    #[jvm_boundary(upcalls = "java/util/ResourceBundle.setParent:(Ljava/util/ResourceBundle;)V java/util/ResourceBundle.containsKey:(Ljava/lang/String;)Z java/util/ResourceBundle.getString:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn __impl_getRules(&self) -> Result<JArray<String>> {
        let rb = self.__number_format_data()?;
        let rules: JArray<String> = JArray::new(2);
        for (i, key) in ["PluralRules", "DayPeriodRules"].into_iter().enumerate() {
            let v = if rb.containsKey(String::from(key))? {
                rb.getString(String::from(key))?
            } else {
                String::from("")
            };
            rules.set(i as i32, v)?;
        }
        Ok(rules)
    }

    /// `getDateTimePattern(int timeStyle, int dateStyle, Calendar cal)`（字节码语义）：
    /// 历法类型取 `cal.getCalendarType()`，按 TimePatterns / DatePatterns / DateTimePatterns
    /// 组装；模式缺席返回 null。
    #[jvm_boundary(upcalls = "java/util/Calendar.getCalendarType:()Ljava/lang/String; java/util/ResourceBundle.setParent:(Ljava/util/ResourceBundle;)V java/util/ResourceBundle.containsKey:(Ljava/lang/String;)Z java/util/ResourceBundle.getStringArray:(Ljava/lang/String;)[Ljava/lang/String;")]
    pub fn __impl_getDateTimePattern_i_i_calendar(&self, time_style: i32, date_style: i32,
                                                  cal: crate::java::util::Calendar) -> Result<String> {
        let cal_type = format!("{}", cal.getCalendarType()?);
        Ok(match self.__date_time_pattern(time_style, date_style, &cal_type)? {
            Some(p) => String::from_owned(p),
            None => String::default(),
        })
    }

    /// private `getDateTimePattern(String prefix=null, int, int, String calType)`：
    /// 时间 / 日期模式各取样式下标；二者皆有时按 DateTimePatterns[max(dateStyle, timeStyle)]
    /// 组合——`{1} {0}` / `{0} {1}` 直接拼接，其余经 MessageFormat（引号加倍后格式化，
    /// 等价于在原模式上以 {0}=时间、{1}=日期 原样替换）。
    #[doc(hidden)]
    pub fn __date_time_pattern(&self, time_style: i32, date_style: i32, cal_type: &str)
                               -> Result<Option<std::string::String>> {
        let time = if time_style >= 0 { self.__dtp_entry("TimePatterns", time_style, cal_type)? } else { None };
        let date = if date_style >= 0 { self.__dtp_entry("DatePatterns", date_style, cal_type)? } else { None };
        if time_style >= 0 {
            if date_style >= 0 {
                let dtp = self.__dtp_entry("DateTimePatterns", date_style.max(time_style), cal_type)?
                    .ok_or_else(JvmError::null_pointer)?;
                let (t, d) = (time.unwrap_or_else(|| "null".into()), date.unwrap_or_else(|| "null".into()));
                return Ok(Some(match dtp.as_str() {
                    "{1} {0}" => format!("{d} {t}"),
                    "{0} {1}" => format!("{t} {d}"),
                    _ => dtp.replace("{0}", "\u{0}").replace("{1}", &d).replace("\u{0}", &t),
                }));
            }
            return Ok(time);
        }
        if date_style >= 0 {
            return Ok(date);
        }
        Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
            String::from("No date or time style specified"))?))
    }

    /// private `getDateTimePattern(prefix, key, styleIndex, calendarType)`：非 gregory 历法键加
    /// `<calType>.` 前缀、缺席回落裸键；数组长度 > 1 取下标，否则取 [0]。
    fn __dtp_entry(&self, key: &str, style_index: i32, cal_type: &str) -> Result<Option<std::string::String>> {
        let rb = self.__number_format_data()?;
        let resource_key = if cal_type == "gregory" { key.to_owned() } else { format!("{cal_type}.{key}") };
        let patterns = if rb.containsKey(String::from(resource_key.as_str()))? {
            rb.getStringArray(String::from(resource_key.as_str()))?
        } else if rb.containsKey(String::from(key))? {
            rb.getStringArray(String::from(key))?
        } else {
            return Ok(None);
        };
        let idx = if patterns.len()? > 1 { style_index } else { 0 };
        Ok(Some(format!("{}", patterns.get(idx)?)))
    }
}
