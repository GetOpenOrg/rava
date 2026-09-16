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

impl From<Character_UnicodeScript> for Enum<Object> {
    fn from(v: Character_UnicodeScript) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Character$UnicodeScript"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/lang/Character$UnicodeScript;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "Character.java"]
    #[inner_classes     = "java/lang/Character$UnicodeScript:java/lang/Character:UnicodeScript:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Character$UnicodeScript;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable"]

    pub struct Character_UnicodeScript;

    impl Character_UnicodeScript {
        #[cfg_attr(any(), java_field(name = "COMMON", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COMMON:Ljava/lang/Character$UnicodeScript;
        pub fn COMMON() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.COMMON:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN:Ljava/lang/Character$UnicodeScript;
        pub fn LATIN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LATIN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "GREEK", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GREEK:Ljava/lang/Character$UnicodeScript;
        pub fn GREEK() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.GREEK:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CYRILLIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYRILLIC:Ljava/lang/Character$UnicodeScript;
        pub fn CYRILLIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CYRILLIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ARMENIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARMENIAN:Ljava/lang/Character$UnicodeScript;
        pub fn ARMENIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ARMENIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "HEBREW", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HEBREW:Ljava/lang/Character$UnicodeScript;
        pub fn HEBREW() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.HEBREW:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC:Ljava/lang/Character$UnicodeScript;
        pub fn ARABIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ARABIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SYRIAC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SYRIAC:Ljava/lang/Character$UnicodeScript;
        pub fn SYRIAC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SYRIAC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "THAANA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: THAANA:Ljava/lang/Character$UnicodeScript;
        pub fn THAANA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.THAANA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "DEVANAGARI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DEVANAGARI:Ljava/lang/Character$UnicodeScript;
        pub fn DEVANAGARI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.DEVANAGARI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BENGALI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BENGALI:Ljava/lang/Character$UnicodeScript;
        pub fn BENGALI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BENGALI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "GURMUKHI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GURMUKHI:Ljava/lang/Character$UnicodeScript;
        pub fn GURMUKHI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.GURMUKHI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "GUJARATI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GUJARATI:Ljava/lang/Character$UnicodeScript;
        pub fn GUJARATI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.GUJARATI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ORIYA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ORIYA:Ljava/lang/Character$UnicodeScript;
        pub fn ORIYA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ORIYA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TAMIL", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAMIL:Ljava/lang/Character$UnicodeScript;
        pub fn TAMIL() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TAMIL:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TELUGU", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TELUGU:Ljava/lang/Character$UnicodeScript;
        pub fn TELUGU() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TELUGU:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KANNADA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KANNADA:Ljava/lang/Character$UnicodeScript;
        pub fn KANNADA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KANNADA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MALAYALAM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MALAYALAM:Ljava/lang/Character$UnicodeScript;
        pub fn MALAYALAM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MALAYALAM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SINHALA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SINHALA:Ljava/lang/Character$UnicodeScript;
        pub fn SINHALA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SINHALA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "THAI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: THAI:Ljava/lang/Character$UnicodeScript;
        pub fn THAI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.THAI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LAO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LAO:Ljava/lang/Character$UnicodeScript;
        pub fn LAO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LAO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TIBETAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TIBETAN:Ljava/lang/Character$UnicodeScript;
        pub fn TIBETAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TIBETAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MYANMAR", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MYANMAR:Ljava/lang/Character$UnicodeScript;
        pub fn MYANMAR() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MYANMAR:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "GEORGIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GEORGIAN:Ljava/lang/Character$UnicodeScript;
        pub fn GEORGIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.GEORGIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "HANGUL", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANGUL:Ljava/lang/Character$UnicodeScript;
        pub fn HANGUL() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.HANGUL:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ETHIOPIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ETHIOPIC:Ljava/lang/Character$UnicodeScript;
        pub fn ETHIOPIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ETHIOPIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CHEROKEE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHEROKEE:Ljava/lang/Character$UnicodeScript;
        pub fn CHEROKEE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CHEROKEE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CANADIAN_ABORIGINAL", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CANADIAN_ABORIGINAL:Ljava/lang/Character$UnicodeScript;
        pub fn CANADIAN_ABORIGINAL() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CANADIAN_ABORIGINAL:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OGHAM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OGHAM:Ljava/lang/Character$UnicodeScript;
        pub fn OGHAM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OGHAM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "RUNIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: RUNIC:Ljava/lang/Character$UnicodeScript;
        pub fn RUNIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.RUNIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KHMER", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHMER:Ljava/lang/Character$UnicodeScript;
        pub fn KHMER() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KHMER:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MONGOLIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MONGOLIAN:Ljava/lang/Character$UnicodeScript;
        pub fn MONGOLIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MONGOLIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "HIRAGANA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HIRAGANA:Ljava/lang/Character$UnicodeScript;
        pub fn HIRAGANA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.HIRAGANA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KATAKANA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KATAKANA:Ljava/lang/Character$UnicodeScript;
        pub fn KATAKANA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KATAKANA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BOPOMOFO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BOPOMOFO:Ljava/lang/Character$UnicodeScript;
        pub fn BOPOMOFO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BOPOMOFO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "HAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HAN:Ljava/lang/Character$UnicodeScript;
        pub fn HAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.HAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "YI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: YI:Ljava/lang/Character$UnicodeScript;
        pub fn YI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.YI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_ITALIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_ITALIC:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_ITALIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_ITALIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "GOTHIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GOTHIC:Ljava/lang/Character$UnicodeScript;
        pub fn GOTHIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.GOTHIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "DESERET", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DESERET:Ljava/lang/Character$UnicodeScript;
        pub fn DESERET() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.DESERET:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "INHERITED", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INHERITED:Ljava/lang/Character$UnicodeScript;
        pub fn INHERITED() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.INHERITED:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TAGALOG", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAGALOG:Ljava/lang/Character$UnicodeScript;
        pub fn TAGALOG() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TAGALOG:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "HANUNOO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANUNOO:Ljava/lang/Character$UnicodeScript;
        pub fn HANUNOO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.HANUNOO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BUHID", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BUHID:Ljava/lang/Character$UnicodeScript;
        pub fn BUHID() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BUHID:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TAGBANWA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAGBANWA:Ljava/lang/Character$UnicodeScript;
        pub fn TAGBANWA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TAGBANWA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LIMBU", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LIMBU:Ljava/lang/Character$UnicodeScript;
        pub fn LIMBU() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LIMBU:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TAI_LE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAI_LE:Ljava/lang/Character$UnicodeScript;
        pub fn TAI_LE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TAI_LE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LINEAR_B", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LINEAR_B:Ljava/lang/Character$UnicodeScript;
        pub fn LINEAR_B() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LINEAR_B:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "UGARITIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UGARITIC:Ljava/lang/Character$UnicodeScript;
        pub fn UGARITIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.UGARITIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SHAVIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SHAVIAN:Ljava/lang/Character$UnicodeScript;
        pub fn SHAVIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SHAVIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OSMANYA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OSMANYA:Ljava/lang/Character$UnicodeScript;
        pub fn OSMANYA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OSMANYA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CYPRIOT", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYPRIOT:Ljava/lang/Character$UnicodeScript;
        pub fn CYPRIOT() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CYPRIOT:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BRAILLE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BRAILLE:Ljava/lang/Character$UnicodeScript;
        pub fn BRAILLE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BRAILLE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BUGINESE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BUGINESE:Ljava/lang/Character$UnicodeScript;
        pub fn BUGINESE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BUGINESE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "COPTIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COPTIC:Ljava/lang/Character$UnicodeScript;
        pub fn COPTIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.COPTIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "NEW_TAI_LUE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NEW_TAI_LUE:Ljava/lang/Character$UnicodeScript;
        pub fn NEW_TAI_LUE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.NEW_TAI_LUE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "GLAGOLITIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GLAGOLITIC:Ljava/lang/Character$UnicodeScript;
        pub fn GLAGOLITIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.GLAGOLITIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TIFINAGH", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TIFINAGH:Ljava/lang/Character$UnicodeScript;
        pub fn TIFINAGH() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TIFINAGH:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SYLOTI_NAGRI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SYLOTI_NAGRI:Ljava/lang/Character$UnicodeScript;
        pub fn SYLOTI_NAGRI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SYLOTI_NAGRI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_PERSIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_PERSIAN:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_PERSIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_PERSIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KHAROSHTHI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHAROSHTHI:Ljava/lang/Character$UnicodeScript;
        pub fn KHAROSHTHI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KHAROSHTHI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BALINESE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BALINESE:Ljava/lang/Character$UnicodeScript;
        pub fn BALINESE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BALINESE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CUNEIFORM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CUNEIFORM:Ljava/lang/Character$UnicodeScript;
        pub fn CUNEIFORM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CUNEIFORM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "PHOENICIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PHOENICIAN:Ljava/lang/Character$UnicodeScript;
        pub fn PHOENICIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.PHOENICIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "PHAGS_PA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PHAGS_PA:Ljava/lang/Character$UnicodeScript;
        pub fn PHAGS_PA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.PHAGS_PA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "NKO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NKO:Ljava/lang/Character$UnicodeScript;
        pub fn NKO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.NKO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SUNDANESE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUNDANESE:Ljava/lang/Character$UnicodeScript;
        pub fn SUNDANESE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SUNDANESE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BATAK", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BATAK:Ljava/lang/Character$UnicodeScript;
        pub fn BATAK() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BATAK:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LEPCHA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LEPCHA:Ljava/lang/Character$UnicodeScript;
        pub fn LEPCHA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LEPCHA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OL_CHIKI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OL_CHIKI:Ljava/lang/Character$UnicodeScript;
        pub fn OL_CHIKI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OL_CHIKI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "VAI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VAI:Ljava/lang/Character$UnicodeScript;
        pub fn VAI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.VAI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SAURASHTRA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SAURASHTRA:Ljava/lang/Character$UnicodeScript;
        pub fn SAURASHTRA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SAURASHTRA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KAYAH_LI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KAYAH_LI:Ljava/lang/Character$UnicodeScript;
        pub fn KAYAH_LI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KAYAH_LI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "REJANG", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: REJANG:Ljava/lang/Character$UnicodeScript;
        pub fn REJANG() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.REJANG:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LYCIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LYCIAN:Ljava/lang/Character$UnicodeScript;
        pub fn LYCIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LYCIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CARIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CARIAN:Ljava/lang/Character$UnicodeScript;
        pub fn CARIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CARIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LYDIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LYDIAN:Ljava/lang/Character$UnicodeScript;
        pub fn LYDIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LYDIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CHAM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHAM:Ljava/lang/Character$UnicodeScript;
        pub fn CHAM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CHAM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TAI_THAM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAI_THAM:Ljava/lang/Character$UnicodeScript;
        pub fn TAI_THAM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TAI_THAM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TAI_VIET", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAI_VIET:Ljava/lang/Character$UnicodeScript;
        pub fn TAI_VIET() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TAI_VIET:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "AVESTAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: AVESTAN:Ljava/lang/Character$UnicodeScript;
        pub fn AVESTAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.AVESTAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "EGYPTIAN_HIEROGLYPHS", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EGYPTIAN_HIEROGLYPHS:Ljava/lang/Character$UnicodeScript;
        pub fn EGYPTIAN_HIEROGLYPHS() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.EGYPTIAN_HIEROGLYPHS:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SAMARITAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SAMARITAN:Ljava/lang/Character$UnicodeScript;
        pub fn SAMARITAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SAMARITAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MANDAIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MANDAIC:Ljava/lang/Character$UnicodeScript;
        pub fn MANDAIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MANDAIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LISU", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LISU:Ljava/lang/Character$UnicodeScript;
        pub fn LISU() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LISU:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BAMUM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BAMUM:Ljava/lang/Character$UnicodeScript;
        pub fn BAMUM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BAMUM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "JAVANESE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: JAVANESE:Ljava/lang/Character$UnicodeScript;
        pub fn JAVANESE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.JAVANESE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MEETEI_MAYEK", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEETEI_MAYEK:Ljava/lang/Character$UnicodeScript;
        pub fn MEETEI_MAYEK() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MEETEI_MAYEK:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "IMPERIAL_ARAMAIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: IMPERIAL_ARAMAIC:Ljava/lang/Character$UnicodeScript;
        pub fn IMPERIAL_ARAMAIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.IMPERIAL_ARAMAIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_SOUTH_ARABIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_SOUTH_ARABIAN:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_SOUTH_ARABIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_SOUTH_ARABIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "INSCRIPTIONAL_PARTHIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INSCRIPTIONAL_PARTHIAN:Ljava/lang/Character$UnicodeScript;
        pub fn INSCRIPTIONAL_PARTHIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.INSCRIPTIONAL_PARTHIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "INSCRIPTIONAL_PAHLAVI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INSCRIPTIONAL_PAHLAVI:Ljava/lang/Character$UnicodeScript;
        pub fn INSCRIPTIONAL_PAHLAVI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.INSCRIPTIONAL_PAHLAVI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_TURKIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_TURKIC:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_TURKIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_TURKIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BRAHMI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BRAHMI:Ljava/lang/Character$UnicodeScript;
        pub fn BRAHMI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BRAHMI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KAITHI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KAITHI:Ljava/lang/Character$UnicodeScript;
        pub fn KAITHI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KAITHI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MEROITIC_HIEROGLYPHS", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEROITIC_HIEROGLYPHS:Ljava/lang/Character$UnicodeScript;
        pub fn MEROITIC_HIEROGLYPHS() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MEROITIC_HIEROGLYPHS:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MEROITIC_CURSIVE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEROITIC_CURSIVE:Ljava/lang/Character$UnicodeScript;
        pub fn MEROITIC_CURSIVE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MEROITIC_CURSIVE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SORA_SOMPENG", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SORA_SOMPENG:Ljava/lang/Character$UnicodeScript;
        pub fn SORA_SOMPENG() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SORA_SOMPENG:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CHAKMA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHAKMA:Ljava/lang/Character$UnicodeScript;
        pub fn CHAKMA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CHAKMA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SHARADA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SHARADA:Ljava/lang/Character$UnicodeScript;
        pub fn SHARADA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SHARADA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TAKRI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAKRI:Ljava/lang/Character$UnicodeScript;
        pub fn TAKRI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TAKRI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MIAO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MIAO:Ljava/lang/Character$UnicodeScript;
        pub fn MIAO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MIAO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CAUCASIAN_ALBANIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CAUCASIAN_ALBANIAN:Ljava/lang/Character$UnicodeScript;
        pub fn CAUCASIAN_ALBANIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CAUCASIAN_ALBANIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BASSA_VAH", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BASSA_VAH:Ljava/lang/Character$UnicodeScript;
        pub fn BASSA_VAH() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BASSA_VAH:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "DUPLOYAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DUPLOYAN:Ljava/lang/Character$UnicodeScript;
        pub fn DUPLOYAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.DUPLOYAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ELBASAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ELBASAN:Ljava/lang/Character$UnicodeScript;
        pub fn ELBASAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ELBASAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "GRANTHA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GRANTHA:Ljava/lang/Character$UnicodeScript;
        pub fn GRANTHA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.GRANTHA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "PAHAWH_HMONG", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PAHAWH_HMONG:Ljava/lang/Character$UnicodeScript;
        pub fn PAHAWH_HMONG() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.PAHAWH_HMONG:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KHOJKI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHOJKI:Ljava/lang/Character$UnicodeScript;
        pub fn KHOJKI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KHOJKI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "LINEAR_A", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LINEAR_A:Ljava/lang/Character$UnicodeScript;
        pub fn LINEAR_A() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.LINEAR_A:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MAHAJANI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAHAJANI:Ljava/lang/Character$UnicodeScript;
        pub fn MAHAJANI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MAHAJANI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MANICHAEAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MANICHAEAN:Ljava/lang/Character$UnicodeScript;
        pub fn MANICHAEAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MANICHAEAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MENDE_KIKAKUI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MENDE_KIKAKUI:Ljava/lang/Character$UnicodeScript;
        pub fn MENDE_KIKAKUI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MENDE_KIKAKUI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MODI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MODI:Ljava/lang/Character$UnicodeScript;
        pub fn MODI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MODI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MRO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MRO:Ljava/lang/Character$UnicodeScript;
        pub fn MRO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MRO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_NORTH_ARABIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_NORTH_ARABIAN:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_NORTH_ARABIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_NORTH_ARABIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "NABATAEAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NABATAEAN:Ljava/lang/Character$UnicodeScript;
        pub fn NABATAEAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.NABATAEAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "PALMYRENE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PALMYRENE:Ljava/lang/Character$UnicodeScript;
        pub fn PALMYRENE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.PALMYRENE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "PAU_CIN_HAU", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PAU_CIN_HAU:Ljava/lang/Character$UnicodeScript;
        pub fn PAU_CIN_HAU() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.PAU_CIN_HAU:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_PERMIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_PERMIC:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_PERMIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_PERMIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "PSALTER_PAHLAVI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PSALTER_PAHLAVI:Ljava/lang/Character$UnicodeScript;
        pub fn PSALTER_PAHLAVI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.PSALTER_PAHLAVI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SIDDHAM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SIDDHAM:Ljava/lang/Character$UnicodeScript;
        pub fn SIDDHAM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SIDDHAM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KHUDAWADI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHUDAWADI:Ljava/lang/Character$UnicodeScript;
        pub fn KHUDAWADI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KHUDAWADI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TIRHUTA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TIRHUTA:Ljava/lang/Character$UnicodeScript;
        pub fn TIRHUTA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TIRHUTA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "WARANG_CITI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: WARANG_CITI:Ljava/lang/Character$UnicodeScript;
        pub fn WARANG_CITI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.WARANG_CITI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "AHOM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: AHOM:Ljava/lang/Character$UnicodeScript;
        pub fn AHOM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.AHOM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ANATOLIAN_HIEROGLYPHS", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ANATOLIAN_HIEROGLYPHS:Ljava/lang/Character$UnicodeScript;
        pub fn ANATOLIAN_HIEROGLYPHS() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ANATOLIAN_HIEROGLYPHS:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "HATRAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HATRAN:Ljava/lang/Character$UnicodeScript;
        pub fn HATRAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.HATRAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MULTANI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MULTANI:Ljava/lang/Character$UnicodeScript;
        pub fn MULTANI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MULTANI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_HUNGARIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_HUNGARIAN:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_HUNGARIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_HUNGARIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SIGNWRITING", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SIGNWRITING:Ljava/lang/Character$UnicodeScript;
        pub fn SIGNWRITING() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SIGNWRITING:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ADLAM", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ADLAM:Ljava/lang/Character$UnicodeScript;
        pub fn ADLAM() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ADLAM:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "BHAIKSUKI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BHAIKSUKI:Ljava/lang/Character$UnicodeScript;
        pub fn BHAIKSUKI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.BHAIKSUKI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MARCHEN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MARCHEN:Ljava/lang/Character$UnicodeScript;
        pub fn MARCHEN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MARCHEN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "NEWA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NEWA:Ljava/lang/Character$UnicodeScript;
        pub fn NEWA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.NEWA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OSAGE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OSAGE:Ljava/lang/Character$UnicodeScript;
        pub fn OSAGE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OSAGE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TANGUT", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TANGUT:Ljava/lang/Character$UnicodeScript;
        pub fn TANGUT() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TANGUT:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MASARAM_GONDI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MASARAM_GONDI:Ljava/lang/Character$UnicodeScript;
        pub fn MASARAM_GONDI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MASARAM_GONDI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "NUSHU", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NUSHU:Ljava/lang/Character$UnicodeScript;
        pub fn NUSHU() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.NUSHU:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SOYOMBO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SOYOMBO:Ljava/lang/Character$UnicodeScript;
        pub fn SOYOMBO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SOYOMBO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ZANABAZAR_SQUARE", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ZANABAZAR_SQUARE:Ljava/lang/Character$UnicodeScript;
        pub fn ZANABAZAR_SQUARE() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ZANABAZAR_SQUARE:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "HANIFI_ROHINGYA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANIFI_ROHINGYA:Ljava/lang/Character$UnicodeScript;
        pub fn HANIFI_ROHINGYA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.HANIFI_ROHINGYA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_SOGDIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_SOGDIAN:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_SOGDIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_SOGDIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "SOGDIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SOGDIAN:Ljava/lang/Character$UnicodeScript;
        pub fn SOGDIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.SOGDIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "DOGRA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DOGRA:Ljava/lang/Character$UnicodeScript;
        pub fn DOGRA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.DOGRA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "GUNJALA_GONDI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GUNJALA_GONDI:Ljava/lang/Character$UnicodeScript;
        pub fn GUNJALA_GONDI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.GUNJALA_GONDI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MAKASAR", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAKASAR:Ljava/lang/Character$UnicodeScript;
        pub fn MAKASAR() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MAKASAR:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "MEDEFAIDRIN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEDEFAIDRIN:Ljava/lang/Character$UnicodeScript;
        pub fn MEDEFAIDRIN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.MEDEFAIDRIN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "ELYMAIC", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ELYMAIC:Ljava/lang/Character$UnicodeScript;
        pub fn ELYMAIC() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.ELYMAIC:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "NANDINAGARI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NANDINAGARI:Ljava/lang/Character$UnicodeScript;
        pub fn NANDINAGARI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.NANDINAGARI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "NYIAKENG_PUACHUE_HMONG", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NYIAKENG_PUACHUE_HMONG:Ljava/lang/Character$UnicodeScript;
        pub fn NYIAKENG_PUACHUE_HMONG() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.NYIAKENG_PUACHUE_HMONG:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "WANCHO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: WANCHO:Ljava/lang/Character$UnicodeScript;
        pub fn WANCHO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.WANCHO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "YEZIDI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: YEZIDI:Ljava/lang/Character$UnicodeScript;
        pub fn YEZIDI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.YEZIDI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CHORASMIAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHORASMIAN:Ljava/lang/Character$UnicodeScript;
        pub fn CHORASMIAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CHORASMIAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "DIVES_AKURU", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DIVES_AKURU:Ljava/lang/Character$UnicodeScript;
        pub fn DIVES_AKURU() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.DIVES_AKURU:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KHITAN_SMALL_SCRIPT", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHITAN_SMALL_SCRIPT:Ljava/lang/Character$UnicodeScript;
        pub fn KHITAN_SMALL_SCRIPT() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KHITAN_SMALL_SCRIPT:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "VITHKUQI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VITHKUQI:Ljava/lang/Character$UnicodeScript;
        pub fn VITHKUQI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.VITHKUQI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_UYGHUR", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_UYGHUR:Ljava/lang/Character$UnicodeScript;
        pub fn OLD_UYGHUR() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.OLD_UYGHUR:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "CYPRO_MINOAN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYPRO_MINOAN:Ljava/lang/Character$UnicodeScript;
        pub fn CYPRO_MINOAN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.CYPRO_MINOAN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TANGSA", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TANGSA:Ljava/lang/Character$UnicodeScript;
        pub fn TANGSA() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TANGSA:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "TOTO", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TOTO:Ljava/lang/Character$UnicodeScript;
        pub fn TOTO() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.TOTO:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "KAWI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KAWI:Ljava/lang/Character$UnicodeScript;
        pub fn KAWI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.KAWI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "NAG_MUNDARI", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NAG_MUNDARI:Ljava/lang/Character$UnicodeScript;
        pub fn NAG_MUNDARI() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.NAG_MUNDARI:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "UNKNOWN", descriptor = "Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UNKNOWN:Ljava/lang/Character$UnicodeScript;
        pub fn UNKNOWN() -> Character_UnicodeScript {
            panic!("stub: java/lang/Character$UnicodeScript.UNKNOWN:Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "scriptStarts", descriptor = "[I", access = "private", modifiers = "static final", is_static = true))]
        // static field: scriptStarts:[I
        pub fn scriptStarts() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/lang/Character$UnicodeScript.scriptStarts:[I")
        }

        #[cfg_attr(any(), java_field(name = "scripts", descriptor = "[Ljava/lang/Character$UnicodeScript;", access = "private", modifiers = "static final", is_static = true))]
        // static field: scripts:[Ljava/lang/Character$UnicodeScript;
        pub fn scripts() -> Rc<RefCell<Vec<Character_UnicodeScript>>> {
            panic!("stub: java/lang/Character$UnicodeScript.scripts:[Ljava/lang/Character$UnicodeScript;")
        }

        #[cfg_attr(any(), java_field(name = "aliases", descriptor = "Ljava/util/HashMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/HashMap<Ljava/lang/String;Ljava/lang/Character$UnicodeScript;>;"))]
        // static field: aliases:Ljava/util/HashMap;
        pub fn aliases() -> HashMap<String, Character_UnicodeScript> {
            panic!("stub: java/lang/Character$UnicodeScript.aliases:Ljava/util/HashMap;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/lang/Character$UnicodeScript;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/lang/Character$UnicodeScript;
        pub fn _VALUES() -> Rc<RefCell<Vec<Character_UnicodeScript>>> {
            panic!("stub: java/lang/Character$UnicodeScript.$VALUES:[Ljava/lang/Character$UnicodeScript;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<Character_UnicodeScript>>>> {
            panic!("stub: java/lang/Character$UnicodeScript.values:()[Ljava/lang/Character$UnicodeScript;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(mut name: String) -> Result<Character_UnicodeScript> {
            let _t0: Enum<Object> = Enum::<Object>::valueOf(Default::default(), Clone::clone(&name))?;
            Ok(Default::default())
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096")]
        pub fn new(arg0: String, arg1: i32) -> Result<Self> {
            panic!("stub: java/lang/Character$UnicodeScript.<init>:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "of", descriptor = "(I)Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(codePoint: i32) -> Result<Character_UnicodeScript> {
            panic!("stub: java/lang/Character$UnicodeScript.of:(I)Ljava/lang/Character$UnicodeScript;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;)Ljava/lang/Character$UnicodeScript;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forName(mut scriptName: String) -> Result<Character_UnicodeScript> {
            let _t0 = scriptName.toUpperCase_locale(Clone::clone(&Locale::ENGLISH()))?;
            scriptName = _t0;
            let _t1 = Character_UnicodeScript::aliases().get(Object::from_any(scriptName.clone()))?;
            let mut sc = (_t1).downcast::<Character_UnicodeScript>();
            if !_is_jnull(&sc) {
                return Ok(sc);
            }
            let _t2: Character_UnicodeScript = Character_UnicodeScript::valueOf(Clone::clone(&scriptName))?;
            Ok(_t2)
        }
    }
}
