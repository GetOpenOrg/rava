#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Character",
    super_class       = "java/lang/Object",
    interfaces        = "java/io/Serializable,java/lang/Comparable,java/lang/constant/Constable",
    access            = "public",
    modifiers         = "final",
    generic_signature = "Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/Comparable<Ljava/lang/Character;>;Ljava/lang/constant/Constable;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Character.java",
    inner_classes     = "java/lang/Character$CharacterCache:java/lang/Character:CharacterCache:26;java/lang/Character$UnicodeBlock:java/lang/Character:UnicodeBlock:25;java/lang/Character$UnicodeScript:java/lang/Character:UnicodeScript:16409;java/lang/Character$Subset:java/lang/Character:Subset:9",
    all_supertypes    = "java/io/Serializable;java/lang/Character;java/lang/Comparable;java/lang/Object;java/lang/constant/Constable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Character {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "C", access = "private", modifiers = "final", is_static = false))]
    pub value: JField<u16>,
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
    pub fn TYPE() -> Object {
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

    #[cfg_attr(any(), java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/constant/DynamicConstantDesc<Ljava/lang/Character;>;>;"))]
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Character.describeConstable:()Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn new(value: u16) -> Result<Self> {
        panic!("stub: java/lang/Character.<init>:(C)V")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(C)Ljava/lang/Character;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf(c: u16) -> Result<Character> {
        panic!("stub: java/lang/Character.valueOf:(C)Ljava/lang/Character;")
    }

    #[cfg_attr(any(), java_method(name = "charValue", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn charValue(&self) -> Result<u16> {
        panic!("stub: java/lang/Character.charValue:()C")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Character.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode_c(value: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.hashCode:(C)I")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Character.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Character.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString_c(c: u16) -> Result<String> {
        panic!("stub: java/lang/Character.toString:(C)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString_i(codePoint: i32) -> Result<String> {
        panic!("stub: java/lang/Character.toString:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "isValidCodePoint", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isValidCodePoint(mut codePoint: i32) -> Result<bool> {
        let mut plane = ((codePoint as u32>>(16i32&0x1f)) as i32);
        Ok(plane < 17i32)
    }

    #[cfg_attr(any(), java_method(name = "isBmpCodePoint", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isBmpCodePoint(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isBmpCodePoint:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isSupplementaryCodePoint", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSupplementaryCodePoint(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isSupplementaryCodePoint:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isHighSurrogate", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isHighSurrogate(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isHighSurrogate:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isLowSurrogate", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isLowSurrogate(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isLowSurrogate:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isSurrogate", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSurrogate(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isSurrogate:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isSurrogatePair", descriptor = "(CC)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSurrogatePair(high: u16, low: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isSurrogatePair:(CC)Z")
    }

    #[cfg_attr(any(), java_method(name = "charCount", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn charCount(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.charCount:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "toCodePoint", descriptor = "(CC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toCodePoint(high: u16, low: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.toCodePoint:(CC)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "(Ljava/lang/CharSequence;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointAt_seq_i(seq: Object, index: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointAt:(Ljava/lang/CharSequence;I)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "([CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointAt_arr_c_i(a: Rc<RefCell<Vec<u16>>>, index: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointAt:([CI)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "([CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointAt_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, index: i32, limit: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointAt:([CII)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointAtImpl", descriptor = "([CII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointAtImpl(a: Rc<RefCell<Vec<u16>>>, index: i32, limit: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointAtImpl:([CII)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "(Ljava/lang/CharSequence;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointBefore_seq_i(seq: Object, index: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointBefore:(Ljava/lang/CharSequence;I)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "([CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointBefore_arr_c_i(a: Rc<RefCell<Vec<u16>>>, index: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointBefore:([CI)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "([CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointBefore_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, index: i32, start: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointBefore:([CII)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointBeforeImpl", descriptor = "([CII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointBeforeImpl(a: Rc<RefCell<Vec<u16>>>, index: i32, start: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointBeforeImpl:([CII)I")
    }

    #[cfg_attr(any(), java_method(name = "highSurrogate", descriptor = "(I)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn highSurrogate(mut codePoint: i32) -> Result<u16> {
        Ok((((((((codePoint as u32>>(10i32&0x1f)) as i32)).wrapping_add(55232i32)) as u16 as i32)) as u16))
    }

    #[cfg_attr(any(), java_method(name = "lowSurrogate", descriptor = "(I)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lowSurrogate(mut codePoint: i32) -> Result<u16> {
        Ok(((((((codePoint&1023i32)).wrapping_add(56320i32)) as u16 as i32)) as u16))
    }

    #[cfg_attr(any(), java_method(name = "toChars", descriptor = "(I[CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toChars_i_arr_c_i(codePoint: i32, dst: Rc<RefCell<Vec<u16>>>, dstIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toChars:(I[CI)I")
    }

    #[cfg_attr(any(), java_method(name = "toChars", descriptor = "(I)[C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toChars_i(codePoint: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
        panic!("stub: java/lang/Character.toChars:(I)[C")
    }

    #[cfg_attr(any(), java_method(name = "toSurrogates", descriptor = "(I[CI)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toSurrogates(codePoint: i32, dst: Rc<RefCell<Vec<u16>>>, index: i32) -> Result<()> {
        panic!("stub: java/lang/Character.toSurrogates:(I[CI)V")
    }

    #[cfg_attr(any(), java_method(name = "codePointCount", descriptor = "(Ljava/lang/CharSequence;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointCount_seq_i_i(seq: Object, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointCount:(Ljava/lang/CharSequence;II)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointCount", descriptor = "([CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointCount_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointCount:([CII)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointCountImpl", descriptor = "([CII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointCountImpl(a: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointCountImpl:([CII)I")
    }

    #[cfg_attr(any(), java_method(name = "offsetByCodePoints", descriptor = "(Ljava/lang/CharSequence;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn offsetByCodePoints_seq_i_i(seq: Object, index: i32, codePointOffset: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.offsetByCodePoints:(Ljava/lang/CharSequence;II)I")
    }

    #[cfg_attr(any(), java_method(name = "offsetByCodePoints", descriptor = "([CIIII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn offsetByCodePoints_arr_c_i_i_i_i(a: Rc<RefCell<Vec<u16>>>, start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.offsetByCodePoints:([CIIII)I")
    }

    #[cfg_attr(any(), java_method(name = "offsetByCodePointsImpl", descriptor = "([CIIII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn offsetByCodePointsImpl(a: Rc<RefCell<Vec<u16>>>, start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.offsetByCodePointsImpl:([CIIII)I")
    }

    #[cfg_attr(any(), java_method(name = "isLowerCase", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isLowerCase_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isLowerCase:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isLowerCase", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isLowerCase_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isLowerCase:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isUpperCase", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isUpperCase_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isUpperCase:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isUpperCase", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isUpperCase_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isUpperCase:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isTitleCase", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isTitleCase_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isTitleCase:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isTitleCase", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isTitleCase_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isTitleCase:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isDigit", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isDigit_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isDigit:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isDigit", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isDigit_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isDigit:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isDefined", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isDefined_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isDefined:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isDefined", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isDefined_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isDefined:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isLetter", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isLetter_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isLetter:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isLetter", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isLetter_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isLetter:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isLetterOrDigit", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isLetterOrDigit_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isLetterOrDigit:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isLetterOrDigit", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isLetterOrDigit_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isLetterOrDigit:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isJavaLetter", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn isJavaLetter(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaLetter:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isJavaLetterOrDigit", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn isJavaLetterOrDigit(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaLetterOrDigit:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isAlphabetic", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isAlphabetic(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isAlphabetic:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isIdeographic", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isIdeographic(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isIdeographic:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isJavaIdentifierStart", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isJavaIdentifierStart_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaIdentifierStart:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isJavaIdentifierStart", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isJavaIdentifierStart_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaIdentifierStart:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isJavaIdentifierPart", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isJavaIdentifierPart_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaIdentifierPart:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isJavaIdentifierPart", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isJavaIdentifierPart_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaIdentifierPart:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isUnicodeIdentifierStart", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isUnicodeIdentifierStart_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isUnicodeIdentifierStart:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isUnicodeIdentifierStart", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isUnicodeIdentifierStart_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isUnicodeIdentifierStart:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isUnicodeIdentifierPart", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isUnicodeIdentifierPart_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isUnicodeIdentifierPart:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isUnicodeIdentifierPart", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isUnicodeIdentifierPart_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isUnicodeIdentifierPart:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isIdentifierIgnorable", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isIdentifierIgnorable_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isIdentifierIgnorable:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isIdentifierIgnorable", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isIdentifierIgnorable_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isIdentifierIgnorable:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isEmoji", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmoji(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmoji:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isEmojiPresentation", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmojiPresentation(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmojiPresentation:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isEmojiModifier", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmojiModifier(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmojiModifier:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isEmojiModifierBase", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmojiModifierBase(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmojiModifierBase:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isEmojiComponent", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmojiComponent(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmojiComponent:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isExtendedPictographic", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isExtendedPictographic(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isExtendedPictographic:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "toLowerCase", descriptor = "(C)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toLowerCase_c(ch: u16) -> Result<u16> {
        panic!("stub: java/lang/Character.toLowerCase:(C)C")
    }

    #[cfg_attr(any(), java_method(name = "toLowerCase", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toLowerCase_i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toLowerCase:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "toUpperCase", descriptor = "(C)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUpperCase_c(ch: u16) -> Result<u16> {
        panic!("stub: java/lang/Character.toUpperCase:(C)C")
    }

    #[cfg_attr(any(), java_method(name = "toUpperCase", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUpperCase_i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toUpperCase:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "toTitleCase", descriptor = "(C)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toTitleCase_c(ch: u16) -> Result<u16> {
        panic!("stub: java/lang/Character.toTitleCase:(C)C")
    }

    #[cfg_attr(any(), java_method(name = "toTitleCase", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toTitleCase_i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toTitleCase:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "digit", descriptor = "(CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn digit_c_i(ch: u16, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.digit:(CI)I")
    }

    #[cfg_attr(any(), java_method(name = "digit", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn digit_i_i(codePoint: i32, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.digit:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "getNumericValue", descriptor = "(C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getNumericValue_c(ch: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.getNumericValue:(C)I")
    }

    #[cfg_attr(any(), java_method(name = "getNumericValue", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getNumericValue_i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.getNumericValue:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "isSpace", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn isSpace(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isSpace:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isSpaceChar", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSpaceChar_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isSpaceChar:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isSpaceChar", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSpaceChar_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isSpaceChar:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isWhitespace", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isWhitespace_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isWhitespace:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isWhitespace", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isWhitespace_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isWhitespace:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isISOControl", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isISOControl_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isISOControl:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isISOControl", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isISOControl_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isISOControl:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "getType", descriptor = "(C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getType_c(ch: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.getType:(C)I")
    }

    #[cfg_attr(any(), java_method(name = "getType", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getType_i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.getType:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "forDigit", descriptor = "(II)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn forDigit(digit: i32, radix: i32) -> Result<u16> {
        panic!("stub: java/lang/Character.forDigit:(II)C")
    }

    #[cfg_attr(any(), java_method(name = "getDirectionality", descriptor = "(C)B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getDirectionality_c(ch: u16) -> Result<i8> {
        panic!("stub: java/lang/Character.getDirectionality:(C)B")
    }

    #[cfg_attr(any(), java_method(name = "getDirectionality", descriptor = "(I)B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getDirectionality_i(codePoint: i32) -> Result<i8> {
        panic!("stub: java/lang/Character.getDirectionality:(I)B")
    }

    #[cfg_attr(any(), java_method(name = "isMirrored", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isMirrored_c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isMirrored:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "isMirrored", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isMirrored_i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isMirrored:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/Character;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareTo(&self, anotherCharacter: Character) -> Result<i32> {
        panic!("stub: java/lang/Character.compareTo:(Ljava/lang/Character;)I")
    }

    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(CC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compare(x: u16, y: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.compare:(CC)I")
    }

    #[cfg_attr(any(), java_method(name = "toUpperCaseEx", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUpperCaseEx(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toUpperCaseEx:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "toUpperCaseCharArray", descriptor = "(I)[C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUpperCaseCharArray(codePoint: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
        panic!("stub: java/lang/Character.toUpperCaseCharArray:(I)[C")
    }

    #[cfg_attr(any(), java_method(name = "reverseBytes", descriptor = "(C)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reverseBytes(ch: u16) -> Result<u16> {
        panic!("stub: java/lang/Character.reverseBytes:(C)C")
    }

    #[cfg_attr(any(), java_method(name = "getName", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getName(codePoint: i32) -> Result<String> {
        panic!("stub: java/lang/Character.getName:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "codePointOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointOf(name: String) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointOf:(Ljava/lang/String;)I")
    }
}
