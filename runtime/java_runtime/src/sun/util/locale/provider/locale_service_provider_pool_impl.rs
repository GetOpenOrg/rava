//! `sun/util/locale/provider/LocaleServiceProviderPool` 手写伴生：内部边界类，按调用链按需实现
//!（K-2 规则）。
//!
//! JDK 的池按 adapterPreference（CLDR → FALLBACK）× 候选 locale 逐级询问 provider 的
//! `isSupportedLocale`，再经取值器（`LocalizedObjectGetter`，翻译字节码）向 provider 取值。原生侧
//! 唯一数据适配器即 CLDR（见 locale_provider_adapter_impl.rs），CLDR provider 的取值本身沿资源束
//! 父链回退到 ROOT——首轮即得终值，逐级询问退化为一次 `getter.getObject(provider, locale, key,
//! params)`。取值器与 provider 语义（键的大小写约定、null 回退）全部与 JDK 同一路径。
//!
//! L-2（docs/plans/2026-09-28-l2-currency-data.md）：CurrencyNameProvider 的实现对象即 JDK
//! `CurrencyNameProviderImpl`：符号 = `getCurrencyName(代码大写)`、显示名 = `getCurrencyName(代码小写)`。

use crate::prelude::*;
use super::locale_service_provider_pool::LocaleServiceProviderPool;
use super::locale_service_provider_pool_localized_object_getter::LocaleServiceProviderPool_LocalizedObjectGetter;
use super::locale_resources::LocaleResources;
use crate::java::lang::Class;
use crate::java::util::Locale;
use crate::java::util::spi::{CurrencyNameProvider, CurrencyNameProvider__VTable, LocaleServiceProvider__VTable};

/// `java/util/spi/CurrencyNameProvider` 的手写实现对象（JDK `CurrencyNameProviderImpl`）。
struct NativeCurrencyNameProvider;

impl NativeCurrencyNameProvider {
    fn name(code: String, locale: Locale, upper: bool) -> Result<String> {
        if code.is_jvm_null() || locale.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let c = format!("{}", code);
        let key = if upper { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() };
        LocaleResources::new(Object::default(), locale)?.__impl_getCurrencyName(String::from(key.as_str()))
    }

    fn view() -> CurrencyNameProvider {
        let rc = Rc::new(NativeCurrencyNameProvider);
        CurrencyNameProvider::__from_parts(
            Rc::clone(&rc) as Rc<dyn CurrencyNameProvider__VTable>,
            rc as crate::sync_model::__AnyRef,
            false,
        )
    }
}

impl CurrencyNameProvider__VTable for NativeCurrencyNameProvider {
    fn getSymbol(&self, arg0: String, arg1: Locale) -> Result<String> {
        Self::name(arg0, arg1, true)
    }

    fn getDisplayName(&self, currencyCode: String, locale: Locale) -> Result<String> {
        Self::name(currencyCode, locale, false)
    }

    fn __as_CurrencyNameProvider(&self) -> CurrencyNameProvider {
        Self::view()
    }
}

impl LocaleServiceProvider__VTable for NativeCurrencyNameProvider {
    fn __as_LocaleServiceProvider(&self) -> crate::java::util::spi::LocaleServiceProvider {
        let rc = Rc::new(NativeCurrencyNameProvider);
        crate::java::util::spi::LocaleServiceProvider::__from_parts(
            Rc::clone(&rc) as Rc<dyn LocaleServiceProvider__VTable>,
            rc as crate::sync_model::__AnyRef,
            false,
        )
    }
}

impl ObjectVTable for NativeCurrencyNameProvider {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str { "sun/util/locale/provider/CurrencyNameProviderImpl" }
    fn __obj_str(&self) -> std::string::String {
        "sun.util.locale.provider.CurrencyNameProviderImpl".to_owned()
    }
}

impl LocaleServiceProviderPool {
    /// `getPool(Class)`：按 provider 类的池（JDK poolOfPools 缓存；池对象只携带 providerClass）。
    #[jvm_boundary]
    pub fn getPool(providerClass: Class) -> Result<LocaleServiceProviderPool> {
        let mut pool = LocaleServiceProviderPool::default();
        pool._init_not_null();
        pool.__set_providerClass(providerClass);
        Ok(pool)
    }

    /// `getLocalizedObject(getter, locale, key, params)`：见模块说明。provider 按池的 providerClass
    /// 取 CLDR 实现对象；未建模的 provider 类 → 精确存根。
    #[jvm_boundary(upcalls = "sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter.getObject:(Ljava/util/spi/LocaleServiceProvider;Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object; java/util/spi/CurrencyNameProvider.getSymbol:(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/String; java/util/spi/CurrencyNameProvider.getDisplayName:(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/String; sun/util/locale/provider/LocaleResources.<init>:(Lsun/util/locale/provider/ResourceBundleBasedAdapter;Ljava/util/Locale;)V sun/util/locale/provider/LocaleResources.getCurrencyName:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn __impl_getLocalizedObject_localeserviceproviderpool_localizedobjectgetter_locale_str_arr_obj(
        &self, getter: Object, locale: Locale, key: String, params: JArray<Object>,
    ) -> Result<Object> {
        if locale.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let cls = format!("{}", self.__get_providerClass().__get_name()).replace('.', "/");
        let provider = match cls.as_str() {
            "java/util/spi/CurrencyNameProvider" => Object::from(NativeCurrencyNameProvider::view()),
            other => panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getLocalizedObject（provider {other} 未建模）"),
        };
        let getter = <LocaleServiceProviderPool_LocalizedObjectGetter<Object, Object> as ::std::convert::From<Object>>::from(getter);
        getter.getObject(provider, locale, key, params)
    }
}
