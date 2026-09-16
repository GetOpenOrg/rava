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
    #[binary_name       = "java/util/regex/CharPredicates"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharPredicates.java"]
    #[inner_classes     = "java/util/regex/Pattern$CharPredicate:java/util/regex/Pattern:CharPredicate:1544;java/lang/Character$UnicodeScript:java/lang/Character:UnicodeScript:16409;java/lang/Character$UnicodeBlock:java/lang/Character:UnicodeBlock:25;java/util/regex/Pattern$BmpCharPredicate:java/util/regex/Pattern:BmpCharPredicate:1544;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/regex/CharPredicates"]

    pub struct CharPredicates;

    impl CharPredicates {
        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/regex/CharPredicates.<init>:()V")
        }

        #[java_method(name = "ALPHABETIC", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ALPHABETIC() -> Result<Object> {
            let __lam_7: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isAlphabetic(_la0) });
            Ok(Object::from_any(__lam_7))
        }

        #[java_method(name = "DIGIT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn DIGIT() -> Result<Object> {
            let __lam_11: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isDigit(_la0) });
            Ok(Object::from_any(__lam_11))
        }

        #[java_method(name = "LETTER", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LETTER() -> Result<Object> {
            let __lam_12: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isLetter(_la0) });
            Ok(Object::from_any(__lam_12))
        }

        #[java_method(name = "IDEOGRAPHIC", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn IDEOGRAPHIC() -> Result<Object> {
            let __lam_13: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isIdeographic(_la0) });
            Ok(Object::from_any(__lam_13))
        }

        #[java_method(name = "LOWERCASE", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOWERCASE() -> Result<Object> {
            let __lam_14: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isLowerCase(_la0) });
            Ok(Object::from_any(__lam_14))
        }

        #[java_method(name = "UPPERCASE", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn UPPERCASE() -> Result<Object> {
            let __lam_15: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isUpperCase(_la0) });
            Ok(Object::from_any(__lam_15))
        }

        #[java_method(name = "TITLECASE", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn TITLECASE() -> Result<Object> {
            let __lam_16: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isTitleCase(_la0) });
            Ok(Object::from_any(__lam_16))
        }

        #[java_method(name = "WHITE_SPACE", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn WHITE_SPACE() -> Result<Object> {
            let __lam_17: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_WHITE_SPACE_0(_la0) });
            Ok(Object::from_any(__lam_17))
        }

        #[java_method(name = "CONTROL", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CONTROL() -> Result<Object> {
            let __lam_18: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_CONTROL_1(_la0) });
            Ok(Object::from_any(__lam_18))
        }

        #[java_method(name = "PUNCTUATION", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn PUNCTUATION() -> Result<Object> {
            let __lam_19: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_PUNCTUATION_2(_la0) });
            Ok(Object::from_any(__lam_19))
        }

        #[java_method(name = "HEX_DIGIT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn HEX_DIGIT() -> Result<Object> {
            let _t0: Object = CharPredicates::DIGIT()?;
            let __lam_25: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_HEX_DIGIT_3(_la0) });
            let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&Object::from_any(__lam_25)))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&Object::from_any(__lam_25)))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&Object::from_any(__lam_25)))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&Object::from_any(__lam_25)))? } else { Default::default() };
            Ok(_vdispatch1)
        }

        #[java_method(name = "ASSIGNED", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ASSIGNED() -> Result<Object> {
            let __lam_32: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_ASSIGNED_4(_la0) });
            Ok(Object::from_any(__lam_32))
        }

        #[java_method(name = "NONCHARACTER_CODE_POINT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn NONCHARACTER_CODE_POINT() -> Result<Object> {
            let __lam_33: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_NONCHARACTER_CODE_POINT_5(_la0) });
            Ok(Object::from_any(__lam_33))
        }

        #[java_method(name = "ALNUM", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ALNUM() -> Result<Object> {
            let _t0: Object = CharPredicates::ALPHABETIC()?;
            let _t1: Object = CharPredicates::DIGIT()?;
            let _vdispatch2: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&_t1))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t1))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t1))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t1))? } else { Default::default() };
            Ok(_vdispatch2)
        }

        #[java_method(name = "BLANK", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn BLANK() -> Result<Object> {
            let __lam_37: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_BLANK_6(_la0) });
            Ok(Object::from_any(__lam_37))
        }

        #[java_method(name = "GRAPH", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn GRAPH() -> Result<Object> {
            let __lam_38: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_GRAPH_7(_la0) });
            Ok(Object::from_any(__lam_38))
        }

        #[java_method(name = "PRINT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn PRINT() -> Result<Object> {
            let _t0: Object = CharPredicates::GRAPH()?;
            let _t1: Object = CharPredicates::BLANK()?;
            let _vdispatch2: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&_t1))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t1))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t1))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t1))? } else { Default::default() };
            let _t3: Object = CharPredicates::CONTROL()?;
            let _vdispatch4: Object = if let Some(_d) = _t3.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = _t3.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let _vdispatch5: Object = if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.and(Clone::clone(&_vdispatch4))? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<Object>() { _d.and(Clone::clone(&_vdispatch4))? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<Object>() { _d.and(Clone::clone(&_vdispatch4))? } else if let Some(__f) = _vdispatch2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_vdispatch4))? } else { Default::default() };
            Ok(_vdispatch5)
        }

        #[java_method(name = "JOIN_CONTROL", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn JOIN_CONTROL() -> Result<Object> {
            let __lam_54: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_JOIN_CONTROL_8(_la0) });
            Ok(Object::from_any(__lam_54))
        }

        #[java_method(name = "WORD", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn WORD() -> Result<Object> {
            let _t0: Object = CharPredicates::ALPHABETIC()?;
            let __lam_55: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_WORD_9(_la0) });
            let _t1: Object = CharPredicates::JOIN_CONTROL()?;
            let _vdispatch2: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter_patter(Clone::clone(&Object::from_any(__lam_55)), Clone::clone(&_t1))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&Object::from_any(__lam_55)), Clone::clone(&_t1))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&Object::from_any(__lam_55)), Clone::clone(&_t1))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&Object::from_any(__lam_55)), Clone::clone(&_t1))? } else { Default::default() };
            Ok(_vdispatch2)
        }

        #[java_method(name = "getPosixPredicate", descriptor = "(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPosixPredicate(name: String, caseIns: bool) -> Result<Object> {
            panic!("stub: java/util/regex/CharPredicates.getPosixPredicate:(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;")
        }

        #[java_method(name = "getUnicodePredicate", descriptor = "(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUnicodePredicate(name: String, caseIns: bool) -> Result<Object> {
            panic!("stub: java/util/regex/CharPredicates.getUnicodePredicate:(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;")
        }

        #[java_method(name = "forUnicodeProperty", descriptor = "(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forUnicodeProperty(mut propName: String, mut caseIns: bool) -> Result<Object> {
            let _t0 = propName.toUpperCase_locale(Clone::clone(&Locale::ROOT()))?;
            propName = _t0;
            let _t1: Object = CharPredicates::getUnicodePredicate(Clone::clone(&propName), caseIns)?;
            let mut p: Object = _t1;
            if !_is_jnull(&p) {
                return Ok(p);
            }
            let _t2: Object = CharPredicates::getPosixPredicate(Clone::clone(&propName), caseIns)?;
            Ok(_t2)
        }

        #[java_method(name = "forPOSIXName", descriptor = "(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forPOSIXName(mut propName: String, mut caseIns: bool) -> Result<Object> {
            let _t0 = propName.toUpperCase_locale(Clone::clone(&Locale::ENGLISH()))?;
            let _t1: Object = CharPredicates::getPosixPredicate(Clone::clone(&_t0), caseIns)?;
            Ok(_t1)
        }

        #[java_method(name = "forUnicodeScript", descriptor = "(Ljava/lang/String;)Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forUnicodeScript(name: String) -> Result<Object> {
            panic!("stub: java/util/regex/CharPredicates.forUnicodeScript:(Ljava/lang/String;)Ljava/util/regex/Pattern$CharPredicate;")
        }

        #[java_method(name = "forUnicodeBlock", descriptor = "(Ljava/lang/String;)Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forUnicodeBlock(name: String) -> Result<Object> {
            panic!("stub: java/util/regex/CharPredicates.forUnicodeBlock:(Ljava/lang/String;)Ljava/util/regex/Pattern$CharPredicate;")
        }

        #[java_method(name = "forProperty", descriptor = "(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forProperty(name: String, caseIns: bool) -> Result<Object> {
            panic!("stub: java/util/regex/CharPredicates.forProperty:(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;")
        }

        #[java_method(name = "category", descriptor = "(I)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn category(mut typeMask: i32) -> Result<Object> {
            let __lam_cap411_0 = typeMask;
            let __lam_411: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_category_15(__lam_cap411_0.clone(), _la0) });
            Ok(Object::from_any(__lam_411))
        }

        #[java_method(name = "range", descriptor = "(II)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(mut lower: i32, mut upper: i32) -> Result<Object> {
            let __lam_cap413_0 = upper;
            let __lam_cap413_1 = lower;
            let __lam_413: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_range_16(__lam_cap413_0.clone(), __lam_cap413_1.clone(), _la0) });
            Ok(Object::from_any(__lam_413))
        }

        #[java_method(name = "ctype", descriptor = "(I)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ctype(mut ctype: i32) -> Result<Object> {
            let __lam_cap416_0 = ctype;
            let __lam_416: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_ctype_17(__lam_cap416_0.clone(), _la0) });
            Ok(Object::from_any(__lam_416))
        }

        #[java_method(name = "ASCII_DIGIT", descriptor = "()Ljava/util/regex/Pattern$BmpCharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ASCII_DIGIT() -> Result<Object> {
            let __lam_419: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_ASCII_DIGIT_18(_la0) });
            Ok(Object::from_any(__lam_419))
        }

        #[java_method(name = "ASCII_WORD", descriptor = "()Ljava/util/regex/Pattern$BmpCharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ASCII_WORD() -> Result<Object> {
            let __lam_422: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_ASCII_WORD_19(_la0) });
            Ok(Object::from_any(__lam_422))
        }

        #[java_method(name = "ASCII_SPACE", descriptor = "()Ljava/util/regex/Pattern$BmpCharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ASCII_SPACE() -> Result<Object> {
            let __lam_423: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { CharPredicates::lambda_ASCII_SPACE_20(_la0) });
            Ok(Object::from_any(__lam_423))
        }

        #[java_method(name = "EMOJI", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EMOJI() -> Result<Object> {
            let __lam_424: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isEmoji(_la0) });
            Ok(Object::from_any(__lam_424))
        }

        #[java_method(name = "EMOJI_PRESENTATION", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EMOJI_PRESENTATION() -> Result<Object> {
            let __lam_425: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isEmojiPresentation(_la0) });
            Ok(Object::from_any(__lam_425))
        }

        #[java_method(name = "EMOJI_MODIFIER", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EMOJI_MODIFIER() -> Result<Object> {
            let __lam_426: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isEmojiModifier(_la0) });
            Ok(Object::from_any(__lam_426))
        }

        #[java_method(name = "EMOJI_MODIFIER_BASE", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EMOJI_MODIFIER_BASE() -> Result<Object> {
            let __lam_427: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isEmojiModifierBase(_la0) });
            Ok(Object::from_any(__lam_427))
        }

        #[java_method(name = "EMOJI_COMPONENT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EMOJI_COMPONENT() -> Result<Object> {
            let __lam_428: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isEmojiComponent(_la0) });
            Ok(Object::from_any(__lam_428))
        }

        #[java_method(name = "EXTENDED_PICTOGRAPHIC", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EXTENDED_PICTOGRAPHIC() -> Result<Object> {
            let __lam_429: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Character::isExtendedPictographic(_la0) });
            Ok(Object::from_any(__lam_429))
        }
    }
}
