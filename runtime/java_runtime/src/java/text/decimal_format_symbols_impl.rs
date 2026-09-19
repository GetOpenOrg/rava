//! `java/text/DecimalFormatSymbols` 手写实现（仅当 `decimal_format_symbols.rs`
//! 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 的 `getInstance(Locale)` 经 `sun.util.locale.provider` SPI（HostLocaleProviderAdapter，
//! 由 CLDR 本地化数据驱动）构造符号集。原生侧无本地化数据源，按 ROOT/en_US 常量
//! 集构造（ASCII 数字与小数点 —— Formatter 的 %f/%e/%g 只消费 zeroDigit 与
//! groupingSeparator，均与 JVM 默认输出一致）。locale 字段如实记录传入 locale，
//! 保持 Formatter 的 DFS 缓存按 locale 键控语义。

use crate::prelude::*;
use crate::java::util::{Locale, Locale_Category};
use super::decimal_format_symbols::DecimalFormatSymbols;

fn us_symbols(locale: Locale) -> Result<DecimalFormatSymbols> {
    let mut dfs = DecimalFormatSymbols::default();
    dfs._init_not_null();
    dfs.__set_zeroDigit('0' as u16);
    dfs.__set_groupingSeparator(',' as u16);
    dfs.__set_decimalSeparator('.' as u16);
    dfs.__set_perMill('\u{2030}' as u16);
    dfs.__set_percent('%' as u16);
    dfs.__set_digit('#' as u16);
    dfs.__set_patternSeparator(';' as u16);
    dfs.__set_infinity(String::from("\u{221E}"));
    dfs.__set_NaN(String::from("NaN"));
    dfs.__set_minusSign('-' as u16);
    dfs.__set_currencySymbol(String::from("$"));
    dfs.__set_intlCurrencySymbol(String::from("USD"));
    dfs.__set_monetarySeparator('.' as u16);
    dfs.__set_exponential('E' as u16);
    dfs.__set_exponentialSeparator(String::from("E"));
    dfs.__set_perMillText(String::from("\u{2030}"));
    dfs.__set_percentText(String::from("%"));
    dfs.__set_minusSignText(String::from("-"));
    dfs.__set_monetaryGroupingSeparator(',' as u16);
    dfs.__set_locale(locale);
    dfs.__set_serialVersionOnStream(5);
    Ok(dfs)
}

impl DecimalFormatSymbols {
    /// `getInstance()`：FORMAT 类别默认 locale（与 JDK 同源）。
    pub fn getInstance() -> Result<DecimalFormatSymbols> {
        let l = Locale::getDefault_locale_category(Locale_Category::FORMAT()?)?;
        Self::getInstance_locale(l)
    }

    /// `getInstance(Locale)`：sun/ SPI 的等价截断 —— 直接构造 ROOT/en_US 常量符号集。
    pub fn getInstance_locale(mut locale: Locale) -> Result<DecimalFormatSymbols> {
        us_symbols(locale)
    }
}
