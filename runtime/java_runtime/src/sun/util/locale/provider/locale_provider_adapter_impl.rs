//! `sun/util/locale/provider/LocaleProviderAdapter` 手写伴生（仅当
//! `locale_provider_adapter.rs` 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 的适配器链（`getAdapterPreference` = [CLDR, FALLBACK] → `forType` 经
//! `Class.forName` 实例化 `sun.util.cldr.CLDRLocaleProviderAdapter` →
//! provider 经 `LocaleServiceProviderPool` 查询 `isSupportedLocale`）的本地化
//! 数据全部来自 CLDR 资源束（jdk.localedata 模块，非字节码语料）。本文件按
//! 内部边界类规则（3b）以手写数据承载该链的可观察行为：
//! - 适配器为单例 `NativeLocaleAdapter`（运行时类名按 JDK 的 CLDR 适配器，
//!   `getAdapter`/`forJRE`/`getResourceBundleBased`/`forType` 都返回它——
//!   JDK 语义的退化态：唯一适配器即 FALLBACK 终态，locale 由符号/模式表
//!   区分，不再逐 adapter 判定）；
//! - `NumberFormatProvider` 四实例族按 CLDR 42 模式表（JDK 21.0.11 实测核对）
//!   构造 `DecimalFormat`（翻译字节码），符号集来自
//!   `DecimalFormatSymbols::getInstance_locale` 的语料 locale 表。

use crate::prelude::*;
use super::locale_provider_adapter::{LocaleProviderAdapter, LocaleProviderAdapter__VTable};
use super::locale_provider_adapter_type::LocaleProviderAdapter_Type;
use crate::java::lang::Class;
use crate::java::text::spi::NumberFormatProvider;
use crate::java::text::spi::NumberFormatProvider__VTable;
use crate::java::util::spi::LocaleServiceProvider__VTable;
use crate::java::text::{DecimalFormat, DecimalFormatSymbols, NumberFormat};
use crate::java::util::Locale;
use std::cell::RefCell;

/// `java/text/spi/NumberFormatProvider` 的手写实现对象（JDK 的
/// CLDRLocaleProviderAdapter 匿名 NumberFormatProvider）：按 CLDR 42 模式表
/// 构造 DecimalFormat。模式与 JDK 21.0.11 实测一致：
/// ROOT/en：number `#,##0.###`、currency `¤#,##0.00`、percent `#,##0%`、
/// integer `#,##0`；de：number `#,##0.###`、currency `#,##0.00 ¤`、
/// percent `#,##0 %`、integer `#,##0`。
struct NativeNumberFormatProvider;

fn _de_locale(locale: &Locale) -> bool {
    if locale.is_jvm_null() {
        return false;
    }
    let base = locale.__get_baseLocale();
    if base.is_jvm_null() {
        return false;
    }
    format!("{}", base.__get_language()) == "de"
}

impl NativeNumberFormatProvider {
    fn new_format(locale: Locale, pattern_root: &str, pattern_de: &str)
                  -> Result<NumberFormat> {
        let pattern = if _de_locale(&locale) { pattern_de } else { pattern_root };
        let symbols = DecimalFormatSymbols::getInstance_locale(Clone::clone(&locale))?;
        let df = DecimalFormat::new_str_decimalformatsymbols(String::from(pattern), symbols)?;
        Ok(<NumberFormat as ::std::convert::From<DecimalFormat>>::from(df))
    }
}

impl NumberFormatProvider__VTable for NativeNumberFormatProvider {
    fn getNumberInstance(&self, arg0: Locale) -> Result<NumberFormat> {
        Self::new_format(arg0, "#,##0.###", "#,##0.###")
    }

    fn getIntegerInstance(&self, arg0: Locale) -> Result<NumberFormat> {
        Self::new_format(arg0, "#,##0", "#,##0")
    }

    fn getCurrencyInstance(&self, arg0: Locale) -> Result<NumberFormat> {
        Self::new_format(arg0, "\u{A4}#,##0.00", "#,##0.00 \u{A4}")
    }

    fn getPercentInstance(&self, arg0: Locale) -> Result<NumberFormat> {
        Self::new_format(arg0, "#,##0%", "#,##0 %")
    }

    /// 祖先 wrapper 重建钩子（vtable trait 的必备条目，与宏为 __inner 生成的
    /// 形态一致）：以自身部件重建 NumberFormatProvider 视图。
    fn __as_NumberFormatProvider(&self) -> NumberFormatProvider {
        let rc = Rc::new(NativeNumberFormatProvider);
        NumberFormatProvider::__from_parts(
            Rc::clone(&rc) as Rc<dyn NumberFormatProvider__VTable>,
            rc as Rc<dyn std::any::Any>,
            false,
        )
    }
}

// 超类型链：LocaleServiceProvider__VTable 的方法（getAvailableLocales /
// isSupportedLocale 等）自带默认 panic 体（语料未消费）；祖先视图重建钩子必备。
impl LocaleServiceProvider__VTable for NativeNumberFormatProvider {
    fn __as_LocaleServiceProvider(&self) -> crate::java::util::spi::LocaleServiceProvider {
        let rc = Rc::new(NativeNumberFormatProvider);
        crate::java::util::spi::LocaleServiceProvider::__from_parts(
            Rc::clone(&rc) as Rc<dyn LocaleServiceProvider__VTable>,
            rc as Rc<dyn std::any::Any>,
            false,
        )
    }
}

impl ObjectVTable for NativeNumberFormatProvider {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str { "sun/util/cldr/CLDRLocaleProviderAdapter$1" }
    fn __obj_str(&self) -> std::string::String {
        "sun.util.cldr.CLDRLocaleProviderAdapter$1".to_owned()
    }
}

/// `LocaleProviderAdapter` 的手写实现对象（JDK 的 CLDR 适配器单例形态）。
struct NativeLocaleAdapter;

impl LocaleProviderAdapter__VTable for NativeLocaleAdapter {
    fn getNumberFormatProvider(&self) -> Result<NumberFormatProvider> {
        _provider_view()
    }

    /// 祖先 wrapper 重建钩子（vtable trait 必备条目，与宏形态一致）：返回
    /// 共享部件的适配器视图（线程内单例语义见 `_adapter_view`，此处按宏的
    /// 逐次重建形态）。
    fn __as_LocaleProviderAdapter(&self) -> LocaleProviderAdapter {
        _adapter_view()
    }
}

impl ObjectVTable for NativeLocaleAdapter {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str { "sun/util/cldr/CLDRLocaleProviderAdapter" }
    fn __obj_str(&self) -> std::string::String {
        "sun.util.cldr.CLDRLocaleProviderAdapter".to_owned()
    }
}

/// 适配器单例视图（线程内唯一——JDK 的 adapterInstances 缓存语义，身份稳定）。
fn _adapter_view() -> LocaleProviderAdapter {
    thread_local! {
        static ADAPTER: LocaleProviderAdapter = {
            let rc = Rc::new(NativeLocaleAdapter);
            LocaleProviderAdapter::__from_parts(
                Rc::clone(&rc) as Rc<dyn LocaleProviderAdapter__VTable>,
                rc as Rc<dyn std::any::Any>,
                false,
            )
        };
    }
    ADAPTER.with(Clone::clone)
}

/// NumberFormatProvider 单例视图（JDK 的 CLDR 适配器在基类构造时创建并缓存）。
fn _provider_view() -> Result<NumberFormatProvider> {
    thread_local! {
        static PROVIDER: RefCell<Option<NumberFormatProvider>> = const { RefCell::new(None) };
    }
    PROVIDER.with(|slot| {
        if slot.borrow().is_none() {
            let rc = Rc::new(NativeNumberFormatProvider);
            *slot.borrow_mut() = Some(NumberFormatProvider::__from_parts(
                Rc::clone(&rc) as Rc<dyn NumberFormatProvider__VTable>,
                rc as Rc<dyn std::any::Any>,
                false,
            ));
        }
        Ok(Clone::clone(slot.borrow().as_ref().unwrap()))
    })
}

impl LocaleProviderAdapter {
    /// `getAdapter(Class, Locale)`：JDK 按 adapterPreference 与 provider 的
    /// isSupportedLocale 逐级挑选，终态 FALLBACK。原生侧唯一数据适配器即终态
    /// ——直接返回（CLDR 子集数据在 provider/符号表层按 locale 区分）。
    #[jvm_boundary]
    pub fn getAdapter(_providerClass: Class, _locale: Locale) -> Result<LocaleProviderAdapter> {
        Ok(_adapter_view())
    }

    /// `forJRE()`：JDK 的 JRE 适配器（COMPAT）。原生侧同一数据适配器承担。
    #[jvm_boundary]
    pub fn forJRE() -> Result<LocaleProviderAdapter> {
        Ok(_adapter_view())
    }

    /// `getResourceBundleBased()`：JDK 在 JRE/CLDR/FALLBACK 里取资源束系适配器。
    /// 原生侧同一数据适配器承担。
    #[jvm_boundary]
    pub fn getResourceBundleBased() -> Result<LocaleProviderAdapter> {
        Ok(_adapter_view())
    }

    /// `forType(Type)`：JDK 按类型实例化（CLDR 走 Class.forName 反射）。
    /// 原生侧各类型返回同一数据适配器（唯一适配器的退化态）。
    #[jvm_boundary]
    pub fn forType(_type: LocaleProviderAdapter_Type) -> Result<LocaleProviderAdapter> {
        let _ = _type;
        Ok(_adapter_view())
    }
}
