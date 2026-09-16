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

impl From<Character_UnicodeBlock> for Character_Subset {
    fn from(v: Character_UnicodeBlock) -> Character_Subset { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Character$UnicodeBlock"]
    #[super_class       = "java/lang/Character$Subset"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Character.java"]
    #[inner_classes     = "java/lang/Character$Subset:java/lang/Character:Subset:9;java/lang/Character$UnicodeBlock:java/lang/Character:UnicodeBlock:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Character_Subset"]
    #[superclass_fields(name: String)]
    #[all_supertypes    = "java/lang/Character$Subset;java/lang/Character$UnicodeBlock;java/lang/Object"]

    pub struct Character_UnicodeBlock;

    impl Character_UnicodeBlock {
        #[cfg_attr(any(), java_field(name = "NUM_ENTITIES", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "756"))]
        // static field: NUM_ENTITIES:I
        pub fn NUM_ENTITIES() -> i32 {
            756
        }

        #[cfg_attr(any(), java_field(name = "map", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/Character$UnicodeBlock;>;"))]
        // static field: map:Ljava/util/Map;
        pub fn map() -> Object {
            panic!("stub: java/lang/Character$UnicodeBlock.map:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "BASIC_LATIN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BASIC_LATIN:Ljava/lang/Character$UnicodeBlock;
        pub fn BASIC_LATIN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BASIC_LATIN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_1_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_1_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_1_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_1_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_EXTENDED_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_EXTENDED_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "IPA_EXTENSIONS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: IPA_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;
        pub fn IPA_EXTENSIONS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.IPA_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SPACING_MODIFIER_LETTERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SPACING_MODIFIER_LETTERS:Ljava/lang/Character$UnicodeBlock;
        pub fn SPACING_MODIFIER_LETTERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SPACING_MODIFIER_LETTERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COMBINING_DIACRITICAL_MARKS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COMBINING_DIACRITICAL_MARKS:Ljava/lang/Character$UnicodeBlock;
        pub fn COMBINING_DIACRITICAL_MARKS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COMBINING_DIACRITICAL_MARKS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GREEK", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GREEK:Ljava/lang/Character$UnicodeBlock;
        pub fn GREEK() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GREEK:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CYRILLIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYRILLIC:Ljava/lang/Character$UnicodeBlock;
        pub fn CYRILLIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CYRILLIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARMENIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARMENIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn ARMENIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARMENIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HEBREW", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HEBREW:Ljava/lang/Character$UnicodeBlock;
        pub fn HEBREW() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HEBREW:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC:Ljava/lang/Character$UnicodeBlock;
        pub fn ARABIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARABIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DEVANAGARI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DEVANAGARI:Ljava/lang/Character$UnicodeBlock;
        pub fn DEVANAGARI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DEVANAGARI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BENGALI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BENGALI:Ljava/lang/Character$UnicodeBlock;
        pub fn BENGALI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BENGALI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GURMUKHI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GURMUKHI:Ljava/lang/Character$UnicodeBlock;
        pub fn GURMUKHI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GURMUKHI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GUJARATI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GUJARATI:Ljava/lang/Character$UnicodeBlock;
        pub fn GUJARATI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GUJARATI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ORIYA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ORIYA:Ljava/lang/Character$UnicodeBlock;
        pub fn ORIYA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ORIYA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAMIL", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAMIL:Ljava/lang/Character$UnicodeBlock;
        pub fn TAMIL() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAMIL:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TELUGU", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TELUGU:Ljava/lang/Character$UnicodeBlock;
        pub fn TELUGU() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TELUGU:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KANNADA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KANNADA:Ljava/lang/Character$UnicodeBlock;
        pub fn KANNADA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KANNADA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MALAYALAM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MALAYALAM:Ljava/lang/Character$UnicodeBlock;
        pub fn MALAYALAM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MALAYALAM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "THAI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: THAI:Ljava/lang/Character$UnicodeBlock;
        pub fn THAI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.THAI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LAO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LAO:Ljava/lang/Character$UnicodeBlock;
        pub fn LAO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LAO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TIBETAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TIBETAN:Ljava/lang/Character$UnicodeBlock;
        pub fn TIBETAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TIBETAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GEORGIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GEORGIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn GEORGIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GEORGIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HANGUL_JAMO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANGUL_JAMO:Ljava/lang/Character$UnicodeBlock;
        pub fn HANGUL_JAMO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HANGUL_JAMO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_EXTENDED_ADDITIONAL", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_EXTENDED_ADDITIONAL:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_EXTENDED_ADDITIONAL() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_EXTENDED_ADDITIONAL:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GREEK_EXTENDED", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GREEK_EXTENDED:Ljava/lang/Character$UnicodeBlock;
        pub fn GREEK_EXTENDED() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GREEK_EXTENDED:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GENERAL_PUNCTUATION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GENERAL_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;
        pub fn GENERAL_PUNCTUATION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GENERAL_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPERSCRIPTS_AND_SUBSCRIPTS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPERSCRIPTS_AND_SUBSCRIPTS:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPERSCRIPTS_AND_SUBSCRIPTS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPERSCRIPTS_AND_SUBSCRIPTS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CURRENCY_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CURRENCY_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn CURRENCY_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CURRENCY_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COMBINING_MARKS_FOR_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COMBINING_MARKS_FOR_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn COMBINING_MARKS_FOR_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COMBINING_MARKS_FOR_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LETTERLIKE_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LETTERLIKE_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn LETTERLIKE_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LETTERLIKE_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NUMBER_FORMS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NUMBER_FORMS:Ljava/lang/Character$UnicodeBlock;
        pub fn NUMBER_FORMS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NUMBER_FORMS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARROWS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARROWS:Ljava/lang/Character$UnicodeBlock;
        pub fn ARROWS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARROWS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MATHEMATICAL_OPERATORS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MATHEMATICAL_OPERATORS:Ljava/lang/Character$UnicodeBlock;
        pub fn MATHEMATICAL_OPERATORS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MATHEMATICAL_OPERATORS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MISCELLANEOUS_TECHNICAL", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MISCELLANEOUS_TECHNICAL:Ljava/lang/Character$UnicodeBlock;
        pub fn MISCELLANEOUS_TECHNICAL() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MISCELLANEOUS_TECHNICAL:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CONTROL_PICTURES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CONTROL_PICTURES:Ljava/lang/Character$UnicodeBlock;
        pub fn CONTROL_PICTURES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CONTROL_PICTURES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OPTICAL_CHARACTER_RECOGNITION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OPTICAL_CHARACTER_RECOGNITION:Ljava/lang/Character$UnicodeBlock;
        pub fn OPTICAL_CHARACTER_RECOGNITION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OPTICAL_CHARACTER_RECOGNITION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ENCLOSED_ALPHANUMERICS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ENCLOSED_ALPHANUMERICS:Ljava/lang/Character$UnicodeBlock;
        pub fn ENCLOSED_ALPHANUMERICS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ENCLOSED_ALPHANUMERICS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BOX_DRAWING", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BOX_DRAWING:Ljava/lang/Character$UnicodeBlock;
        pub fn BOX_DRAWING() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BOX_DRAWING:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BLOCK_ELEMENTS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BLOCK_ELEMENTS:Ljava/lang/Character$UnicodeBlock;
        pub fn BLOCK_ELEMENTS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BLOCK_ELEMENTS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GEOMETRIC_SHAPES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GEOMETRIC_SHAPES:Ljava/lang/Character$UnicodeBlock;
        pub fn GEOMETRIC_SHAPES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GEOMETRIC_SHAPES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MISCELLANEOUS_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MISCELLANEOUS_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn MISCELLANEOUS_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MISCELLANEOUS_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DINGBATS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DINGBATS:Ljava/lang/Character$UnicodeBlock;
        pub fn DINGBATS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DINGBATS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_SYMBOLS_AND_PUNCTUATION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_SYMBOLS_AND_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_SYMBOLS_AND_PUNCTUATION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_SYMBOLS_AND_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HIRAGANA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HIRAGANA:Ljava/lang/Character$UnicodeBlock;
        pub fn HIRAGANA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HIRAGANA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KATAKANA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KATAKANA:Ljava/lang/Character$UnicodeBlock;
        pub fn KATAKANA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KATAKANA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BOPOMOFO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BOPOMOFO:Ljava/lang/Character$UnicodeBlock;
        pub fn BOPOMOFO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BOPOMOFO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HANGUL_COMPATIBILITY_JAMO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANGUL_COMPATIBILITY_JAMO:Ljava/lang/Character$UnicodeBlock;
        pub fn HANGUL_COMPATIBILITY_JAMO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HANGUL_COMPATIBILITY_JAMO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KANBUN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KANBUN:Ljava/lang/Character$UnicodeBlock;
        pub fn KANBUN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KANBUN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ENCLOSED_CJK_LETTERS_AND_MONTHS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ENCLOSED_CJK_LETTERS_AND_MONTHS:Ljava/lang/Character$UnicodeBlock;
        pub fn ENCLOSED_CJK_LETTERS_AND_MONTHS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ENCLOSED_CJK_LETTERS_AND_MONTHS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_COMPATIBILITY", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_COMPATIBILITY:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_COMPATIBILITY() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_COMPATIBILITY:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HANGUL_SYLLABLES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANGUL_SYLLABLES:Ljava/lang/Character$UnicodeBlock;
        pub fn HANGUL_SYLLABLES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HANGUL_SYLLABLES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PRIVATE_USE_AREA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PRIVATE_USE_AREA:Ljava/lang/Character$UnicodeBlock;
        pub fn PRIVATE_USE_AREA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PRIVATE_USE_AREA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_COMPATIBILITY_IDEOGRAPHS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_COMPATIBILITY_IDEOGRAPHS:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_COMPATIBILITY_IDEOGRAPHS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_COMPATIBILITY_IDEOGRAPHS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ALPHABETIC_PRESENTATION_FORMS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ALPHABETIC_PRESENTATION_FORMS:Ljava/lang/Character$UnicodeBlock;
        pub fn ALPHABETIC_PRESENTATION_FORMS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ALPHABETIC_PRESENTATION_FORMS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC_PRESENTATION_FORMS_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC_PRESENTATION_FORMS_A:Ljava/lang/Character$UnicodeBlock;
        pub fn ARABIC_PRESENTATION_FORMS_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARABIC_PRESENTATION_FORMS_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COMBINING_HALF_MARKS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COMBINING_HALF_MARKS:Ljava/lang/Character$UnicodeBlock;
        pub fn COMBINING_HALF_MARKS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COMBINING_HALF_MARKS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_COMPATIBILITY_FORMS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_COMPATIBILITY_FORMS:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_COMPATIBILITY_FORMS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_COMPATIBILITY_FORMS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SMALL_FORM_VARIANTS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SMALL_FORM_VARIANTS:Ljava/lang/Character$UnicodeBlock;
        pub fn SMALL_FORM_VARIANTS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SMALL_FORM_VARIANTS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC_PRESENTATION_FORMS_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC_PRESENTATION_FORMS_B:Ljava/lang/Character$UnicodeBlock;
        pub fn ARABIC_PRESENTATION_FORMS_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARABIC_PRESENTATION_FORMS_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HALFWIDTH_AND_FULLWIDTH_FORMS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HALFWIDTH_AND_FULLWIDTH_FORMS:Ljava/lang/Character$UnicodeBlock;
        pub fn HALFWIDTH_AND_FULLWIDTH_FORMS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HALFWIDTH_AND_FULLWIDTH_FORMS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SPECIALS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SPECIALS:Ljava/lang/Character$UnicodeBlock;
        pub fn SPECIALS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SPECIALS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SURROGATES_AREA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true, is_deprecated = true))]
        // static field: SURROGATES_AREA:Ljava/lang/Character$UnicodeBlock;
        pub fn SURROGATES_AREA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SURROGATES_AREA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SYRIAC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SYRIAC:Ljava/lang/Character$UnicodeBlock;
        pub fn SYRIAC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SYRIAC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "THAANA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: THAANA:Ljava/lang/Character$UnicodeBlock;
        pub fn THAANA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.THAANA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SINHALA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SINHALA:Ljava/lang/Character$UnicodeBlock;
        pub fn SINHALA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SINHALA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MYANMAR", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MYANMAR:Ljava/lang/Character$UnicodeBlock;
        pub fn MYANMAR() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MYANMAR:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ETHIOPIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ETHIOPIC:Ljava/lang/Character$UnicodeBlock;
        pub fn ETHIOPIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ETHIOPIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CHEROKEE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHEROKEE:Ljava/lang/Character$UnicodeBlock;
        pub fn CHEROKEE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CHEROKEE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS:Ljava/lang/Character$UnicodeBlock;
        pub fn UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OGHAM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OGHAM:Ljava/lang/Character$UnicodeBlock;
        pub fn OGHAM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OGHAM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "RUNIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: RUNIC:Ljava/lang/Character$UnicodeBlock;
        pub fn RUNIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.RUNIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KHMER", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHMER:Ljava/lang/Character$UnicodeBlock;
        pub fn KHMER() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KHMER:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MONGOLIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MONGOLIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn MONGOLIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MONGOLIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BRAILLE_PATTERNS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BRAILLE_PATTERNS:Ljava/lang/Character$UnicodeBlock;
        pub fn BRAILLE_PATTERNS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BRAILLE_PATTERNS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_RADICALS_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_RADICALS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_RADICALS_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_RADICALS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KANGXI_RADICALS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KANGXI_RADICALS:Ljava/lang/Character$UnicodeBlock;
        pub fn KANGXI_RADICALS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KANGXI_RADICALS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "IDEOGRAPHIC_DESCRIPTION_CHARACTERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: IDEOGRAPHIC_DESCRIPTION_CHARACTERS:Ljava/lang/Character$UnicodeBlock;
        pub fn IDEOGRAPHIC_DESCRIPTION_CHARACTERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.IDEOGRAPHIC_DESCRIPTION_CHARACTERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BOPOMOFO_EXTENDED", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BOPOMOFO_EXTENDED:Ljava/lang/Character$UnicodeBlock;
        pub fn BOPOMOFO_EXTENDED() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BOPOMOFO_EXTENDED:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "YI_SYLLABLES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: YI_SYLLABLES:Ljava/lang/Character$UnicodeBlock;
        pub fn YI_SYLLABLES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.YI_SYLLABLES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "YI_RADICALS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: YI_RADICALS:Ljava/lang/Character$UnicodeBlock;
        pub fn YI_RADICALS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.YI_RADICALS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CYRILLIC_SUPPLEMENTARY", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYRILLIC_SUPPLEMENTARY:Ljava/lang/Character$UnicodeBlock;
        pub fn CYRILLIC_SUPPLEMENTARY() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CYRILLIC_SUPPLEMENTARY:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAGALOG", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAGALOG:Ljava/lang/Character$UnicodeBlock;
        pub fn TAGALOG() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAGALOG:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HANUNOO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANUNOO:Ljava/lang/Character$UnicodeBlock;
        pub fn HANUNOO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HANUNOO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BUHID", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BUHID:Ljava/lang/Character$UnicodeBlock;
        pub fn BUHID() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BUHID:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAGBANWA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAGBANWA:Ljava/lang/Character$UnicodeBlock;
        pub fn TAGBANWA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAGBANWA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LIMBU", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LIMBU:Ljava/lang/Character$UnicodeBlock;
        pub fn LIMBU() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LIMBU:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAI_LE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAI_LE:Ljava/lang/Character$UnicodeBlock;
        pub fn TAI_LE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAI_LE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KHMER_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHMER_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn KHMER_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KHMER_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PHONETIC_EXTENSIONS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PHONETIC_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;
        pub fn PHONETIC_EXTENSIONS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PHONETIC_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A:Ljava/lang/Character$UnicodeBlock;
        pub fn MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPPLEMENTAL_ARROWS_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPPLEMENTAL_ARROWS_A:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPPLEMENTAL_ARROWS_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPPLEMENTAL_ARROWS_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPPLEMENTAL_ARROWS_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPPLEMENTAL_ARROWS_B:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPPLEMENTAL_ARROWS_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPPLEMENTAL_ARROWS_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B:Ljava/lang/Character$UnicodeBlock;
        pub fn MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPPLEMENTAL_MATHEMATICAL_OPERATORS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPPLEMENTAL_MATHEMATICAL_OPERATORS:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPPLEMENTAL_MATHEMATICAL_OPERATORS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPPLEMENTAL_MATHEMATICAL_OPERATORS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MISCELLANEOUS_SYMBOLS_AND_ARROWS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MISCELLANEOUS_SYMBOLS_AND_ARROWS:Ljava/lang/Character$UnicodeBlock;
        pub fn MISCELLANEOUS_SYMBOLS_AND_ARROWS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MISCELLANEOUS_SYMBOLS_AND_ARROWS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KATAKANA_PHONETIC_EXTENSIONS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KATAKANA_PHONETIC_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;
        pub fn KATAKANA_PHONETIC_EXTENSIONS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KATAKANA_PHONETIC_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "YIJING_HEXAGRAM_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: YIJING_HEXAGRAM_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn YIJING_HEXAGRAM_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.YIJING_HEXAGRAM_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "VARIATION_SELECTORS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VARIATION_SELECTORS:Ljava/lang/Character$UnicodeBlock;
        pub fn VARIATION_SELECTORS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.VARIATION_SELECTORS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LINEAR_B_SYLLABARY", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LINEAR_B_SYLLABARY:Ljava/lang/Character$UnicodeBlock;
        pub fn LINEAR_B_SYLLABARY() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LINEAR_B_SYLLABARY:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LINEAR_B_IDEOGRAMS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LINEAR_B_IDEOGRAMS:Ljava/lang/Character$UnicodeBlock;
        pub fn LINEAR_B_IDEOGRAMS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LINEAR_B_IDEOGRAMS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "AEGEAN_NUMBERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: AEGEAN_NUMBERS:Ljava/lang/Character$UnicodeBlock;
        pub fn AEGEAN_NUMBERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.AEGEAN_NUMBERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_ITALIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_ITALIC:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_ITALIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_ITALIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GOTHIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GOTHIC:Ljava/lang/Character$UnicodeBlock;
        pub fn GOTHIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GOTHIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "UGARITIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UGARITIC:Ljava/lang/Character$UnicodeBlock;
        pub fn UGARITIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.UGARITIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DESERET", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DESERET:Ljava/lang/Character$UnicodeBlock;
        pub fn DESERET() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DESERET:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SHAVIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SHAVIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn SHAVIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SHAVIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OSMANYA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OSMANYA:Ljava/lang/Character$UnicodeBlock;
        pub fn OSMANYA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OSMANYA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CYPRIOT_SYLLABARY", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYPRIOT_SYLLABARY:Ljava/lang/Character$UnicodeBlock;
        pub fn CYPRIOT_SYLLABARY() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CYPRIOT_SYLLABARY:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BYZANTINE_MUSICAL_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BYZANTINE_MUSICAL_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn BYZANTINE_MUSICAL_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BYZANTINE_MUSICAL_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MUSICAL_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MUSICAL_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn MUSICAL_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MUSICAL_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAI_XUAN_JING_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAI_XUAN_JING_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn TAI_XUAN_JING_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAI_XUAN_JING_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MATHEMATICAL_ALPHANUMERIC_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MATHEMATICAL_ALPHANUMERIC_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn MATHEMATICAL_ALPHANUMERIC_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MATHEMATICAL_ALPHANUMERIC_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAGS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAGS:Ljava/lang/Character$UnicodeBlock;
        pub fn TAGS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAGS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "VARIATION_SELECTORS_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VARIATION_SELECTORS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn VARIATION_SELECTORS_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.VARIATION_SELECTORS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPPLEMENTARY_PRIVATE_USE_AREA_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPPLEMENTARY_PRIVATE_USE_AREA_A:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPPLEMENTARY_PRIVATE_USE_AREA_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPPLEMENTARY_PRIVATE_USE_AREA_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPPLEMENTARY_PRIVATE_USE_AREA_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPPLEMENTARY_PRIVATE_USE_AREA_B:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPPLEMENTARY_PRIVATE_USE_AREA_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPPLEMENTARY_PRIVATE_USE_AREA_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HIGH_SURROGATES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HIGH_SURROGATES:Ljava/lang/Character$UnicodeBlock;
        pub fn HIGH_SURROGATES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HIGH_SURROGATES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HIGH_PRIVATE_USE_SURROGATES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HIGH_PRIVATE_USE_SURROGATES:Ljava/lang/Character$UnicodeBlock;
        pub fn HIGH_PRIVATE_USE_SURROGATES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HIGH_PRIVATE_USE_SURROGATES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LOW_SURROGATES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LOW_SURROGATES:Ljava/lang/Character$UnicodeBlock;
        pub fn LOW_SURROGATES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LOW_SURROGATES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn ARABIC_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARABIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NKO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NKO:Ljava/lang/Character$UnicodeBlock;
        pub fn NKO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NKO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SAMARITAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SAMARITAN:Ljava/lang/Character$UnicodeBlock;
        pub fn SAMARITAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SAMARITAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MANDAIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MANDAIC:Ljava/lang/Character$UnicodeBlock;
        pub fn MANDAIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MANDAIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ETHIOPIC_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ETHIOPIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn ETHIOPIC_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ETHIOPIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED:Ljava/lang/Character$UnicodeBlock;
        pub fn UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NEW_TAI_LUE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NEW_TAI_LUE:Ljava/lang/Character$UnicodeBlock;
        pub fn NEW_TAI_LUE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NEW_TAI_LUE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BUGINESE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BUGINESE:Ljava/lang/Character$UnicodeBlock;
        pub fn BUGINESE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BUGINESE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAI_THAM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAI_THAM:Ljava/lang/Character$UnicodeBlock;
        pub fn TAI_THAM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAI_THAM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BALINESE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BALINESE:Ljava/lang/Character$UnicodeBlock;
        pub fn BALINESE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BALINESE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUNDANESE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUNDANESE:Ljava/lang/Character$UnicodeBlock;
        pub fn SUNDANESE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUNDANESE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BATAK", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BATAK:Ljava/lang/Character$UnicodeBlock;
        pub fn BATAK() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BATAK:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LEPCHA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LEPCHA:Ljava/lang/Character$UnicodeBlock;
        pub fn LEPCHA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LEPCHA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OL_CHIKI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OL_CHIKI:Ljava/lang/Character$UnicodeBlock;
        pub fn OL_CHIKI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OL_CHIKI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "VEDIC_EXTENSIONS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VEDIC_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;
        pub fn VEDIC_EXTENSIONS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.VEDIC_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PHONETIC_EXTENSIONS_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PHONETIC_EXTENSIONS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn PHONETIC_EXTENSIONS_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PHONETIC_EXTENSIONS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COMBINING_DIACRITICAL_MARKS_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COMBINING_DIACRITICAL_MARKS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn COMBINING_DIACRITICAL_MARKS_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COMBINING_DIACRITICAL_MARKS_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GLAGOLITIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GLAGOLITIC:Ljava/lang/Character$UnicodeBlock;
        pub fn GLAGOLITIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GLAGOLITIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_EXTENDED_C", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_EXTENDED_C:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_EXTENDED_C() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_EXTENDED_C:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COPTIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COPTIC:Ljava/lang/Character$UnicodeBlock;
        pub fn COPTIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COPTIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GEORGIAN_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GEORGIAN_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn GEORGIAN_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GEORGIAN_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TIFINAGH", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TIFINAGH:Ljava/lang/Character$UnicodeBlock;
        pub fn TIFINAGH() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TIFINAGH:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ETHIOPIC_EXTENDED", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ETHIOPIC_EXTENDED:Ljava/lang/Character$UnicodeBlock;
        pub fn ETHIOPIC_EXTENDED() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ETHIOPIC_EXTENDED:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CYRILLIC_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYRILLIC_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn CYRILLIC_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CYRILLIC_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPPLEMENTAL_PUNCTUATION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPPLEMENTAL_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPPLEMENTAL_PUNCTUATION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPPLEMENTAL_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_STROKES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_STROKES:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_STROKES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_STROKES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LISU", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LISU:Ljava/lang/Character$UnicodeBlock;
        pub fn LISU() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LISU:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "VAI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VAI:Ljava/lang/Character$UnicodeBlock;
        pub fn VAI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.VAI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CYRILLIC_EXTENDED_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYRILLIC_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;
        pub fn CYRILLIC_EXTENDED_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CYRILLIC_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BAMUM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BAMUM:Ljava/lang/Character$UnicodeBlock;
        pub fn BAMUM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BAMUM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MODIFIER_TONE_LETTERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MODIFIER_TONE_LETTERS:Ljava/lang/Character$UnicodeBlock;
        pub fn MODIFIER_TONE_LETTERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MODIFIER_TONE_LETTERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_EXTENDED_D", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_EXTENDED_D:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_EXTENDED_D() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_EXTENDED_D:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SYLOTI_NAGRI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SYLOTI_NAGRI:Ljava/lang/Character$UnicodeBlock;
        pub fn SYLOTI_NAGRI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SYLOTI_NAGRI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COMMON_INDIC_NUMBER_FORMS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COMMON_INDIC_NUMBER_FORMS:Ljava/lang/Character$UnicodeBlock;
        pub fn COMMON_INDIC_NUMBER_FORMS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COMMON_INDIC_NUMBER_FORMS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PHAGS_PA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PHAGS_PA:Ljava/lang/Character$UnicodeBlock;
        pub fn PHAGS_PA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PHAGS_PA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SAURASHTRA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SAURASHTRA:Ljava/lang/Character$UnicodeBlock;
        pub fn SAURASHTRA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SAURASHTRA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DEVANAGARI_EXTENDED", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DEVANAGARI_EXTENDED:Ljava/lang/Character$UnicodeBlock;
        pub fn DEVANAGARI_EXTENDED() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DEVANAGARI_EXTENDED:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KAYAH_LI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KAYAH_LI:Ljava/lang/Character$UnicodeBlock;
        pub fn KAYAH_LI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KAYAH_LI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "REJANG", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: REJANG:Ljava/lang/Character$UnicodeBlock;
        pub fn REJANG() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.REJANG:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HANGUL_JAMO_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANGUL_JAMO_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn HANGUL_JAMO_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HANGUL_JAMO_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "JAVANESE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: JAVANESE:Ljava/lang/Character$UnicodeBlock;
        pub fn JAVANESE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.JAVANESE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CHAM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHAM:Ljava/lang/Character$UnicodeBlock;
        pub fn CHAM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CHAM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MYANMAR_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MYANMAR_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn MYANMAR_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MYANMAR_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAI_VIET", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAI_VIET:Ljava/lang/Character$UnicodeBlock;
        pub fn TAI_VIET() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAI_VIET:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ETHIOPIC_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ETHIOPIC_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn ETHIOPIC_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ETHIOPIC_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MEETEI_MAYEK", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEETEI_MAYEK:Ljava/lang/Character$UnicodeBlock;
        pub fn MEETEI_MAYEK() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MEETEI_MAYEK:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HANGUL_JAMO_EXTENDED_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANGUL_JAMO_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;
        pub fn HANGUL_JAMO_EXTENDED_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HANGUL_JAMO_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "VERTICAL_FORMS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VERTICAL_FORMS:Ljava/lang/Character$UnicodeBlock;
        pub fn VERTICAL_FORMS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.VERTICAL_FORMS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ANCIENT_GREEK_NUMBERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ANCIENT_GREEK_NUMBERS:Ljava/lang/Character$UnicodeBlock;
        pub fn ANCIENT_GREEK_NUMBERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ANCIENT_GREEK_NUMBERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ANCIENT_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ANCIENT_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn ANCIENT_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ANCIENT_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PHAISTOS_DISC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PHAISTOS_DISC:Ljava/lang/Character$UnicodeBlock;
        pub fn PHAISTOS_DISC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PHAISTOS_DISC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LYCIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LYCIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn LYCIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LYCIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CARIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CARIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn CARIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CARIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_PERSIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_PERSIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_PERSIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_PERSIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "IMPERIAL_ARAMAIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: IMPERIAL_ARAMAIC:Ljava/lang/Character$UnicodeBlock;
        pub fn IMPERIAL_ARAMAIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.IMPERIAL_ARAMAIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PHOENICIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PHOENICIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn PHOENICIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PHOENICIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LYDIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LYDIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn LYDIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LYDIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KHAROSHTHI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHAROSHTHI:Ljava/lang/Character$UnicodeBlock;
        pub fn KHAROSHTHI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KHAROSHTHI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_SOUTH_ARABIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_SOUTH_ARABIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_SOUTH_ARABIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_SOUTH_ARABIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "AVESTAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: AVESTAN:Ljava/lang/Character$UnicodeBlock;
        pub fn AVESTAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.AVESTAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "INSCRIPTIONAL_PARTHIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INSCRIPTIONAL_PARTHIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn INSCRIPTIONAL_PARTHIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.INSCRIPTIONAL_PARTHIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "INSCRIPTIONAL_PAHLAVI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INSCRIPTIONAL_PAHLAVI:Ljava/lang/Character$UnicodeBlock;
        pub fn INSCRIPTIONAL_PAHLAVI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.INSCRIPTIONAL_PAHLAVI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_TURKIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_TURKIC:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_TURKIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_TURKIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "RUMI_NUMERAL_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: RUMI_NUMERAL_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn RUMI_NUMERAL_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.RUMI_NUMERAL_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BRAHMI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BRAHMI:Ljava/lang/Character$UnicodeBlock;
        pub fn BRAHMI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BRAHMI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KAITHI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KAITHI:Ljava/lang/Character$UnicodeBlock;
        pub fn KAITHI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KAITHI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CUNEIFORM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CUNEIFORM:Ljava/lang/Character$UnicodeBlock;
        pub fn CUNEIFORM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CUNEIFORM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CUNEIFORM_NUMBERS_AND_PUNCTUATION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CUNEIFORM_NUMBERS_AND_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;
        pub fn CUNEIFORM_NUMBERS_AND_PUNCTUATION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CUNEIFORM_NUMBERS_AND_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "EGYPTIAN_HIEROGLYPHS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EGYPTIAN_HIEROGLYPHS:Ljava/lang/Character$UnicodeBlock;
        pub fn EGYPTIAN_HIEROGLYPHS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.EGYPTIAN_HIEROGLYPHS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BAMUM_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BAMUM_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn BAMUM_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BAMUM_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KANA_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KANA_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn KANA_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KANA_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ANCIENT_GREEK_MUSICAL_NOTATION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ANCIENT_GREEK_MUSICAL_NOTATION:Ljava/lang/Character$UnicodeBlock;
        pub fn ANCIENT_GREEK_MUSICAL_NOTATION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ANCIENT_GREEK_MUSICAL_NOTATION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COUNTING_ROD_NUMERALS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COUNTING_ROD_NUMERALS:Ljava/lang/Character$UnicodeBlock;
        pub fn COUNTING_ROD_NUMERALS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COUNTING_ROD_NUMERALS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MAHJONG_TILES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAHJONG_TILES:Ljava/lang/Character$UnicodeBlock;
        pub fn MAHJONG_TILES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MAHJONG_TILES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DOMINO_TILES", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DOMINO_TILES:Ljava/lang/Character$UnicodeBlock;
        pub fn DOMINO_TILES() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DOMINO_TILES:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PLAYING_CARDS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PLAYING_CARDS:Ljava/lang/Character$UnicodeBlock;
        pub fn PLAYING_CARDS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PLAYING_CARDS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ENCLOSED_ALPHANUMERIC_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ENCLOSED_ALPHANUMERIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn ENCLOSED_ALPHANUMERIC_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ENCLOSED_ALPHANUMERIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ENCLOSED_IDEOGRAPHIC_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ENCLOSED_IDEOGRAPHIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn ENCLOSED_IDEOGRAPHIC_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ENCLOSED_IDEOGRAPHIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS:Ljava/lang/Character$UnicodeBlock;
        pub fn MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "EMOTICONS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EMOTICONS:Ljava/lang/Character$UnicodeBlock;
        pub fn EMOTICONS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.EMOTICONS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TRANSPORT_AND_MAP_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TRANSPORT_AND_MAP_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn TRANSPORT_AND_MAP_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TRANSPORT_AND_MAP_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ALCHEMICAL_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ALCHEMICAL_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn ALCHEMICAL_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ALCHEMICAL_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn ARABIC_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARABIC_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUNDANESE_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUNDANESE_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn SUNDANESE_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUNDANESE_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MEETEI_MAYEK_EXTENSIONS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEETEI_MAYEK_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;
        pub fn MEETEI_MAYEK_EXTENSIONS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MEETEI_MAYEK_EXTENSIONS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MEROITIC_HIEROGLYPHS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEROITIC_HIEROGLYPHS:Ljava/lang/Character$UnicodeBlock;
        pub fn MEROITIC_HIEROGLYPHS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MEROITIC_HIEROGLYPHS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MEROITIC_CURSIVE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEROITIC_CURSIVE:Ljava/lang/Character$UnicodeBlock;
        pub fn MEROITIC_CURSIVE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MEROITIC_CURSIVE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SORA_SOMPENG", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SORA_SOMPENG:Ljava/lang/Character$UnicodeBlock;
        pub fn SORA_SOMPENG() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SORA_SOMPENG:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CHAKMA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHAKMA:Ljava/lang/Character$UnicodeBlock;
        pub fn CHAKMA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CHAKMA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SHARADA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SHARADA:Ljava/lang/Character$UnicodeBlock;
        pub fn SHARADA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SHARADA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAKRI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAKRI:Ljava/lang/Character$UnicodeBlock;
        pub fn TAKRI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAKRI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MIAO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MIAO:Ljava/lang/Character$UnicodeBlock;
        pub fn MIAO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MIAO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COMBINING_DIACRITICAL_MARKS_EXTENDED", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COMBINING_DIACRITICAL_MARKS_EXTENDED:Ljava/lang/Character$UnicodeBlock;
        pub fn COMBINING_DIACRITICAL_MARKS_EXTENDED() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COMBINING_DIACRITICAL_MARKS_EXTENDED:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MYANMAR_EXTENDED_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MYANMAR_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;
        pub fn MYANMAR_EXTENDED_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MYANMAR_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_EXTENDED_E", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_EXTENDED_E:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_EXTENDED_E() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_EXTENDED_E:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "COPTIC_EPACT_NUMBERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: COPTIC_EPACT_NUMBERS:Ljava/lang/Character$UnicodeBlock;
        pub fn COPTIC_EPACT_NUMBERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.COPTIC_EPACT_NUMBERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_PERMIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_PERMIC:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_PERMIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_PERMIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ELBASAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ELBASAN:Ljava/lang/Character$UnicodeBlock;
        pub fn ELBASAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ELBASAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CAUCASIAN_ALBANIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CAUCASIAN_ALBANIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn CAUCASIAN_ALBANIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CAUCASIAN_ALBANIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LINEAR_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LINEAR_A:Ljava/lang/Character$UnicodeBlock;
        pub fn LINEAR_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LINEAR_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PALMYRENE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PALMYRENE:Ljava/lang/Character$UnicodeBlock;
        pub fn PALMYRENE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PALMYRENE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NABATAEAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NABATAEAN:Ljava/lang/Character$UnicodeBlock;
        pub fn NABATAEAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NABATAEAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_NORTH_ARABIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_NORTH_ARABIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_NORTH_ARABIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_NORTH_ARABIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MANICHAEAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MANICHAEAN:Ljava/lang/Character$UnicodeBlock;
        pub fn MANICHAEAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MANICHAEAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PSALTER_PAHLAVI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PSALTER_PAHLAVI:Ljava/lang/Character$UnicodeBlock;
        pub fn PSALTER_PAHLAVI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PSALTER_PAHLAVI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MAHAJANI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAHAJANI:Ljava/lang/Character$UnicodeBlock;
        pub fn MAHAJANI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MAHAJANI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SINHALA_ARCHAIC_NUMBERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SINHALA_ARCHAIC_NUMBERS:Ljava/lang/Character$UnicodeBlock;
        pub fn SINHALA_ARCHAIC_NUMBERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SINHALA_ARCHAIC_NUMBERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KHOJKI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHOJKI:Ljava/lang/Character$UnicodeBlock;
        pub fn KHOJKI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KHOJKI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KHUDAWADI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHUDAWADI:Ljava/lang/Character$UnicodeBlock;
        pub fn KHUDAWADI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KHUDAWADI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GRANTHA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GRANTHA:Ljava/lang/Character$UnicodeBlock;
        pub fn GRANTHA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GRANTHA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TIRHUTA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TIRHUTA:Ljava/lang/Character$UnicodeBlock;
        pub fn TIRHUTA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TIRHUTA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SIDDHAM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SIDDHAM:Ljava/lang/Character$UnicodeBlock;
        pub fn SIDDHAM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SIDDHAM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MODI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MODI:Ljava/lang/Character$UnicodeBlock;
        pub fn MODI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MODI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "WARANG_CITI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: WARANG_CITI:Ljava/lang/Character$UnicodeBlock;
        pub fn WARANG_CITI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.WARANG_CITI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PAU_CIN_HAU", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PAU_CIN_HAU:Ljava/lang/Character$UnicodeBlock;
        pub fn PAU_CIN_HAU() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PAU_CIN_HAU:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MRO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MRO:Ljava/lang/Character$UnicodeBlock;
        pub fn MRO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MRO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BASSA_VAH", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BASSA_VAH:Ljava/lang/Character$UnicodeBlock;
        pub fn BASSA_VAH() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BASSA_VAH:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "PAHAWH_HMONG", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PAHAWH_HMONG:Ljava/lang/Character$UnicodeBlock;
        pub fn PAHAWH_HMONG() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.PAHAWH_HMONG:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DUPLOYAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DUPLOYAN:Ljava/lang/Character$UnicodeBlock;
        pub fn DUPLOYAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DUPLOYAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SHORTHAND_FORMAT_CONTROLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SHORTHAND_FORMAT_CONTROLS:Ljava/lang/Character$UnicodeBlock;
        pub fn SHORTHAND_FORMAT_CONTROLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SHORTHAND_FORMAT_CONTROLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MENDE_KIKAKUI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MENDE_KIKAKUI:Ljava/lang/Character$UnicodeBlock;
        pub fn MENDE_KIKAKUI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MENDE_KIKAKUI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ORNAMENTAL_DINGBATS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ORNAMENTAL_DINGBATS:Ljava/lang/Character$UnicodeBlock;
        pub fn ORNAMENTAL_DINGBATS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ORNAMENTAL_DINGBATS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GEOMETRIC_SHAPES_EXTENDED", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GEOMETRIC_SHAPES_EXTENDED:Ljava/lang/Character$UnicodeBlock;
        pub fn GEOMETRIC_SHAPES_EXTENDED() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GEOMETRIC_SHAPES_EXTENDED:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPPLEMENTAL_ARROWS_C", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPPLEMENTAL_ARROWS_C:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPPLEMENTAL_ARROWS_C() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPPLEMENTAL_ARROWS_C:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CHEROKEE_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHEROKEE_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn CHEROKEE_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CHEROKEE_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HATRAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HATRAN:Ljava/lang/Character$UnicodeBlock;
        pub fn HATRAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HATRAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_HUNGARIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_HUNGARIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_HUNGARIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_HUNGARIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MULTANI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MULTANI:Ljava/lang/Character$UnicodeBlock;
        pub fn MULTANI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MULTANI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "AHOM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: AHOM:Ljava/lang/Character$UnicodeBlock;
        pub fn AHOM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.AHOM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "EARLY_DYNASTIC_CUNEIFORM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EARLY_DYNASTIC_CUNEIFORM:Ljava/lang/Character$UnicodeBlock;
        pub fn EARLY_DYNASTIC_CUNEIFORM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.EARLY_DYNASTIC_CUNEIFORM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ANATOLIAN_HIEROGLYPHS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ANATOLIAN_HIEROGLYPHS:Ljava/lang/Character$UnicodeBlock;
        pub fn ANATOLIAN_HIEROGLYPHS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ANATOLIAN_HIEROGLYPHS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUTTON_SIGNWRITING", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUTTON_SIGNWRITING:Ljava/lang/Character$UnicodeBlock;
        pub fn SUTTON_SIGNWRITING() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUTTON_SIGNWRITING:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS:Ljava/lang/Character$UnicodeBlock;
        pub fn SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SYRIAC_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SYRIAC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn SYRIAC_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SYRIAC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CYRILLIC_EXTENDED_C", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYRILLIC_EXTENDED_C:Ljava/lang/Character$UnicodeBlock;
        pub fn CYRILLIC_EXTENDED_C() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CYRILLIC_EXTENDED_C:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OSAGE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OSAGE:Ljava/lang/Character$UnicodeBlock;
        pub fn OSAGE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OSAGE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NEWA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NEWA:Ljava/lang/Character$UnicodeBlock;
        pub fn NEWA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NEWA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MONGOLIAN_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MONGOLIAN_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn MONGOLIAN_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MONGOLIAN_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MARCHEN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MARCHEN:Ljava/lang/Character$UnicodeBlock;
        pub fn MARCHEN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MARCHEN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;
        pub fn IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TANGUT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TANGUT:Ljava/lang/Character$UnicodeBlock;
        pub fn TANGUT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TANGUT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TANGUT_COMPONENTS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TANGUT_COMPONENTS:Ljava/lang/Character$UnicodeBlock;
        pub fn TANGUT_COMPONENTS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TANGUT_COMPONENTS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KANA_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KANA_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn KANA_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KANA_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GLAGOLITIC_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GLAGOLITIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn GLAGOLITIC_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GLAGOLITIC_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ADLAM", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ADLAM:Ljava/lang/Character$UnicodeBlock;
        pub fn ADLAM() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ADLAM:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MASARAM_GONDI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MASARAM_GONDI:Ljava/lang/Character$UnicodeBlock;
        pub fn MASARAM_GONDI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MASARAM_GONDI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ZANABAZAR_SQUARE", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ZANABAZAR_SQUARE:Ljava/lang/Character$UnicodeBlock;
        pub fn ZANABAZAR_SQUARE() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ZANABAZAR_SQUARE:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NUSHU", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NUSHU:Ljava/lang/Character$UnicodeBlock;
        pub fn NUSHU() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NUSHU:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SOYOMBO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SOYOMBO:Ljava/lang/Character$UnicodeBlock;
        pub fn SOYOMBO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SOYOMBO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "BHAIKSUKI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: BHAIKSUKI:Ljava/lang/Character$UnicodeBlock;
        pub fn BHAIKSUKI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.BHAIKSUKI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GEORGIAN_EXTENDED", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GEORGIAN_EXTENDED:Ljava/lang/Character$UnicodeBlock;
        pub fn GEORGIAN_EXTENDED() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GEORGIAN_EXTENDED:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "HANIFI_ROHINGYA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HANIFI_ROHINGYA:Ljava/lang/Character$UnicodeBlock;
        pub fn HANIFI_ROHINGYA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.HANIFI_ROHINGYA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_SOGDIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_SOGDIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_SOGDIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_SOGDIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SOGDIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SOGDIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn SOGDIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SOGDIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DOGRA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DOGRA:Ljava/lang/Character$UnicodeBlock;
        pub fn DOGRA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DOGRA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "GUNJALA_GONDI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GUNJALA_GONDI:Ljava/lang/Character$UnicodeBlock;
        pub fn GUNJALA_GONDI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.GUNJALA_GONDI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MAKASAR", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAKASAR:Ljava/lang/Character$UnicodeBlock;
        pub fn MAKASAR() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MAKASAR:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MEDEFAIDRIN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MEDEFAIDRIN:Ljava/lang/Character$UnicodeBlock;
        pub fn MEDEFAIDRIN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MEDEFAIDRIN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "MAYAN_NUMERALS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAYAN_NUMERALS:Ljava/lang/Character$UnicodeBlock;
        pub fn MAYAN_NUMERALS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.MAYAN_NUMERALS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "INDIC_SIYAQ_NUMBERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INDIC_SIYAQ_NUMBERS:Ljava/lang/Character$UnicodeBlock;
        pub fn INDIC_SIYAQ_NUMBERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.INDIC_SIYAQ_NUMBERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CHESS_SYMBOLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHESS_SYMBOLS:Ljava/lang/Character$UnicodeBlock;
        pub fn CHESS_SYMBOLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CHESS_SYMBOLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ELYMAIC", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ELYMAIC:Ljava/lang/Character$UnicodeBlock;
        pub fn ELYMAIC() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ELYMAIC:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NANDINAGARI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NANDINAGARI:Ljava/lang/Character$UnicodeBlock;
        pub fn NANDINAGARI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NANDINAGARI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TAMIL_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAMIL_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn TAMIL_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TAMIL_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS:Ljava/lang/Character$UnicodeBlock;
        pub fn EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SMALL_KANA_EXTENSION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SMALL_KANA_EXTENSION:Ljava/lang/Character$UnicodeBlock;
        pub fn SMALL_KANA_EXTENSION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SMALL_KANA_EXTENSION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NYIAKENG_PUACHUE_HMONG", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NYIAKENG_PUACHUE_HMONG:Ljava/lang/Character$UnicodeBlock;
        pub fn NYIAKENG_PUACHUE_HMONG() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NYIAKENG_PUACHUE_HMONG:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "WANCHO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: WANCHO:Ljava/lang/Character$UnicodeBlock;
        pub fn WANCHO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.WANCHO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OTTOMAN_SIYAQ_NUMBERS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OTTOMAN_SIYAQ_NUMBERS:Ljava/lang/Character$UnicodeBlock;
        pub fn OTTOMAN_SIYAQ_NUMBERS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OTTOMAN_SIYAQ_NUMBERS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "YEZIDI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: YEZIDI:Ljava/lang/Character$UnicodeBlock;
        pub fn YEZIDI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.YEZIDI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CHORASMIAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHORASMIAN:Ljava/lang/Character$UnicodeBlock;
        pub fn CHORASMIAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CHORASMIAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DIVES_AKURU", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DIVES_AKURU:Ljava/lang/Character$UnicodeBlock;
        pub fn DIVES_AKURU() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DIVES_AKURU:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LISU_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LISU_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn LISU_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LISU_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KHITAN_SMALL_SCRIPT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KHITAN_SMALL_SCRIPT:Ljava/lang/Character$UnicodeBlock;
        pub fn KHITAN_SMALL_SCRIPT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KHITAN_SMALL_SCRIPT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TANGUT_SUPPLEMENT", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TANGUT_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;
        pub fn TANGUT_SUPPLEMENT() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TANGUT_SUPPLEMENT:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "SYMBOLS_FOR_LEGACY_COMPUTING", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SYMBOLS_FOR_LEGACY_COMPUTING:Ljava/lang/Character$UnicodeBlock;
        pub fn SYMBOLS_FOR_LEGACY_COMPUTING() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.SYMBOLS_FOR_LEGACY_COMPUTING:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC_EXTENDED_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;
        pub fn ARABIC_EXTENDED_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARABIC_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "VITHKUQI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VITHKUQI:Ljava/lang/Character$UnicodeBlock;
        pub fn VITHKUQI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.VITHKUQI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_EXTENDED_F", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_EXTENDED_F:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_EXTENDED_F() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_EXTENDED_F:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "OLD_UYGHUR", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OLD_UYGHUR:Ljava/lang/Character$UnicodeBlock;
        pub fn OLD_UYGHUR() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.OLD_UYGHUR:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CYPRO_MINOAN", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYPRO_MINOAN:Ljava/lang/Character$UnicodeBlock;
        pub fn CYPRO_MINOAN() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CYPRO_MINOAN:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TANGSA", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TANGSA:Ljava/lang/Character$UnicodeBlock;
        pub fn TANGSA() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TANGSA:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KANA_EXTENDED_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KANA_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;
        pub fn KANA_EXTENDED_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KANA_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ZNAMENNY_MUSICAL_NOTATION", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ZNAMENNY_MUSICAL_NOTATION:Ljava/lang/Character$UnicodeBlock;
        pub fn ZNAMENNY_MUSICAL_NOTATION() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ZNAMENNY_MUSICAL_NOTATION:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN_EXTENDED_G", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LATIN_EXTENDED_G:Ljava/lang/Character$UnicodeBlock;
        pub fn LATIN_EXTENDED_G() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.LATIN_EXTENDED_G:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "TOTO", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TOTO:Ljava/lang/Character$UnicodeBlock;
        pub fn TOTO() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.TOTO:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ETHIOPIC_EXTENDED_B", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ETHIOPIC_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;
        pub fn ETHIOPIC_EXTENDED_B() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ETHIOPIC_EXTENDED_B:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "ARABIC_EXTENDED_C", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARABIC_EXTENDED_C:Ljava/lang/Character$UnicodeBlock;
        pub fn ARABIC_EXTENDED_C() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.ARABIC_EXTENDED_C:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "DEVANAGARI_EXTENDED_A", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DEVANAGARI_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;
        pub fn DEVANAGARI_EXTENDED_A() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.DEVANAGARI_EXTENDED_A:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KAWI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KAWI:Ljava/lang/Character$UnicodeBlock;
        pub fn KAWI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KAWI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "KAKTOVIK_NUMERALS", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KAKTOVIK_NUMERALS:Ljava/lang/Character$UnicodeBlock;
        pub fn KAKTOVIK_NUMERALS() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.KAKTOVIK_NUMERALS:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CYRILLIC_EXTENDED_D", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CYRILLIC_EXTENDED_D:Ljava/lang/Character$UnicodeBlock;
        pub fn CYRILLIC_EXTENDED_D() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CYRILLIC_EXTENDED_D:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "NAG_MUNDARI", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NAG_MUNDARI:Ljava/lang/Character$UnicodeBlock;
        pub fn NAG_MUNDARI() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.NAG_MUNDARI:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H", descriptor = "Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H:Ljava/lang/Character$UnicodeBlock;
        pub fn CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H() -> Character_UnicodeBlock {
            panic!("stub: java/lang/Character$UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H:Ljava/lang/Character$UnicodeBlock;")
        }

        #[cfg_attr(any(), java_field(name = "blockStarts", descriptor = "[I", access = "private", modifiers = "static final", is_static = true))]
        // static field: blockStarts:[I
        pub fn blockStarts() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/lang/Character$UnicodeBlock.blockStarts:[I")
        }

        #[cfg_attr(any(), java_field(name = "blocks", descriptor = "[Ljava/lang/Character$UnicodeBlock;", access = "private", modifiers = "static final", is_static = true))]
        // static field: blocks:[Ljava/lang/Character$UnicodeBlock;
        pub fn blocks() -> Rc<RefCell<Vec<Character_UnicodeBlock>>> {
            panic!("stub: java/lang/Character$UnicodeBlock.blocks:[Ljava/lang/Character$UnicodeBlock;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(idName: String) -> Result<Self> {
            panic!("stub: java/lang/Character$UnicodeBlock.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_str(idName: String, alias: String) -> Result<Self> {
            panic!("stub: java/lang/Character$UnicodeBlock.<init>:(Ljava/lang/String;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;[Ljava/lang/String;)V", access = "private", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_arr_str(idName: String, aliases: Rc<RefCell<Vec<String>>>) -> Result<Self> {
            panic!("stub: java/lang/Character$UnicodeBlock.<init>:(Ljava/lang/String;[Ljava/lang/String;)V")
        }

        #[java_method(name = "of", descriptor = "(C)Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_c(c: u16) -> Result<Character_UnicodeBlock> {
            panic!("stub: java/lang/Character$UnicodeBlock.of:(C)Ljava/lang/Character$UnicodeBlock;")
        }

        #[java_method(name = "of", descriptor = "(I)Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: of(I)Ljava/lang/Character$UnicodeBlock;
        pub fn of_i(mut codePoint: i32) -> Result<Character_UnicodeBlock> {
            let _t0: bool = Character::isValidCodePoint(codePoint)?;
            if !(_t0) {
                let mut _arr1: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr1.borrow_mut()[0i32 as usize] = codePoint.into();
                let _t2: String = String::format_str_arr_obj(Clone::clone(&String::from("Not a valid Unicode code point: 0x%X")), Clone::clone(&_arr1))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut bottom: i32 = 0i32;
            let mut top = (Character_UnicodeBlock::blockStarts().borrow().len() as i32);
            let mut current = (top/2i32);
            loop {
                if (top).wrapping_sub(bottom) <= 1i32 { break; }
                if codePoint >= Character_UnicodeBlock::blockStarts().borrow()[current as usize] {
                    bottom = current;
                } else {
                    top = current;
                }
                current = ((top).wrapping_add(bottom)/2i32);
            }
            Ok(Clone::clone(&Character_UnicodeBlock::blocks().borrow()[current as usize]))
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;)Ljava/lang/Character$UnicodeBlock;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forName(mut blockName: String) -> Result<Character_UnicodeBlock> {
            let _t0 = blockName.toUpperCase_locale(Clone::clone(&Locale::US()))?;
            let _vdispatch1: Object = if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<Properties>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(_d) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(_t0.clone()))? } else if let Some(__f) = Character_UnicodeBlock::map().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(_t0.clone()))? } else { Default::default() };
            let mut block = (_vdispatch1).downcast::<Character_UnicodeBlock>();
            if _is_jnull(&block) {
                let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Not a valid block name: ")))?;
                let _t3 = _t2.append_str(Clone::clone(&blockName))?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(block)
        }
    }
}
