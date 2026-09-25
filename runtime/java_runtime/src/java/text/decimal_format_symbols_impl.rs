//! `java/text/DecimalFormatSymbols` 手写实现（仅当 `decimal_format_symbols.rs`
//! 进入闭包生成时编译，见 K-2 规则）。
//!
//! L-1：`getInstance(Locale)` / 构造器 / `initialize(Locale)` 全部走翻译字节码——符号集
//! 数据来自 `LocaleResources.getDecimalFormatSymbolsData()`（CLDR 资源束翻译字节码，
//! 见 `sun/util/locale/provider/locale_resources_impl.rs`）。本文件只剩货币一项：
//!
//! `initializeCurrency(Locale)` 在 JDK 中经 `Currency.getInstance(locale)`（currency.data
//! 二进制资源，非字节码语料）与 `Currency.getSymbol(locale)`（CurrencyNames 资源束）
//! 取货币；Currency 数据层不在 L-1 范围，此处按地区码给出语料所及的货币代码 / 本地符号
//! （JDK 21/25 实测核对），其余地区与无地区 locale 按 JDK 的无货币分支给 `XXX` / `¤`。
//! `currency` 字段不建模（`getCurrency()` 为 null——compatibility.md 偏差入账）。

use crate::prelude::*;
use crate::java::util::Locale;
use super::decimal_format_symbols::DecimalFormatSymbols;

/// 地区码 → (ISO 4217 代码, 按语言选择的本地符号)。
fn currency_for(language: &str, region: &str) -> (&'static str, &'static str) {
    match region {
        "US" => ("USD", "$"),
        "CA" => ("CAD", "$"),
        "GB" => ("GBP", "\u{A3}"),
        "DE" | "FR" | "IT" | "ES" | "NL" | "AT" | "BE" | "FI" | "IE" | "PT" | "GR" | "LU" =>
            ("EUR", "\u{20AC}"),
        "JP" => ("JPY", if language == "ja" { "\u{FFE5}" } else { "\u{A5}" }),
        "CN" => ("CNY", if language == "zh" { "\u{A5}" } else { "CN\u{A5}" }),
        "KR" => ("KRW", "\u{20A9}"),
        "TW" => ("TWD", if language == "zh" { "$" } else { "NT$" }),
        _ => ("XXX", "\u{A4}"),
    }
}

impl DecimalFormatSymbols {
    /// private `initializeCurrency(Locale)`：幂等（`currencyInitialized`），按地区填
    /// `intlCurrencySymbol` / `currencySymbol`（见模块说明）。
    #[jvm_boundary]
    pub fn initializeCurrency(&self, locale: Locale) -> Result<()> {
        if self.__get_currencyInitialized() {
            return Ok(());
        }
        let base = if locale.is_jvm_null() { None } else { Some(locale.__get_baseLocale()) };
        let (language, region) = match base {
            Some(b) if !b.is_jvm_null() => (format!("{}", b.__get_language()), format!("{}", b.__get_region())),
            _ => Default::default(),
        };
        let (code, symbol) = currency_for(&language, &region);
        self.__set_intlCurrencySymbol(String::from(code));
        self.__set_currencySymbol(String::from(symbol));
        self.__set_currencyInitialized(true);
        Ok(())
    }
}
