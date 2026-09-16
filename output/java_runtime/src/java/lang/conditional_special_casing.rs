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
use crate::sun::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/ConditionalSpecialCasing"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ConditionalSpecialCasing.java"]
    #[inner_classes     = "java/lang/ConditionalSpecialCasing$Entry:java/lang/ConditionalSpecialCasing:Entry:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/ConditionalSpecialCasing;java/lang/Object"]

    pub struct ConditionalSpecialCasing;

    impl ConditionalSpecialCasing {
        #[cfg_attr(any(), java_field(name = "FINAL_CASED", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: FINAL_CASED:I
        pub fn FINAL_CASED() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "AFTER_SOFT_DOTTED", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: AFTER_SOFT_DOTTED:I
        pub fn AFTER_SOFT_DOTTED() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "MORE_ABOVE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: MORE_ABOVE:I
        pub fn MORE_ABOVE() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "AFTER_I", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: AFTER_I:I
        pub fn AFTER_I() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "NOT_BEFORE_DOT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: NOT_BEFORE_DOT:I
        pub fn NOT_BEFORE_DOT() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "COMBINING_CLASS_ABOVE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "230"))]
        // static field: COMBINING_CLASS_ABOVE:I
        pub fn COMBINING_CLASS_ABOVE() -> i32 {
            230
        }

        #[cfg_attr(any(), java_field(name = "entry", descriptor = "[Ljava/lang/ConditionalSpecialCasing$Entry;", access = "package", modifiers = "static", is_static = true))]
        // static field: entry:[Ljava/lang/ConditionalSpecialCasing$Entry;
        pub fn entry() -> Rc<RefCell<Vec<ConditionalSpecialCasing_Entry>>> {
            panic!("stub: java/lang/ConditionalSpecialCasing.entry:[Ljava/lang/ConditionalSpecialCasing$Entry;")
        }

        #[cfg_attr(any(), java_field(name = "entryTable", descriptor = "Ljava/util/HashMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/HashMap<Ljava/lang/Integer;Ljava/util/HashSet<Ljava/lang/ConditionalSpecialCasing$Entry;>;>;"))]
        // static field: entryTable:Ljava/util/HashMap;
        pub fn entryTable() -> HashMap<i32, HashSet<ConditionalSpecialCasing_Entry>> {
            panic!("stub: java/lang/ConditionalSpecialCasing.entryTable:Ljava/util/HashMap;")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/ConditionalSpecialCasing.<init>:()V")
        }

        #[java_method(name = "toLowerCaseEx", descriptor = "(Ljava/lang/String;ILjava/util/Locale;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCaseEx(mut src: String, mut index: i32, mut locale: Locale) -> Result<i32> {
            let _t0: Rc<RefCell<Vec<u16>>> = ConditionalSpecialCasing::lookUpTable(Clone::clone(&src), index, Clone::clone(&locale), (1i32 != 0i32))?;
            let mut result: Rc<RefCell<Vec<u16>>> = _t0;
            if (result.borrow().len() as i32) == 1i32 {
                return Ok((result.borrow()[0i32 as usize] as i32));
            }
            return Ok(-1i32);
            let _t1 = src.codePointAt(index)?;
            let _t2: i32 = Character::toLowerCase_i(_t1)?;
            Ok(_t2)
        }

        #[java_method(name = "toUpperCaseEx", descriptor = "(Ljava/lang/String;ILjava/util/Locale;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseEx(mut src: String, mut index: i32, mut locale: Locale) -> Result<i32> {
            let _t0: Rc<RefCell<Vec<u16>>> = ConditionalSpecialCasing::lookUpTable(Clone::clone(&src), index, Clone::clone(&locale), (0i32 != 0i32))?;
            let mut result: Rc<RefCell<Vec<u16>>> = _t0;
            if (result.borrow().len() as i32) == 1i32 {
                return Ok((result.borrow()[0i32 as usize] as i32));
            }
            return Ok(-1i32);
            let _t1 = src.codePointAt(index)?;
            let _t2: i32 = Character::toUpperCaseEx(_t1)?;
            Ok(_t2)
        }

        #[java_method(name = "toLowerCaseCharArray", descriptor = "(Ljava/lang/String;ILjava/util/Locale;)[C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCaseCharArray(mut src: String, mut index: i32, mut locale: Locale) -> Result<Rc<RefCell<Vec<u16>>>> {
            let _t0: Rc<RefCell<Vec<u16>>> = ConditionalSpecialCasing::lookUpTable(Clone::clone(&src), index, Clone::clone(&locale), (1i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "toUpperCaseCharArray", descriptor = "(Ljava/lang/String;ILjava/util/Locale;)[C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseCharArray(mut src: String, mut index: i32, mut locale: Locale) -> Result<Rc<RefCell<Vec<u16>>>> {
            let _t0: Rc<RefCell<Vec<u16>>> = ConditionalSpecialCasing::lookUpTable(Clone::clone(&src), index, Clone::clone(&locale), (0i32 != 0i32))?;
            let mut result: Rc<RefCell<Vec<u16>>> = _t0;
            if !_is_jnull(&result) {
                return Ok(result);
            }
            let _t1 = src.codePointAt(index)?;
            let _t2: Rc<RefCell<Vec<u16>>> = Character::toUpperCaseCharArray(_t1)?;
            Ok(_t2)
        }

        #[java_method(name = "lookUpTable", descriptor = "(Ljava/lang/String;ILjava/util/Locale;Z)[C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lookUpTable(mut src: String, mut index: i32, mut locale: Locale, mut bLowerCasing: bool) -> Result<Rc<RefCell<Vec<u16>>>> {
            let _t0 = src.codePointAt(index)?;
            let _t1 = ConditionalSpecialCasing::entryTable().get(_t0.into())?;
            let mut set = (_t1).downcast::<HashSet<Object>>();
            let mut ret: Object = Object::default();
            let _t2 = set.iterator()?;
            let mut iter: Object = _t2;
            let _t3 = locale.getLanguage()?;
            let mut currentLang: String = _t3;
            let mut ret: Rc<RefCell<Vec<u16>>> = Default::default();
            loop {
                let _vdispatch4: bool = if let Some(_d) = iter.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = iter.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch4) { break; }
                let _vdispatch4: Object = if let Some(_d) = iter.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = iter.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = iter.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut entry = (_vdispatch4).downcast::<ConditionalSpecialCasing_Entry>();
                let _t5 = entry.getLanguage()?;
                let mut conditionLang: String = _t5;
                let _t6 = conditionLang.equals(Object::from_any(currentLang.clone()))?;
                if _t6 {
                    let _t7 = entry.getCondition()?;
                    let _t8: bool = ConditionalSpecialCasing::isConditionMet(Clone::clone(&src), index, Clone::clone(&locale), _t7)?;
                    if _t8 {
                        let mut _merged10: Rc<RefCell<Vec<u16>>>;
                        if bLowerCasing {
                            let _t9 = entry.getLowerCase()?;
                            _merged10 = _t9;
                        } else {
                            let _t9 = entry.getUpperCase()?;
                            _merged10 = _t9;
                        }
                        ret = _merged10;
                        if !_is_jnull(&conditionLang) {
                            break;
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            Ok(ret)
        }

        #[java_method(name = "isConditionMet", descriptor = "(Ljava/lang/String;ILjava/util/Locale;I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isConditionMet(src: String, index: i32, locale: Locale, condition: i32) -> Result<bool> {
            panic!("stub: java/lang/ConditionalSpecialCasing.isConditionMet:(Ljava/lang/String;ILjava/util/Locale;I)Z")
        }

        #[java_method(name = "isFinalCased", descriptor = "(Ljava/lang/String;ILjava/util/Locale;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFinalCased(mut src: String, mut index: i32, mut locale: Locale) -> Result<bool> {
            let _t0: BreakIterator = BreakIterator::getWordInstance_locale(Clone::clone(&locale))?;
            let mut wordBoundary: BreakIterator = _t0;
            wordBoundary.setText_str(Clone::clone(&src))?;
            let mut i: i32 = index;
            loop {
                if (i<0) { break; }
                let _t1 = wordBoundary.isBoundary(i)?;
                let _t2 = src.codePointBefore(i)?;
                let mut ch: i32 = _t2;
                let _t3: bool = ConditionalSpecialCasing::isCased(ch)?;
                let _t4 = src.length()?;
                let mut len: i32 = _t4;
                let _t5 = src.codePointAt(index)?;
                let _t6: i32 = Character::charCount(_t5)?;
                i = (index).wrapping_add(_t6);
                loop {
                    if i >= len { break; }
                    let _t7 = wordBoundary.isBoundary(i)?;
                    let _t8 = src.codePointAt(i)?;
                    ch = _t8;
                    let _t9: bool = ConditionalSpecialCasing::isCased(ch)?;
                    if _t9 {
                        return Ok((0i32 != 0i32));
                    }
                    let _t10: i32 = Character::charCount(ch)?;
                    i = (i).wrapping_add(_t10);
                }
                return Ok((1i32 != 0i32));
                let _t7: i32 = Character::charCount(ch)?;
                i = (i).wrapping_sub(_t7);
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "isAfterI", descriptor = "(Ljava/lang/String;I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAfterI(mut src: String, mut index: i32) -> Result<bool> {
            let mut i: i32 = index;
            loop {
                if (i<=0) { break; }
                let _t0 = src.codePointBefore(i)?;
                let mut ch: i32 = _t0;
                if ch == 73i32 {
                    return Ok((1i32 != 0i32));
                }
                let _t1: i32 = Normalizer::getCombiningClass(ch)?;
                let mut cc: i32 = _t1;
                if cc == 230i32 {
                    return Ok((0i32 != 0i32));
                }
                let _t2: i32 = Character::charCount(ch)?;
                i = (i).wrapping_sub(_t2);
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "isAfterSoftDotted", descriptor = "(Ljava/lang/String;I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAfterSoftDotted(mut src: String, mut index: i32) -> Result<bool> {
            let mut i: i32 = index;
            loop {
                if (i<=0) { break; }
                let _t0 = src.codePointBefore(i)?;
                let mut ch: i32 = _t0;
                let _t1: bool = ConditionalSpecialCasing::isSoftDotted(ch)?;
                if _t1 {
                    return Ok((1i32 != 0i32));
                }
                let _t2: i32 = Normalizer::getCombiningClass(ch)?;
                let mut cc: i32 = _t2;
                if cc == 230i32 {
                    return Ok((0i32 != 0i32));
                }
                let _t3: i32 = Character::charCount(ch)?;
                i = (i).wrapping_sub(_t3);
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "isMoreAbove", descriptor = "(Ljava/lang/String;I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMoreAbove(mut src: String, mut index: i32) -> Result<bool> {
            let _t0 = src.length()?;
            let mut len: i32 = _t0;
            let _t1 = src.codePointAt(index)?;
            let _t2: i32 = Character::charCount(_t1)?;
            let mut i = (index).wrapping_add(_t2);
            loop {
                if i >= len { break; }
                let _t3 = src.codePointAt(i)?;
                let mut ch: i32 = _t3;
                let _t4: i32 = Normalizer::getCombiningClass(ch)?;
                let mut cc: i32 = _t4;
                if cc == 230i32 {
                    return Ok((1i32 != 0i32));
                }
                if (cc==0) {
                    return Ok((0i32 != 0i32));
                }
                let _t5: i32 = Character::charCount(ch)?;
                i = (i).wrapping_add(_t5);
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "isBeforeDot", descriptor = "(Ljava/lang/String;I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBeforeDot(mut src: String, mut index: i32) -> Result<bool> {
            let _t0 = src.length()?;
            let mut len: i32 = _t0;
            let _t1 = src.codePointAt(index)?;
            let _t2: i32 = Character::charCount(_t1)?;
            let mut i = (index).wrapping_add(_t2);
            loop {
                if i >= len { break; }
                let _t3 = src.codePointAt(i)?;
                let mut ch: i32 = _t3;
                if ch == 775i32 {
                    return Ok((1i32 != 0i32));
                }
                let _t4: i32 = Normalizer::getCombiningClass(ch)?;
                let mut cc: i32 = _t4;
                if cc == 230i32 {
                    return Ok((0i32 != 0i32));
                }
                let _t5: i32 = Character::charCount(ch)?;
                i = (i).wrapping_add(_t5);
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "isCased", descriptor = "(I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCased(mut ch: i32) -> Result<bool> {
            let _t0: i32 = Character::getType_i(ch)?;
            let mut type_: i32 = _t0;
            if type_ == 3i32 {
                return Ok((1i32 != 0i32));
            }
            if ch <= 696i32 {
                return Ok((1i32 != 0i32));
            }
            if ch <= 705i32 {
                return Ok((1i32 != 0i32));
            }
            if ch <= 740i32 {
                return Ok((1i32 != 0i32));
            }
            if ch == 837i32 {
                return Ok((1i32 != 0i32));
            }
            if ch == 890i32 {
                return Ok((1i32 != 0i32));
            }
            if ch <= 7521i32 {
                return Ok((1i32 != 0i32));
            }
            if ch <= 8575i32 {
                return Ok((1i32 != 0i32));
            }
            if ch <= 9449i32 {
                return Ok((1i32 != 0i32));
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "isSoftDotted", descriptor = "(I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSoftDotted(mut ch: i32) -> Result<bool> {
            let _switch_key = ch;
            return Ok((1i32 != 0i32));
            Ok((0i32 != 0i32))
        }
    }
}
