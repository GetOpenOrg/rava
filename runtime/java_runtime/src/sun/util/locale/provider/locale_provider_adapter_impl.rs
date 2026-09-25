//! `sun/util/locale/provider/LocaleProviderAdapter` 手写伴生（仅当
//! `locale_provider_adapter.rs` 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 的适配器链（`getAdapterPreference` = [CLDR, FALLBACK] → `forType` 经
//! `Class.forName` 实例化 `sun.util.cldr.CLDRLocaleProviderAdapter` →
//! provider 经 `LocaleServiceProviderPool` 查询 `isSupportedLocale`）按内部边界类规则
//! （3b）截断为单例 `NativeLocaleAdapter`（运行时类名按 JDK 的 CLDR 适配器，
//! `getAdapter`/`forJRE`/`getResourceBundleBased`/`forType` 都返回它——唯一适配器即
//! FALLBACK 终态）。
//!
//! L-1：本地化**数据**不再手写——`getLocaleResources(locale)` 返回的
//! `LocaleResources`（手写边界，`locale_resources_impl.rs`）按候选链装载 CLDR 资源束的
//! 翻译字节码；`DecimalFormatSymbolsProvider.getInstance` 即 JDK 的
//! `new DecimalFormatSymbols(locale)`（翻译构造器 → initialize → getDecimalFormatSymbolsData）；
//! `NumberFormatProvider` 四族按 JDK `NumberFormatProviderImpl.getInstance` 语义取
//! `getNumberPatterns()` 的模式构造 `DecimalFormat`（翻译字节码）。

use crate::prelude::*;
use super::locale_provider_adapter::{LocaleProviderAdapter, LocaleProviderAdapter__VTable};
use super::locale_provider_adapter_type::LocaleProviderAdapter_Type;
use crate::java::lang::Class;
use crate::java::text::spi::NumberFormatProvider;
use crate::java::text::spi::NumberFormatProvider__VTable;
use crate::java::text::spi::{DecimalFormatSymbolsProvider, DecimalFormatSymbolsProvider__VTable};
use super::locale_resources::LocaleResources;
use crate::java::util::spi::LocaleServiceProvider__VTable;
use crate::java::text::{DecimalFormat, DecimalFormatSymbols, NumberFormat};
use crate::java::util::Locale;
use std::cell::RefCell;

/// `java/text/spi/NumberFormatProvider` 的手写实现对象（JDK 的
/// CLDRLocaleProviderAdapter 匿名 NumberFormatProvider → `NumberFormatProviderImpl`）：
/// 模式取自 `LocaleResources.getNumberPatterns()`（CLDR 束翻译字节码），下标即 JDK
/// `NumberFormat` 的样式常量（NUMBERSTYLE=0 / CURRENCYSTYLE=1 / PERCENTSTYLE=2；
/// INTEGERSTYLE 取 NUMBERSTYLE 模式后收窄小数位）。
struct NativeNumberFormatProvider;

const NUMBERSTYLE: i32 = 0;
const CURRENCYSTYLE: i32 = 1;
const PERCENTSTYLE: i32 = 2;

impl NativeNumberFormatProvider {
    fn new_format(locale: Locale, entry: i32) -> Result<DecimalFormat> {
        let patterns = LocaleResources::new(Object::default(), Clone::clone(&locale))?.getNumberPatterns()?;
        let symbols = DecimalFormatSymbols::getInstance_locale(locale)?;
        DecimalFormat::new_str_decimalformatsymbols(patterns.get(entry)?, symbols)
    }

    fn view(df: DecimalFormat) -> NumberFormat {
        <NumberFormat as ::std::convert::From<DecimalFormat>>::from(df)
    }
}

impl NumberFormatProvider__VTable for NativeNumberFormatProvider {
    fn getNumberInstance(&self, arg0: Locale) -> Result<NumberFormat> {
        Ok(Self::view(Self::new_format(arg0, NUMBERSTYLE)?))
    }

    /// INTEGERSTYLE：NUMBERSTYLE 模式 + `setMaximumFractionDigits(0)` /
    /// `setDecimalSeparatorAlwaysShown(false)` / `setParseIntegerOnly(true)`（JDK 同序）。
    fn getIntegerInstance(&self, arg0: Locale) -> Result<NumberFormat> {
        let df = Self::new_format(arg0, NUMBERSTYLE)?;
        df.setMaximumFractionDigits(0)?;
        df.setDecimalSeparatorAlwaysShown(false)?;
        df.setParseIntegerOnly(true)?;
        Ok(Self::view(df))
    }

    fn getCurrencyInstance(&self, arg0: Locale) -> Result<NumberFormat> {
        Ok(Self::view(Self::new_format(arg0, CURRENCYSTYLE)?))
    }

    fn getPercentInstance(&self, arg0: Locale) -> Result<NumberFormat> {
        Ok(Self::view(Self::new_format(arg0, PERCENTSTYLE)?))
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

/// `java/text/spi/DecimalFormatSymbolsProvider` 的手写实现对象（JDK 的
/// `DecimalFormatSymbolsProviderImpl`）：`getInstance(locale)` = `new DecimalFormatSymbols(locale)`
/// ——符号集由翻译构造器按 CLDR 束数据初始化。
struct NativeDecimalFormatSymbolsProvider;

impl DecimalFormatSymbolsProvider__VTable for NativeDecimalFormatSymbolsProvider {
    fn getInstance(&self, arg0: Locale) -> Result<DecimalFormatSymbols> {
        DecimalFormatSymbols::new_locale(arg0)
    }

    fn __as_DecimalFormatSymbolsProvider(&self) -> DecimalFormatSymbolsProvider {
        let rc = Rc::new(NativeDecimalFormatSymbolsProvider);
        DecimalFormatSymbolsProvider::__from_parts(
            Rc::clone(&rc) as Rc<dyn DecimalFormatSymbolsProvider__VTable>,
            rc as Rc<dyn std::any::Any>,
            false,
        )
    }
}

impl LocaleServiceProvider__VTable for NativeDecimalFormatSymbolsProvider {
    fn __as_LocaleServiceProvider(&self) -> crate::java::util::spi::LocaleServiceProvider {
        let rc = Rc::new(NativeDecimalFormatSymbolsProvider);
        crate::java::util::spi::LocaleServiceProvider::__from_parts(
            Rc::clone(&rc) as Rc<dyn LocaleServiceProvider__VTable>,
            rc as Rc<dyn std::any::Any>,
            false,
        )
    }
}

impl ObjectVTable for NativeDecimalFormatSymbolsProvider {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str { "sun/util/locale/provider/DecimalFormatSymbolsProviderImpl" }
    fn __obj_str(&self) -> std::string::String {
        "sun.util.locale.provider.DecimalFormatSymbolsProviderImpl".to_owned()
    }
}

/// `LocaleProviderAdapter` 的手写实现对象（JDK 的 CLDR 适配器单例形态）。
struct NativeLocaleAdapter;

impl LocaleProviderAdapter__VTable for NativeLocaleAdapter {
    fn getNumberFormatProvider(&self) -> Result<NumberFormatProvider> {
        _provider_view()
    }

    fn getDecimalFormatSymbolsProvider(&self) -> Result<DecimalFormatSymbolsProvider> {
        let rc = Rc::new(NativeDecimalFormatSymbolsProvider);
        Ok(DecimalFormatSymbolsProvider::__from_parts(
            Rc::clone(&rc) as Rc<dyn DecimalFormatSymbolsProvider__VTable>,
            rc as Rc<dyn std::any::Any>,
            false,
        ))
    }

    /// `getLocaleResources(locale)`：CLDR 束数据的装载截断点（见 locale_resources_impl.rs）。
    fn getLocaleResources(&self, arg0: Locale) -> Result<LocaleResources> {
        LocaleResources::new(Object::default(), arg0)
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
    ///
    /// 回调边：DFS provider 的翻译构造器、LocaleResources 构造、NumberFormatProvider 的
    /// INTEGERSTYLE 收窄（DecimalFormat 成员）。
    #[jvm_boundary(upcalls = "java/text/DecimalFormatSymbols.<init>:(Ljava/util/Locale;)V sun/util/locale/provider/LocaleResources.<init>:(Lsun/util/locale/provider/ResourceBundleBasedAdapter;Ljava/util/Locale;)V sun/util/locale/provider/LocaleResources.getNumberPatterns:()[Ljava/lang/String; sun/util/locale/provider/LocaleResources.getDecimalFormatSymbolsData:()[Ljava/lang/Object; java/text/DecimalFormat.setMaximumFractionDigits:(I)V java/text/DecimalFormat.setDecimalSeparatorAlwaysShown:(Z)V java/text/NumberFormat.setParseIntegerOnly:(Z)V")]
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
