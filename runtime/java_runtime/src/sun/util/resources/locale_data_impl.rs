//! `sun/util/resources/LocaleData` 手写伴生（内部边界类，按调用链按需实现）。
//!
//! JDK 的 `getBundle(<包>.FormatData, locale)` 经 `ResourceBundle.getBundle` 类名反射装载
//! CLDR 束；原生侧与 `LocaleResources` 同一截断点：候选链上已登记的束逐个实例化、
//! `setParent` 串接（`locale_resources_impl.rs::__number_format_data`）。CLDR 适配器下
//! DateFormatData 与 NumberFormatData 是同一个 FormatData 束。

use crate::prelude::*;
use super::locale_data::LocaleData;
use crate::java::util::{Locale, ResourceBundle};
use crate::sun::util::locale::provider::LocaleResources;

impl LocaleData {
    /// `getDateFormatData(Locale)`：FormatData 束链（月名 / 星期名 / 纪元 / 日期模式）。
    /// 消费方：DateFormatSymbols.initializeData（`new DateFormatSymbols(locale)`）。
    #[jvm_boundary(upcalls = "java/util/ResourceBundle.setParent:(Ljava/util/ResourceBundle;)V")]
    pub fn __impl_getDateFormatData(&self, locale: Locale) -> Result<ResourceBundle> {
        LocaleResources::new(Object::default(), locale)?.__number_format_data()
    }
}
