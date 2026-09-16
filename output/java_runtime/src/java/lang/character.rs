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
    #[binary_name       = "java/lang/Character"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/Comparable,java/lang/constant/Constable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/Comparable<Ljava/lang/Character;>;Ljava/lang/constant/Constable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Character.java"]
    #[inner_classes     = "java/lang/Character$CharacterCache:java/lang/Character:CharacterCache:26;java/lang/Character$UnicodeBlock:java/lang/Character:UnicodeBlock:25;java/lang/Character$UnicodeScript:java/lang/Character:UnicodeScript:16409;java/lang/Character$Subset:java/lang/Character:Subset:9"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Character;java/lang/Comparable;java/lang/Object;java/lang/constant/Constable"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Character {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "C", access = "private", modifiers = "final", is_static = false))]
        pub value: u16,
    }

    impl Character {
        #[cfg_attr(any(), java_field(name = "MIN_RADIX", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: MIN_RADIX:I
        pub fn MIN_RADIX() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "MAX_RADIX", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "36"))]
        // static field: MAX_RADIX:I
        pub fn MAX_RADIX() -> i32 {
            36
        }

        #[cfg_attr(any(), java_field(name = "MIN_VALUE", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: MIN_VALUE:C
        pub fn MIN_VALUE() -> u16 {
            0
        }

        #[cfg_attr(any(), java_field(name = "MAX_VALUE", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "65535"))]
        // static field: MAX_VALUE:C
        pub fn MAX_VALUE() -> u16 {
            65535
        }

        #[cfg_attr(any(), java_field(name = "TYPE", descriptor = "Ljava/lang/Class;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/Class<Ljava/lang/Character;>;"))]
        // static field: TYPE:Ljava/lang/Class;
        pub fn TYPE() -> Class<Character> {
            panic!("stub: java/lang/Character.TYPE:Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "UNASSIGNED", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: UNASSIGNED:B
        pub fn UNASSIGNED() -> i8 {
            0
        }

        #[cfg_attr(any(), java_field(name = "UPPERCASE_LETTER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: UPPERCASE_LETTER:B
        pub fn UPPERCASE_LETTER() -> i8 {
            1
        }

        #[cfg_attr(any(), java_field(name = "LOWERCASE_LETTER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: LOWERCASE_LETTER:B
        pub fn LOWERCASE_LETTER() -> i8 {
            2
        }

        #[cfg_attr(any(), java_field(name = "TITLECASE_LETTER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: TITLECASE_LETTER:B
        pub fn TITLECASE_LETTER() -> i8 {
            3
        }

        #[cfg_attr(any(), java_field(name = "MODIFIER_LETTER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: MODIFIER_LETTER:B
        pub fn MODIFIER_LETTER() -> i8 {
            4
        }

        #[cfg_attr(any(), java_field(name = "OTHER_LETTER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: OTHER_LETTER:B
        pub fn OTHER_LETTER() -> i8 {
            5
        }

        #[cfg_attr(any(), java_field(name = "NON_SPACING_MARK", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: NON_SPACING_MARK:B
        pub fn NON_SPACING_MARK() -> i8 {
            6
        }

        #[cfg_attr(any(), java_field(name = "ENCLOSING_MARK", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: ENCLOSING_MARK:B
        pub fn ENCLOSING_MARK() -> i8 {
            7
        }

        #[cfg_attr(any(), java_field(name = "COMBINING_SPACING_MARK", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: COMBINING_SPACING_MARK:B
        pub fn COMBINING_SPACING_MARK() -> i8 {
            8
        }

        #[cfg_attr(any(), java_field(name = "DECIMAL_DIGIT_NUMBER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: DECIMAL_DIGIT_NUMBER:B
        pub fn DECIMAL_DIGIT_NUMBER() -> i8 {
            9
        }

        #[cfg_attr(any(), java_field(name = "LETTER_NUMBER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: LETTER_NUMBER:B
        pub fn LETTER_NUMBER() -> i8 {
            10
        }

        #[cfg_attr(any(), java_field(name = "OTHER_NUMBER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: OTHER_NUMBER:B
        pub fn OTHER_NUMBER() -> i8 {
            11
        }

        #[cfg_attr(any(), java_field(name = "SPACE_SEPARATOR", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: SPACE_SEPARATOR:B
        pub fn SPACE_SEPARATOR() -> i8 {
            12
        }

        #[cfg_attr(any(), java_field(name = "LINE_SEPARATOR", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "13"))]
        // static field: LINE_SEPARATOR:B
        pub fn LINE_SEPARATOR() -> i8 {
            13
        }

        #[cfg_attr(any(), java_field(name = "PARAGRAPH_SEPARATOR", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "14"))]
        // static field: PARAGRAPH_SEPARATOR:B
        pub fn PARAGRAPH_SEPARATOR() -> i8 {
            14
        }

        #[cfg_attr(any(), java_field(name = "CONTROL", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "15"))]
        // static field: CONTROL:B
        pub fn CONTROL() -> i8 {
            15
        }

        #[cfg_attr(any(), java_field(name = "FORMAT", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: FORMAT:B
        pub fn FORMAT() -> i8 {
            16
        }

        #[cfg_attr(any(), java_field(name = "PRIVATE_USE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "18"))]
        // static field: PRIVATE_USE:B
        pub fn PRIVATE_USE() -> i8 {
            18
        }

        #[cfg_attr(any(), java_field(name = "SURROGATE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "19"))]
        // static field: SURROGATE:B
        pub fn SURROGATE() -> i8 {
            19
        }

        #[cfg_attr(any(), java_field(name = "DASH_PUNCTUATION", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "20"))]
        // static field: DASH_PUNCTUATION:B
        pub fn DASH_PUNCTUATION() -> i8 {
            20
        }

        #[cfg_attr(any(), java_field(name = "START_PUNCTUATION", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "21"))]
        // static field: START_PUNCTUATION:B
        pub fn START_PUNCTUATION() -> i8 {
            21
        }

        #[cfg_attr(any(), java_field(name = "END_PUNCTUATION", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "22"))]
        // static field: END_PUNCTUATION:B
        pub fn END_PUNCTUATION() -> i8 {
            22
        }

        #[cfg_attr(any(), java_field(name = "CONNECTOR_PUNCTUATION", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "23"))]
        // static field: CONNECTOR_PUNCTUATION:B
        pub fn CONNECTOR_PUNCTUATION() -> i8 {
            23
        }

        #[cfg_attr(any(), java_field(name = "OTHER_PUNCTUATION", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "24"))]
        // static field: OTHER_PUNCTUATION:B
        pub fn OTHER_PUNCTUATION() -> i8 {
            24
        }

        #[cfg_attr(any(), java_field(name = "MATH_SYMBOL", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "25"))]
        // static field: MATH_SYMBOL:B
        pub fn MATH_SYMBOL() -> i8 {
            25
        }

        #[cfg_attr(any(), java_field(name = "CURRENCY_SYMBOL", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "26"))]
        // static field: CURRENCY_SYMBOL:B
        pub fn CURRENCY_SYMBOL() -> i8 {
            26
        }

        #[cfg_attr(any(), java_field(name = "MODIFIER_SYMBOL", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "27"))]
        // static field: MODIFIER_SYMBOL:B
        pub fn MODIFIER_SYMBOL() -> i8 {
            27
        }

        #[cfg_attr(any(), java_field(name = "OTHER_SYMBOL", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "28"))]
        // static field: OTHER_SYMBOL:B
        pub fn OTHER_SYMBOL() -> i8 {
            28
        }

        #[cfg_attr(any(), java_field(name = "INITIAL_QUOTE_PUNCTUATION", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "29"))]
        // static field: INITIAL_QUOTE_PUNCTUATION:B
        pub fn INITIAL_QUOTE_PUNCTUATION() -> i8 {
            29
        }

        #[cfg_attr(any(), java_field(name = "FINAL_QUOTE_PUNCTUATION", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "30"))]
        // static field: FINAL_QUOTE_PUNCTUATION:B
        pub fn FINAL_QUOTE_PUNCTUATION() -> i8 {
            30
        }

        #[cfg_attr(any(), java_field(name = "ERROR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-1"))]
        // static field: ERROR:I
        pub fn ERROR() -> i32 {
            -1
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_UNDEFINED", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "-1"))]
        // static field: DIRECTIONALITY_UNDEFINED:B
        pub fn DIRECTIONALITY_UNDEFINED() -> i8 {
            -1
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_LEFT_TO_RIGHT", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: DIRECTIONALITY_LEFT_TO_RIGHT:B
        pub fn DIRECTIONALITY_LEFT_TO_RIGHT() -> i8 {
            0
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_RIGHT_TO_LEFT", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: DIRECTIONALITY_RIGHT_TO_LEFT:B
        pub fn DIRECTIONALITY_RIGHT_TO_LEFT() -> i8 {
            1
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_RIGHT_TO_LEFT_ARABIC", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: DIRECTIONALITY_RIGHT_TO_LEFT_ARABIC:B
        pub fn DIRECTIONALITY_RIGHT_TO_LEFT_ARABIC() -> i8 {
            2
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_EUROPEAN_NUMBER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: DIRECTIONALITY_EUROPEAN_NUMBER:B
        pub fn DIRECTIONALITY_EUROPEAN_NUMBER() -> i8 {
            3
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_EUROPEAN_NUMBER_SEPARATOR", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: DIRECTIONALITY_EUROPEAN_NUMBER_SEPARATOR:B
        pub fn DIRECTIONALITY_EUROPEAN_NUMBER_SEPARATOR() -> i8 {
            4
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_EUROPEAN_NUMBER_TERMINATOR", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: DIRECTIONALITY_EUROPEAN_NUMBER_TERMINATOR:B
        pub fn DIRECTIONALITY_EUROPEAN_NUMBER_TERMINATOR() -> i8 {
            5
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_ARABIC_NUMBER", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: DIRECTIONALITY_ARABIC_NUMBER:B
        pub fn DIRECTIONALITY_ARABIC_NUMBER() -> i8 {
            6
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_COMMON_NUMBER_SEPARATOR", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: DIRECTIONALITY_COMMON_NUMBER_SEPARATOR:B
        pub fn DIRECTIONALITY_COMMON_NUMBER_SEPARATOR() -> i8 {
            7
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_NONSPACING_MARK", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: DIRECTIONALITY_NONSPACING_MARK:B
        pub fn DIRECTIONALITY_NONSPACING_MARK() -> i8 {
            8
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_BOUNDARY_NEUTRAL", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: DIRECTIONALITY_BOUNDARY_NEUTRAL:B
        pub fn DIRECTIONALITY_BOUNDARY_NEUTRAL() -> i8 {
            9
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_PARAGRAPH_SEPARATOR", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: DIRECTIONALITY_PARAGRAPH_SEPARATOR:B
        pub fn DIRECTIONALITY_PARAGRAPH_SEPARATOR() -> i8 {
            10
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_SEGMENT_SEPARATOR", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: DIRECTIONALITY_SEGMENT_SEPARATOR:B
        pub fn DIRECTIONALITY_SEGMENT_SEPARATOR() -> i8 {
            11
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_WHITESPACE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: DIRECTIONALITY_WHITESPACE:B
        pub fn DIRECTIONALITY_WHITESPACE() -> i8 {
            12
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_OTHER_NEUTRALS", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "13"))]
        // static field: DIRECTIONALITY_OTHER_NEUTRALS:B
        pub fn DIRECTIONALITY_OTHER_NEUTRALS() -> i8 {
            13
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_LEFT_TO_RIGHT_EMBEDDING", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "14"))]
        // static field: DIRECTIONALITY_LEFT_TO_RIGHT_EMBEDDING:B
        pub fn DIRECTIONALITY_LEFT_TO_RIGHT_EMBEDDING() -> i8 {
            14
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_LEFT_TO_RIGHT_OVERRIDE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "15"))]
        // static field: DIRECTIONALITY_LEFT_TO_RIGHT_OVERRIDE:B
        pub fn DIRECTIONALITY_LEFT_TO_RIGHT_OVERRIDE() -> i8 {
            15
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_RIGHT_TO_LEFT_EMBEDDING", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: DIRECTIONALITY_RIGHT_TO_LEFT_EMBEDDING:B
        pub fn DIRECTIONALITY_RIGHT_TO_LEFT_EMBEDDING() -> i8 {
            16
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_RIGHT_TO_LEFT_OVERRIDE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "17"))]
        // static field: DIRECTIONALITY_RIGHT_TO_LEFT_OVERRIDE:B
        pub fn DIRECTIONALITY_RIGHT_TO_LEFT_OVERRIDE() -> i8 {
            17
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_POP_DIRECTIONAL_FORMAT", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "18"))]
        // static field: DIRECTIONALITY_POP_DIRECTIONAL_FORMAT:B
        pub fn DIRECTIONALITY_POP_DIRECTIONAL_FORMAT() -> i8 {
            18
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_LEFT_TO_RIGHT_ISOLATE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "19"))]
        // static field: DIRECTIONALITY_LEFT_TO_RIGHT_ISOLATE:B
        pub fn DIRECTIONALITY_LEFT_TO_RIGHT_ISOLATE() -> i8 {
            19
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_RIGHT_TO_LEFT_ISOLATE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "20"))]
        // static field: DIRECTIONALITY_RIGHT_TO_LEFT_ISOLATE:B
        pub fn DIRECTIONALITY_RIGHT_TO_LEFT_ISOLATE() -> i8 {
            20
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_FIRST_STRONG_ISOLATE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "21"))]
        // static field: DIRECTIONALITY_FIRST_STRONG_ISOLATE:B
        pub fn DIRECTIONALITY_FIRST_STRONG_ISOLATE() -> i8 {
            21
        }

        #[cfg_attr(any(), java_field(name = "DIRECTIONALITY_POP_DIRECTIONAL_ISOLATE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "22"))]
        // static field: DIRECTIONALITY_POP_DIRECTIONAL_ISOLATE:B
        pub fn DIRECTIONALITY_POP_DIRECTIONAL_ISOLATE() -> i8 {
            22
        }

        #[cfg_attr(any(), java_field(name = "MIN_HIGH_SURROGATE", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "55296"))]
        // static field: MIN_HIGH_SURROGATE:C
        pub fn MIN_HIGH_SURROGATE() -> u16 {
            55296
        }

        #[cfg_attr(any(), java_field(name = "MAX_HIGH_SURROGATE", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "56319"))]
        // static field: MAX_HIGH_SURROGATE:C
        pub fn MAX_HIGH_SURROGATE() -> u16 {
            56319
        }

        #[cfg_attr(any(), java_field(name = "MIN_LOW_SURROGATE", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "56320"))]
        // static field: MIN_LOW_SURROGATE:C
        pub fn MIN_LOW_SURROGATE() -> u16 {
            56320
        }

        #[cfg_attr(any(), java_field(name = "MAX_LOW_SURROGATE", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "57343"))]
        // static field: MAX_LOW_SURROGATE:C
        pub fn MAX_LOW_SURROGATE() -> u16 {
            57343
        }

        #[cfg_attr(any(), java_field(name = "MIN_SURROGATE", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "55296"))]
        // static field: MIN_SURROGATE:C
        pub fn MIN_SURROGATE() -> u16 {
            55296
        }

        #[cfg_attr(any(), java_field(name = "MAX_SURROGATE", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "57343"))]
        // static field: MAX_SURROGATE:C
        pub fn MAX_SURROGATE() -> u16 {
            57343
        }

        #[cfg_attr(any(), java_field(name = "MIN_SUPPLEMENTARY_CODE_POINT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "65536"))]
        // static field: MIN_SUPPLEMENTARY_CODE_POINT:I
        pub fn MIN_SUPPLEMENTARY_CODE_POINT() -> i32 {
            65536
        }

        #[cfg_attr(any(), java_field(name = "MIN_CODE_POINT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: MIN_CODE_POINT:I
        pub fn MIN_CODE_POINT() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "MAX_CODE_POINT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1114111"))]
        // static field: MAX_CODE_POINT:I
        pub fn MAX_CODE_POINT() -> i32 {
            1114111
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "3786198910865385080"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            3786198910865385080i64
        }

        #[cfg_attr(any(), java_field(name = "SIZE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: SIZE:I
        pub fn SIZE() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "BYTES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: BYTES:I
        pub fn BYTES() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/constant/DynamicConstantDesc<Ljava/lang/Character;>;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Character.describeConstable:()Ljava/util/Optional;")
        }

        #[java_method(name = "<init>", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new(mut value: u16) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_value(value);
            Ok(this)
        }

        #[java_method(name = "valueOf", descriptor = "(C)Ljava/lang/Character;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf(mut c: u16) -> Result<Character> {
            if (c as i32) <= 127i32 {
                return Ok(Clone::clone(&Character_CharacterCache::cache().borrow()[c as usize]));
            }
            Ok(Character::new(c)?)
        }

        #[java_method(name = "charValue", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charValue(&self) -> Result<u16> {
            panic!("stub: java/lang/Character.charValue:()C")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "hashCode", descriptor = "(C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_c(value: u16) -> Result<i32> {
            panic!("stub: java/lang/Character.hashCode:(C)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/Character.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString()Ljava/lang/String;
        pub fn toString(&self) -> Result<String> {
            let this = self;
            Ok(String::from_owned(format!("{}", this.__get_value())))
        }

        #[java_method(name = "toString", descriptor = "(C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_c(c: u16) -> Result<String> {
            panic!("stub: java/lang/Character.toString:(C)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_i(codePoint: i32) -> Result<String> {
            panic!("stub: java/lang/Character.toString:(I)Ljava/lang/String;")
        }

        #[java_method(name = "isValidCodePoint", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValidCodePoint(mut codePoint: i32) -> Result<bool> {
            let mut plane = ((codePoint as u32>>(16i32&0x1f)) as i32);
            Ok(plane < 17i32)
        }

        #[java_method(name = "isBmpCodePoint", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBmpCodePoint(mut codePoint: i32) -> Result<bool> {
            Ok((((codePoint as u32>>(16i32&0x1f)) as i32)==0))
        }

        #[java_method(name = "isSupplementaryCodePoint", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupplementaryCodePoint(mut codePoint: i32) -> Result<bool> {
            Ok((if codePoint >= 65536i32 { codePoint < 1114112i32 } else { (0i32 != 0) }))
        }

        #[java_method(name = "isHighSurrogate", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isHighSurrogate(mut ch: u16) -> Result<bool> {
            Ok((if (ch as i32) >= 55296i32 { (ch as i32) < 56320i32 } else { (0i32 != 0) }))
        }

        #[java_method(name = "isLowSurrogate", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLowSurrogate(mut ch: u16) -> Result<bool> {
            Ok((if (ch as i32) >= 56320i32 { (ch as i32) < 57344i32 } else { (0i32 != 0) }))
        }

        #[java_method(name = "isSurrogate", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSurrogate(mut ch: u16) -> Result<bool> {
            Ok((if (ch as i32) >= 55296i32 { (ch as i32) < 57344i32 } else { (0i32 != 0) }))
        }

        #[java_method(name = "isSurrogatePair", descriptor = "(CC)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSurrogatePair(high: u16, low: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isSurrogatePair:(CC)Z")
        }

        #[java_method(name = "charCount", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charCount(mut codePoint: i32) -> Result<i32> {
            Ok((codePoint < 65536i32) as i32)
        }

        #[java_method(name = "toCodePoint", descriptor = "(CC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toCodePoint(mut high: u16, mut low: u16) -> Result<i32> {
            Ok(((((high as i32)<<(10i32&0x1f))).wrapping_add((low as i32))).wrapping_add(-56613888i32))
        }

        #[java_method(name = "codePointAt", descriptor = "(Ljava/lang/CharSequence;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: codePointAt(Ljava/lang/CharSequence;I)I
        pub fn codePointAt_seq_i(mut seq: Object, mut index: i32) -> Result<i32> {
            let _vdispatch0: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt(index)? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(index)? } else { Default::default() };
            let mut c1: u16 = _vdispatch0;
            let _t1: bool = Character::isHighSurrogate(c1)?;
            index = index.wrapping_add(1i32);
            let _vdispatch2: i32 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let _vdispatch3: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt(index)? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(index)? } else { Default::default() };
            let mut c2: u16 = _vdispatch3;
            let _t4: bool = Character::isLowSurrogate(c2)?;
            if _t4 {
                let _t5: i32 = Character::toCodePoint(c1, c2)?;
                return Ok(_t5);
            }
            Ok((c1) as i32)
        }

        #[java_method(name = "codePointAt", descriptor = "([CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointAt_arr_c_i(a: Rc<RefCell<Vec<u16>>>, index: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointAt:([CI)I")
        }

        #[java_method(name = "codePointAt", descriptor = "([CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointAt_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, index: i32, limit: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointAt:([CII)I")
        }

        #[java_method(name = "codePointAtImpl", descriptor = "([CII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointAtImpl(a: Rc<RefCell<Vec<u16>>>, index: i32, limit: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointAtImpl:([CII)I")
        }

        #[java_method(name = "codePointBefore", descriptor = "(Ljava/lang/CharSequence;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointBefore_seq_i(seq: Object, index: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointBefore:(Ljava/lang/CharSequence;I)I")
        }

        #[java_method(name = "codePointBefore", descriptor = "([CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointBefore_arr_c_i(a: Rc<RefCell<Vec<u16>>>, index: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointBefore:([CI)I")
        }

        #[java_method(name = "codePointBefore", descriptor = "([CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointBefore_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, index: i32, start: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointBefore:([CII)I")
        }

        #[java_method(name = "codePointBeforeImpl", descriptor = "([CII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointBeforeImpl(a: Rc<RefCell<Vec<u16>>>, index: i32, start: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointBeforeImpl:([CII)I")
        }

        #[java_method(name = "highSurrogate", descriptor = "(I)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn highSurrogate(mut codePoint: i32) -> Result<u16> {
            Ok((((((((codePoint as u32>>(10i32&0x1f)) as i32)).wrapping_add(55232i32)) as u16 as i32)) as u16))
        }

        #[java_method(name = "lowSurrogate", descriptor = "(I)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lowSurrogate(mut codePoint: i32) -> Result<u16> {
            Ok(((((((codePoint&1023i32)).wrapping_add(56320i32)) as u16 as i32)) as u16))
        }

        #[java_method(name = "toChars", descriptor = "(I[CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars_i_arr_c_i(codePoint: i32, dst: Rc<RefCell<Vec<u16>>>, dstIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.toChars:(I[CI)I")
        }

        #[java_method(name = "toChars", descriptor = "(I)[C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toChars(I)[C
        pub fn toChars_i(mut codePoint: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            let _t0: bool = Character::isBmpCodePoint(codePoint)?;
            if _t0 {
                let mut _arr1: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; 1i32 as usize]));
                _arr1.borrow_mut()[0i32 as usize] = (((codePoint) as u16 as i32)) as u16;
                return Ok(_arr1);
            }
            let _t1: bool = Character::isValidCodePoint(codePoint)?;
            if _t1 {
                let mut _arr2: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; 2i32 as usize]));
                let mut result: Rc<RefCell<Vec<u16>>> = _arr2;
                Character::toSurrogates(codePoint, Clone::clone(&result), 0i32)?;
                return Ok(result);
            }
            let mut _arr2: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = codePoint.into();
            let _t3: String = String::format_str_arr_obj(Clone::clone(&String::from("Not a valid Unicode code point: 0x%X")), Clone::clone(&_arr2))?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "toSurrogates", descriptor = "(I[CI)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toSurrogates(mut codePoint: i32, mut dst: Rc<RefCell<Vec<u16>>>, mut index: i32) -> Result<()> {
            let _t0: u16 = Character::lowSurrogate(codePoint)?;
            dst.borrow_mut()[(index).wrapping_add(1i32) as usize] = (_t0) as u16;
            let _t1: u16 = Character::highSurrogate(codePoint)?;
            dst.borrow_mut()[index as usize] = (_t1) as u16;
            Ok(())
        }

        #[java_method(name = "codePointCount", descriptor = "(Ljava/lang/CharSequence;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointCount_seq_i_i(seq: Object, beginIndex: i32, endIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointCount:(Ljava/lang/CharSequence;II)I")
        }

        #[java_method(name = "codePointCount", descriptor = "([CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointCount_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointCount:([CII)I")
        }

        #[java_method(name = "codePointCountImpl", descriptor = "([CII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointCountImpl(a: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.codePointCountImpl:([CII)I")
        }

        #[java_method(name = "offsetByCodePoints", descriptor = "(Ljava/lang/CharSequence;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn offsetByCodePoints_seq_i_i(seq: Object, index: i32, codePointOffset: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.offsetByCodePoints:(Ljava/lang/CharSequence;II)I")
        }

        #[java_method(name = "offsetByCodePoints", descriptor = "([CIIII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn offsetByCodePoints_arr_c_i_i_i_i(a: Rc<RefCell<Vec<u16>>>, start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.offsetByCodePoints:([CIIII)I")
        }

        #[java_method(name = "offsetByCodePointsImpl", descriptor = "([CIIII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn offsetByCodePointsImpl(a: Rc<RefCell<Vec<u16>>>, start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.offsetByCodePointsImpl:([CIIII)I")
        }

        #[java_method(name = "isLowerCase", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLowerCase_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isLowerCase:(C)Z")
        }

        #[java_method(name = "isLowerCase", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLowerCase_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isLowerCase:(I)Z")
        }

        #[java_method(name = "isUpperCase", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: isUpperCase(C)Z
        pub fn isUpperCase_c(mut ch: u16) -> Result<bool> {
            let _t0: bool = Character::isUpperCase_i((ch as i32))?;
            Ok(_t0)
        }

        #[java_method(name = "isUpperCase", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: isUpperCase(I)Z
        pub fn isUpperCase_i(mut codePoint: i32) -> Result<bool> {
            let _t0: CharacterData = CharacterData::of(codePoint)?;
            let _t1 = _t0.isUpperCase(codePoint)?;
            Ok(_t1)
        }

        #[java_method(name = "isTitleCase", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isTitleCase_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isTitleCase:(C)Z")
        }

        #[java_method(name = "isTitleCase", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isTitleCase_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isTitleCase:(I)Z")
        }

        #[java_method(name = "isDigit", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDigit_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isDigit:(C)Z")
        }

        #[java_method(name = "isDigit", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDigit_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isDigit:(I)Z")
        }

        #[java_method(name = "isDefined", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDefined_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isDefined:(C)Z")
        }

        #[java_method(name = "isDefined", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDefined_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isDefined:(I)Z")
        }

        #[java_method(name = "isLetter", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLetter_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isLetter:(C)Z")
        }

        #[java_method(name = "isLetter", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLetter_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isLetter:(I)Z")
        }

        #[java_method(name = "isLetterOrDigit", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLetterOrDigit_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isLetterOrDigit:(C)Z")
        }

        #[java_method(name = "isLetterOrDigit", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLetterOrDigit_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isLetterOrDigit:(I)Z")
        }

        #[java_method(name = "isJavaLetter", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn isJavaLetter(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isJavaLetter:(C)Z")
        }

        #[java_method(name = "isJavaLetterOrDigit", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn isJavaLetterOrDigit(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isJavaLetterOrDigit:(C)Z")
        }

        #[java_method(name = "isAlphabetic", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAlphabetic(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isAlphabetic:(I)Z")
        }

        #[java_method(name = "isIdeographic", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdeographic(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isIdeographic:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierStart", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierStart_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isJavaIdentifierStart:(C)Z")
        }

        #[java_method(name = "isJavaIdentifierStart", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierStart_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isJavaIdentifierStart:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierPart", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierPart_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isJavaIdentifierPart:(C)Z")
        }

        #[java_method(name = "isJavaIdentifierPart", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierPart_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isJavaIdentifierPart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierStart", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierStart_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isUnicodeIdentifierStart:(C)Z")
        }

        #[java_method(name = "isUnicodeIdentifierStart", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierStart_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isUnicodeIdentifierStart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierPart", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierPart_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isUnicodeIdentifierPart:(C)Z")
        }

        #[java_method(name = "isUnicodeIdentifierPart", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierPart_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isUnicodeIdentifierPart:(I)Z")
        }

        #[java_method(name = "isIdentifierIgnorable", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdentifierIgnorable_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isIdentifierIgnorable:(C)Z")
        }

        #[java_method(name = "isIdentifierIgnorable", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdentifierIgnorable_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isIdentifierIgnorable:(I)Z")
        }

        #[java_method(name = "isEmoji", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmoji(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isEmoji:(I)Z")
        }

        #[java_method(name = "isEmojiPresentation", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiPresentation(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isEmojiPresentation:(I)Z")
        }

        #[java_method(name = "isEmojiModifier", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifier(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isEmojiModifier:(I)Z")
        }

        #[java_method(name = "isEmojiModifierBase", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifierBase(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isEmojiModifierBase:(I)Z")
        }

        #[java_method(name = "isEmojiComponent", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiComponent(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isEmojiComponent:(I)Z")
        }

        #[java_method(name = "isExtendedPictographic", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isExtendedPictographic(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isExtendedPictographic:(I)Z")
        }

        #[java_method(name = "toLowerCase", descriptor = "(C)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toLowerCase(C)C
        pub fn toLowerCase_c(mut ch: u16) -> Result<u16> {
            let _t0: i32 = Character::toLowerCase_i((ch as i32))?;
            Ok(((((_t0) as u16 as i32)) as u16))
        }

        #[java_method(name = "toLowerCase", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toLowerCase(I)I
        pub fn toLowerCase_i(mut codePoint: i32) -> Result<i32> {
            let _t0: CharacterData = CharacterData::of(codePoint)?;
            let _t1 = _t0.toLowerCase(codePoint)?;
            Ok(_t1)
        }

        #[java_method(name = "toUpperCase", descriptor = "(C)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toUpperCase(C)C
        pub fn toUpperCase_c(mut ch: u16) -> Result<u16> {
            let _t0: i32 = Character::toUpperCase_i((ch as i32))?;
            Ok(((((_t0) as u16 as i32)) as u16))
        }

        #[java_method(name = "toUpperCase", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toUpperCase(I)I
        pub fn toUpperCase_i(mut codePoint: i32) -> Result<i32> {
            let _t0: CharacterData = CharacterData::of(codePoint)?;
            let _t1 = _t0.toUpperCase(codePoint)?;
            Ok(_t1)
        }

        #[java_method(name = "toTitleCase", descriptor = "(C)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toTitleCase_c(ch: u16) -> Result<u16> {
            panic!("stub: java/lang/Character.toTitleCase:(C)C")
        }

        #[java_method(name = "toTitleCase", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toTitleCase_i(codePoint: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.toTitleCase:(I)I")
        }

        #[java_method(name = "digit", descriptor = "(CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: digit(CI)I
        pub fn digit_c_i(mut ch: u16, mut radix: i32) -> Result<i32> {
            let _t0: i32 = Character::digit_i_i((ch as i32), radix)?;
            Ok(_t0)
        }

        #[java_method(name = "digit", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: digit(II)I
        pub fn digit_i_i(mut codePoint: i32, mut radix: i32) -> Result<i32> {
            let _t0: CharacterData = CharacterData::of(codePoint)?;
            let _t1 = _t0.digit(codePoint, radix)?;
            Ok(_t1)
        }

        #[java_method(name = "getNumericValue", descriptor = "(C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumericValue_c(ch: u16) -> Result<i32> {
            panic!("stub: java/lang/Character.getNumericValue:(C)I")
        }

        #[java_method(name = "getNumericValue", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumericValue_i(codePoint: i32) -> Result<i32> {
            panic!("stub: java/lang/Character.getNumericValue:(I)I")
        }

        #[java_method(name = "isSpace", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn isSpace(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isSpace:(C)Z")
        }

        #[java_method(name = "isSpaceChar", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSpaceChar_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isSpaceChar:(C)Z")
        }

        #[java_method(name = "isSpaceChar", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSpaceChar_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isSpaceChar:(I)Z")
        }

        #[java_method(name = "isWhitespace", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWhitespace_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isWhitespace:(C)Z")
        }

        #[java_method(name = "isWhitespace", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: isWhitespace(I)Z
        pub fn isWhitespace_i(mut codePoint: i32) -> Result<bool> {
            let _t0: CharacterData = CharacterData::of(codePoint)?;
            let _t1 = _t0.isWhitespace(codePoint)?;
            Ok(_t1)
        }

        #[java_method(name = "isISOControl", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isISOControl_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isISOControl:(C)Z")
        }

        #[java_method(name = "isISOControl", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isISOControl_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isISOControl:(I)Z")
        }

        #[java_method(name = "getType", descriptor = "(C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getType(C)I
        pub fn getType_c(mut ch: u16) -> Result<i32> {
            let _t0: i32 = Character::getType_i((ch as i32))?;
            Ok(_t0)
        }

        #[java_method(name = "getType", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getType(I)I
        pub fn getType_i(mut codePoint: i32) -> Result<i32> {
            let _t0: CharacterData = CharacterData::of(codePoint)?;
            let _t1 = _t0.getType(codePoint)?;
            Ok(_t1)
        }

        #[java_method(name = "forDigit", descriptor = "(II)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forDigit(digit: i32, radix: i32) -> Result<u16> {
            panic!("stub: java/lang/Character.forDigit:(II)C")
        }

        #[java_method(name = "getDirectionality", descriptor = "(C)B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDirectionality_c(ch: u16) -> Result<i8> {
            panic!("stub: java/lang/Character.getDirectionality:(C)B")
        }

        #[java_method(name = "getDirectionality", descriptor = "(I)B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDirectionality_i(codePoint: i32) -> Result<i8> {
            panic!("stub: java/lang/Character.getDirectionality:(I)B")
        }

        #[java_method(name = "isMirrored", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMirrored_c(ch: u16) -> Result<bool> {
            panic!("stub: java/lang/Character.isMirrored:(C)Z")
        }

        #[java_method(name = "isMirrored", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMirrored_i(codePoint: i32) -> Result<bool> {
            panic!("stub: java/lang/Character.isMirrored:(I)Z")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/Character;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut anotherCharacter: Character) -> Result<i32> {
            let this = self;
            let _t0: i32 = Character::compare(this.__get_value(), anotherCharacter.__get_value())?;
            Ok(_t0)
        }

        #[java_method(name = "compare", descriptor = "(CC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(mut x: u16, mut y: u16) -> Result<i32> {
            Ok(((x as i32)).wrapping_sub((y as i32)))
        }

        #[java_method(name = "toUpperCaseEx", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseEx(mut codePoint: i32) -> Result<i32> {
            let _t0: bool = Character::isValidCodePoint(codePoint)?;
            if !(_t0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: CharacterData = CharacterData::of(codePoint)?;
            let _t2 = _t1.toUpperCaseEx(codePoint)?;
            Ok(_t2)
        }

        #[java_method(name = "toUpperCaseCharArray", descriptor = "(I)[C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseCharArray(mut codePoint: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            let _t0: bool = Character::isBmpCodePoint(codePoint)?;
            if !(_t0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: CharacterData = CharacterData::of(codePoint)?;
            let _t2 = _t1.toUpperCaseCharArray(codePoint)?;
            Ok(_t2)
        }

        #[java_method(name = "reverseBytes", descriptor = "(C)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reverseBytes(ch: u16) -> Result<u16> {
            panic!("stub: java/lang/Character.reverseBytes:(C)C")
        }

        #[java_method(name = "getName", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getName(mut codePoint: i32) -> Result<String> {
            let _t0: bool = Character::isValidCodePoint(codePoint)?;
            if !(_t0) {
                let mut _arr1: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr1.borrow_mut()[0i32 as usize] = codePoint.into();
                let _t2: String = String::format_str_arr_obj(Clone::clone(&String::from("Not a valid Unicode code point: 0x%X")), Clone::clone(&_arr1))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: CharacterName = CharacterName::getInstance()?;
            let _t2 = _t1.getName(codePoint)?;
            let mut name: String = _t2;
            if !_is_jnull(&name) {
                return Ok(name);
            }
            let _t3: i32 = Character::getType_i(codePoint)?;
            if (_t3==0) {
                return Ok(Default::default());
            }
            let _t4: Character_UnicodeBlock = Character_UnicodeBlock::of_i(codePoint)?;
            let mut block: Character_UnicodeBlock = _t4;
            if !_is_jnull(&block) {
                let _t5 = block.__super().toString()?;
                let _t6 = _t5.replace_c_c(((95i32) as u16), ((32i32) as u16))?;
                let _t7 = StringBuilder::new()?.append_str(Clone::clone(&_t6))?;
                let _t8 = _t7.append_str(Clone::clone(&String::from(" ")))?;
                let _t9: String = Integer::toHexString(codePoint)?;
                let _t10 = _t9.toUpperCase_locale(Clone::clone(&Locale::ROOT()))?;
                let _t11 = _t8.append_str(Clone::clone(&_t10))?;
                let _t12 = _t11.toString()?;
                return Ok(_t12);
            }
            let _t5: String = Integer::toHexString(codePoint)?;
            let _t6 = _t5.toUpperCase_locale(Clone::clone(&Locale::ROOT()))?;
            Ok(_t6)
        }

        #[java_method(name = "codePointOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointOf(mut name: String) -> Result<i32> {
            let _t0 = name.trim()?;
            let _t1 = _t0.toUpperCase_locale(Clone::clone(&Locale::ROOT()))?;
            name = _t1;
            let _t2: CharacterName = CharacterName::getInstance()?;
            let _t3 = _t2.getCodePoint(Clone::clone(&name))?;
            let mut cp: i32 = _t3;
            if cp != -1i32 {
                return Ok(cp);
            }
            let _t4 = name.lastIndexOf_i(32i32)?;
            let mut off: i32 = _t4;
            let _t5 = name.length()?;
            let _t6: i32 = Integer::parseInt_seq_i_i_i(Object::from_any(name.clone()), (off).wrapping_add(1i32), _t5, 16i32)?;
            cp = _t6;
            let _t7: bool = Character::isValidCodePoint(cp)?;
            let _t8: String = Character::getName(cp)?;
            let _t9 = name.equals(Object::from_any(_t8.clone()))?;
            if _t9 {
                return Ok(cp);
            }
            let _t10 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Unrecognized character name :")))?;
            let _t11 = _t10.append_str(Clone::clone(&name))?;
            let _t12 = _t11.toString()?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }
    }
}
