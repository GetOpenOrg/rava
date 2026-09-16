#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Currency$OtherCurrencyEntry"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Currency.java"]
    #[inner_classes     = "java/util/Currency$OtherCurrencyEntry:java/util/Currency:OtherCurrencyEntry:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Currency$OtherCurrencyEntry"]

    pub struct Currency_OtherCurrencyEntry {
        #[cfg_attr(any(), java_field(name = "currencyCode", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub currencyCode: String,
        #[cfg_attr(any(), java_field(name = "fraction", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub fraction: i32,
        #[cfg_attr(any(), java_field(name = "numericCode", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub numericCode: i32,
    }

    impl Currency_OtherCurrencyEntry {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(currencyCode: String, fraction: i32, numericCode: i32) -> Result<Self> {
            panic!("stub: java/util/Currency$OtherCurrencyEntry.<init>:(Ljava/lang/String;II)V")
        }

        #[java_method(name = "findEntry", descriptor = "(Ljava/lang/String;)Ljava/util/Currency$OtherCurrencyEntry;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findEntry(mut code: String) -> Result<Currency_OtherCurrencyEntry> {
            let _vdispatch0: i32 = if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let mut size: i32 = _vdispatch0;
            let mut index: i32 = 0i32;
            loop {
                if index >= size { break; }
                let _vdispatch1: Object = if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<Object>() { _d.get(index)? } else if let Some(__f) = Currency::otherCurrenciesList().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<Object>>>() { (__f)(index)? } else { Default::default() };
                let mut ocEntry = (_vdispatch1).downcast::<Currency_OtherCurrencyEntry>();
                let _t2 = ocEntry.__get_currencyCode().equalsIgnoreCase(Clone::clone(&code))?;
                if _t2 {
                    return Ok(ocEntry);
                }
                index = index.wrapping_add(1i32);
            }
            Ok(Default::default())
        }
    }
}
