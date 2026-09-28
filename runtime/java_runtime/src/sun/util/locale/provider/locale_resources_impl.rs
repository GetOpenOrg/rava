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

/// ResourceBundle 候选后缀（由具体到一般，不含 ROOT）——与 `codegen/locale_seed.py`
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
}
