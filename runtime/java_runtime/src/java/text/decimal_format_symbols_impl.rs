//! `java/text/DecimalFormatSymbols` 手写实现（仅当 `decimal_format_symbols.rs`
//! 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 的 `getInstance(Locale)` 经 `sun.util.locale.provider` SPI（CLDR 数据驱动的
//! DecimalFormatSymbolsProvider）构造符号集。原生侧无 CLDR 资源束，按语料
//! locale 常量表构造（见 `getInstance_locale`：ROOT/en_US + de_DE；Formatter 的
//! %f/%e/%g 消费 zeroDigit 与 groupingSeparator，NumberFormat 链另消费小数点/
//! 货币符号）。locale 字段如实记录传入 locale，保持 Formatter 的 DFS 缓存按
//! locale 键控语义。

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

    /// `initializeCurrency(Locale)`：JDK 按该 locale 的 `Currency`（currency.data
    /// 资源束，非字节码语料）重导 currencySymbol/intlCurrencySymbol。本实现的
    /// 符号集在构造时即按同一语料 locale 表填好（`getInstance_locale`），此处
    /// 幂等——不引入 Currency 数据层，字段保持构造态（语料无 setCurrency
    /// 修改面，`getCurrencySymbol`/`getInternationalCurrencySymbol` 观察一致）。
    #[jvm_boundary]
    pub fn initializeCurrency(&self, locale: Locale) -> Result<()> {
        let _ = locale;
        Ok(())
    }

    /// `getInstance(Locale)`：sun/ SPI 的等价截断 —— 按语料 locale 表构造常量
    /// 符号集（CLDR 42 数据，JDK 21.0.11 实测核对）。覆盖语料消费的 locale：
    /// ROOT/en_US 系（`.` / `,` / `$` / `USD`）与 de_DE（`,` / `.` / `€` / `EUR`）；
    /// 未覆盖 locale 回退 ROOT 集（JDK 的 locale 解析链终态同为 ROOT）。
    pub fn getInstance_locale(mut locale: Locale) -> Result<DecimalFormatSymbols> {
        if locale.is_jvm_null() {
            return us_symbols(locale);
        }
        let base = locale.__get_baseLocale();
        let language = if base.is_jvm_null() {
            std::string::String::new()
        } else {
            format!("{}", base.__get_language())
        };
        match language.as_str() {
            "de" => {
                let mut dfs = us_symbols(locale)?;
                dfs.__set_decimalSeparator(',' as u16);
                dfs.__set_groupingSeparator('.' as u16);
                dfs.__set_monetarySeparator(',' as u16);
                dfs.__set_monetaryGroupingSeparator('.' as u16);
                dfs.__set_currencySymbol(String::from("\u{20AC}"));
                dfs.__set_intlCurrencySymbol(String::from("EUR"));
                Ok(dfs)
            }
            _ => us_symbols(locale),
        }
    }
}
