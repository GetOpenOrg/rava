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
    #[binary_name       = "java/util/Currency$SpecialCaseEntry"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Currency.java"]
    #[inner_classes     = "java/util/Currency$SpecialCaseEntry:java/util/Currency:SpecialCaseEntry:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Currency$SpecialCaseEntry"]

    pub struct Currency_SpecialCaseEntry {
        #[cfg_attr(any(), java_field(name = "cutOverTime", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
        pub cutOverTime: i64,
        #[cfg_attr(any(), java_field(name = "oldCurrency", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub oldCurrency: String,
        #[cfg_attr(any(), java_field(name = "newCurrency", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub newCurrency: String,
        #[cfg_attr(any(), java_field(name = "oldCurrencyFraction", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub oldCurrencyFraction: i32,
        #[cfg_attr(any(), java_field(name = "newCurrencyFraction", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub newCurrencyFraction: i32,
        #[cfg_attr(any(), java_field(name = "oldCurrencyNumericCode", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub oldCurrencyNumericCode: i32,
        #[cfg_attr(any(), java_field(name = "newCurrencyNumericCode", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub newCurrencyNumericCode: i32,
    }

    impl Currency_SpecialCaseEntry {
        #[java_method(name = "<init>", descriptor = "(JLjava/lang/String;Ljava/lang/String;IIII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_l_str_str_i_i_i_i(cutOverTime: i64, arg1: String, oldCurrency: String, newCurrency: i32, oldCurrencyFraction: i32, newCurrencyFraction: i32, oldCurrencyNumericCode: i32) -> Result<Self> {
            panic!("stub: java/util/Currency$SpecialCaseEntry.<init>:(JLjava/lang/String;Ljava/lang/String;IIII)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_i_i(currencyCode: String, fraction: i32, numericCode: i32) -> Result<Self> {
            panic!("stub: java/util/Currency$SpecialCaseEntry.<init>:(Ljava/lang/String;II)V")
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/String;II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf(code: String, fraction: i32, numeric: i32) -> Result<i32> {
            panic!("stub: java/util/Currency$SpecialCaseEntry.indexOf:(Ljava/lang/String;II)I")
        }

        #[java_method(name = "findEntry", descriptor = "(Ljava/lang/String;)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findEntry(mut code: String) -> Result<Rc<RefCell<Vec<i32>>>> {
            let mut fractionAndNumericCode: Object = Object::default();
            let _vdispatch0: i32 = if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = Currency::specialCasesList().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let mut size: i32 = _vdispatch0;
            let mut index: i32 = 0i32;
            loop {
                if index >= size { break; }
                let _vdispatch1: Object = if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.get(index)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<Object>() { _d.get(index)? } else if let Some(__f) = Currency::specialCasesList().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<Object>>>() { (__f)(index)? } else { Default::default() };
                let mut scEntry = (_vdispatch1).downcast::<Currency_SpecialCaseEntry>();
                let _t2 = scEntry.__get_oldCurrency().equals(Object::from_any(code.clone()))?;
                if _t2 {
                    let _t3: i64 = System::currentTimeMillis()?;
                    if (((_t3>(scEntry.__get_cutOverTime())) as i32-((_t3)<(scEntry.__get_cutOverTime())) as i32)<0) {
                        let mut _arr4: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 2i32 as usize]));
                        let mut fractionAndNumericCode: Rc<RefCell<Vec<i32>>> = _arr4;
                        fractionAndNumericCode.borrow_mut()[0i32 as usize] = scEntry.__get_oldCurrencyFraction();
                        fractionAndNumericCode.borrow_mut()[1i32 as usize] = scEntry.__get_oldCurrencyNumericCode();
                        break;
                    }
                } else {
                    let _t3 = scEntry.__get_newCurrency().equals(Object::from_any(code.clone()))?;
                    if _t3 {
                        let _t4: i64 = System::currentTimeMillis()?;
                        if (((_t4>(scEntry.__get_cutOverTime())) as i32-((_t4)<(scEntry.__get_cutOverTime())) as i32)>=0) {
                            let mut _arr5: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 2i32 as usize]));
                            let mut fractionAndNumericCode: Rc<RefCell<Vec<i32>>> = _arr5;
                            fractionAndNumericCode.borrow_mut()[0i32 as usize] = scEntry.__get_newCurrencyFraction();
                            fractionAndNumericCode.borrow_mut()[1i32 as usize] = scEntry.__get_newCurrencyNumericCode();
                            break;
                        }
                    } else {
                        index = index.wrapping_add(1i32);
                        continue;
                    }
                }
            }
            Ok(Default::default())
        }

        #[java_method(name = "currencyCodeIndex", descriptor = "(Ljava/lang/String;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn currencyCodeIndex(code: String) -> Result<i32> {
            panic!("stub: java/util/Currency$SpecialCaseEntry.currencyCodeIndex:(Ljava/lang/String;)I")
        }

        #[java_method(name = "toIndex", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toIndex(mut tableEntry: i32) -> Result<i32> {
            Ok(((tableEntry&31i32)).wrapping_sub(1i32))
        }
    }
}
