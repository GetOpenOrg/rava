#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Character",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable,java/lang/Comparable,java/lang/constant/Constable",
    access      = "public final",
    source      = "Character.java",
))]
pub struct Character {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "C", access = "private final"))]
    pub value: Field<u16>,
}

impl Character {
    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        todo!("abstract java/lang/Character.describeConstable")
    }

    // java: <init>(C)V
    pub fn new(&self, value: u16) -> Result<()> {
        todo!("abstract java/lang/Character.<init>")
    }

    // java: valueOf(C)Ljava/lang/Character;
    pub fn valueOf(c: u16) -> Result<Object> {
        todo!("abstract java/lang/Character.valueOf")
    }

    // java: charValue()C
    pub fn charValue(&self) -> Result<u16> {
        todo!("abstract java/lang/Character.charValue")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/Character.hashCode")
    }

    // java: hashCode(C)I
    pub fn hashCode__c(value: u16) -> Result<i32> {
        todo!("abstract java/lang/Character.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/lang/Character.equals")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/Character.toString")
    }

    // java: toString(C)Ljava/lang/String;
    pub fn toString__c(c: u16) -> Result<String> {
        todo!("abstract java/lang/Character.toString")
    }

    // java: toString(I)Ljava/lang/String;
    pub fn toString__i(codePoint: i32) -> Result<String> {
        todo!("abstract java/lang/Character.toString")
    }

    // java: isValidCodePoint(I)Z
    pub fn isValidCodePoint(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isValidCodePoint")
    }

    // java: isBmpCodePoint(I)Z
    pub fn isBmpCodePoint(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isBmpCodePoint")
    }

    // java: isSupplementaryCodePoint(I)Z
    pub fn isSupplementaryCodePoint(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isSupplementaryCodePoint")
    }

    // java: isHighSurrogate(C)Z
    pub fn isHighSurrogate(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isHighSurrogate")
    }

    // java: isLowSurrogate(C)Z
    pub fn isLowSurrogate(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isLowSurrogate")
    }

    // java: isSurrogate(C)Z
    pub fn isSurrogate(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isSurrogate")
    }

    // java: isSurrogatePair(CC)Z
    pub fn isSurrogatePair(high: u16, low: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isSurrogatePair")
    }

    // java: charCount(I)I
    pub fn charCount(codePoint: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.charCount")
    }

    // java: toCodePoint(CC)I
    pub fn toCodePoint(high: u16, low: u16) -> Result<i32> {
        todo!("abstract java/lang/Character.toCodePoint")
    }

    // java: codePointAt(Ljava/lang/CharSequence;I)I
    pub fn codePointAt__seq_i(seq: Object, index: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointAt")
    }

    // java: codePointAt([CI)I
    pub fn codePointAt__arr_c_i(a: Vec<u16>, index: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointAt")
    }

    // java: codePointAt([CII)I
    pub fn codePointAt__arr_c_i_i(a: Vec<u16>, index: i32, limit: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointAt")
    }

    // java: codePointAtImpl([CII)I
    pub fn codePointAtImpl(a: Vec<u16>, index: i32, limit: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointAtImpl")
    }

    // java: codePointBefore(Ljava/lang/CharSequence;I)I
    pub fn codePointBefore__seq_i(seq: Object, index: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointBefore")
    }

    // java: codePointBefore([CI)I
    pub fn codePointBefore__arr_c_i(a: Vec<u16>, index: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointBefore")
    }

    // java: codePointBefore([CII)I
    pub fn codePointBefore__arr_c_i_i(a: Vec<u16>, index: i32, start: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointBefore")
    }

    // java: codePointBeforeImpl([CII)I
    pub fn codePointBeforeImpl(a: Vec<u16>, index: i32, start: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointBeforeImpl")
    }

    // java: highSurrogate(I)C
    pub fn highSurrogate(codePoint: i32) -> Result<u16> {
        todo!("abstract java/lang/Character.highSurrogate")
    }

    // java: lowSurrogate(I)C
    pub fn lowSurrogate(codePoint: i32) -> Result<u16> {
        todo!("abstract java/lang/Character.lowSurrogate")
    }

    // java: toChars(I[CI)I
    pub fn toChars__i_arr_c_i(codePoint: i32, dst: Vec<u16>, dstIndex: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.toChars")
    }

    // java: toChars(I)[C
    pub fn toChars__i(codePoint: i32) -> Result<Vec<u16>> {
        todo!("abstract java/lang/Character.toChars")
    }

    // java: toSurrogates(I[CI)V
    pub fn toSurrogates(codePoint: i32, dst: Vec<u16>, index: i32) -> Result<()> {
        todo!("abstract java/lang/Character.toSurrogates")
    }

    // java: codePointCount(Ljava/lang/CharSequence;II)I
    pub fn codePointCount__seq_i_i(seq: Object, beginIndex: i32, endIndex: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointCount")
    }

    // java: codePointCount([CII)I
    pub fn codePointCount__arr_c_i_i(a: Vec<u16>, offset: i32, count: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointCount")
    }

    // java: codePointCountImpl([CII)I
    pub fn codePointCountImpl(a: Vec<u16>, offset: i32, count: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointCountImpl")
    }

    // java: offsetByCodePoints(Ljava/lang/CharSequence;II)I
    pub fn offsetByCodePoints__seq_i_i(seq: Object, index: i32, codePointOffset: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.offsetByCodePoints")
    }

    // java: offsetByCodePoints([CIIII)I
    pub fn offsetByCodePoints__arr_c_i_i_i_i(a: Vec<u16>, start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.offsetByCodePoints")
    }

    // java: offsetByCodePointsImpl([CIIII)I
    pub fn offsetByCodePointsImpl(a: Vec<u16>, start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.offsetByCodePointsImpl")
    }

    // java: isLowerCase(C)Z
    pub fn isLowerCase__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isLowerCase")
    }

    // java: isLowerCase(I)Z
    pub fn isLowerCase__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isLowerCase")
    }

    // java: isUpperCase(C)Z
    pub fn isUpperCase__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isUpperCase")
    }

    // java: isUpperCase(I)Z
    pub fn isUpperCase__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isUpperCase")
    }

    // java: isTitleCase(C)Z
    pub fn isTitleCase__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isTitleCase")
    }

    // java: isTitleCase(I)Z
    pub fn isTitleCase__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isTitleCase")
    }

    // java: isDigit(C)Z
    pub fn isDigit__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isDigit")
    }

    // java: isDigit(I)Z
    pub fn isDigit__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isDigit")
    }

    // java: isDefined(C)Z
    pub fn isDefined__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isDefined")
    }

    // java: isDefined(I)Z
    pub fn isDefined__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isDefined")
    }

    // java: isLetter(C)Z
    pub fn isLetter__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isLetter")
    }

    // java: isLetter(I)Z
    pub fn isLetter__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isLetter")
    }

    // java: isLetterOrDigit(C)Z
    pub fn isLetterOrDigit__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isLetterOrDigit")
    }

    // java: isLetterOrDigit(I)Z
    pub fn isLetterOrDigit__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isLetterOrDigit")
    }

    // java: isJavaLetter(C)Z
    pub fn isJavaLetter(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isJavaLetter")
    }

    // java: isJavaLetterOrDigit(C)Z
    pub fn isJavaLetterOrDigit(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isJavaLetterOrDigit")
    }

    // java: isAlphabetic(I)Z
    pub fn isAlphabetic(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isAlphabetic")
    }

    // java: isIdeographic(I)Z
    pub fn isIdeographic(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isIdeographic")
    }

    // java: isJavaIdentifierStart(C)Z
    pub fn isJavaIdentifierStart__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isJavaIdentifierStart")
    }

    // java: isJavaIdentifierStart(I)Z
    pub fn isJavaIdentifierStart__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isJavaIdentifierStart")
    }

    // java: isJavaIdentifierPart(C)Z
    pub fn isJavaIdentifierPart__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isJavaIdentifierPart")
    }

    // java: isJavaIdentifierPart(I)Z
    pub fn isJavaIdentifierPart__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isJavaIdentifierPart")
    }

    // java: isUnicodeIdentifierStart(C)Z
    pub fn isUnicodeIdentifierStart__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isUnicodeIdentifierStart")
    }

    // java: isUnicodeIdentifierStart(I)Z
    pub fn isUnicodeIdentifierStart__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isUnicodeIdentifierStart")
    }

    // java: isUnicodeIdentifierPart(C)Z
    pub fn isUnicodeIdentifierPart__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isUnicodeIdentifierPart")
    }

    // java: isUnicodeIdentifierPart(I)Z
    pub fn isUnicodeIdentifierPart__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isUnicodeIdentifierPart")
    }

    // java: isIdentifierIgnorable(C)Z
    pub fn isIdentifierIgnorable__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isIdentifierIgnorable")
    }

    // java: isIdentifierIgnorable(I)Z
    pub fn isIdentifierIgnorable__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isIdentifierIgnorable")
    }

    // java: isEmoji(I)Z
    pub fn isEmoji(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isEmoji")
    }

    // java: isEmojiPresentation(I)Z
    pub fn isEmojiPresentation(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isEmojiPresentation")
    }

    // java: isEmojiModifier(I)Z
    pub fn isEmojiModifier(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isEmojiModifier")
    }

    // java: isEmojiModifierBase(I)Z
    pub fn isEmojiModifierBase(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isEmojiModifierBase")
    }

    // java: isEmojiComponent(I)Z
    pub fn isEmojiComponent(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isEmojiComponent")
    }

    // java: isExtendedPictographic(I)Z
    pub fn isExtendedPictographic(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isExtendedPictographic")
    }

    // java: toLowerCase(C)C
    pub fn toLowerCase__c(ch: u16) -> Result<u16> {
        todo!("abstract java/lang/Character.toLowerCase")
    }

    // java: toLowerCase(I)I
    pub fn toLowerCase__i(codePoint: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.toLowerCase")
    }

    // java: toUpperCase(C)C
    pub fn toUpperCase__c(ch: u16) -> Result<u16> {
        todo!("abstract java/lang/Character.toUpperCase")
    }

    // java: toUpperCase(I)I
    pub fn toUpperCase__i(codePoint: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.toUpperCase")
    }

    // java: toTitleCase(C)C
    pub fn toTitleCase__c(ch: u16) -> Result<u16> {
        todo!("abstract java/lang/Character.toTitleCase")
    }

    // java: toTitleCase(I)I
    pub fn toTitleCase__i(codePoint: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.toTitleCase")
    }

    // java: digit(CI)I
    pub fn digit__c_i(ch: u16, radix: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.digit")
    }

    // java: digit(II)I
    pub fn digit__i_i(codePoint: i32, radix: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.digit")
    }

    // java: getNumericValue(C)I
    pub fn getNumericValue__c(ch: u16) -> Result<i32> {
        todo!("abstract java/lang/Character.getNumericValue")
    }

    // java: getNumericValue(I)I
    pub fn getNumericValue__i(codePoint: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.getNumericValue")
    }

    // java: isSpace(C)Z
    pub fn isSpace(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isSpace")
    }

    // java: isSpaceChar(C)Z
    pub fn isSpaceChar__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isSpaceChar")
    }

    // java: isSpaceChar(I)Z
    pub fn isSpaceChar__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isSpaceChar")
    }

    // java: isWhitespace(C)Z
    pub fn isWhitespace__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isWhitespace")
    }

    // java: isWhitespace(I)Z
    pub fn isWhitespace__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isWhitespace")
    }

    // java: isISOControl(C)Z
    pub fn isISOControl__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isISOControl")
    }

    // java: isISOControl(I)Z
    pub fn isISOControl__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isISOControl")
    }

    // java: getType(C)I
    pub fn getType__c(ch: u16) -> Result<i32> {
        todo!("abstract java/lang/Character.getType")
    }

    // java: getType(I)I
    pub fn getType__i(codePoint: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.getType")
    }

    // java: forDigit(II)C
    pub fn forDigit(digit: i32, radix: i32) -> Result<u16> {
        todo!("abstract java/lang/Character.forDigit")
    }

    // java: getDirectionality(C)B
    pub fn getDirectionality__c(ch: u16) -> Result<i8> {
        todo!("abstract java/lang/Character.getDirectionality")
    }

    // java: getDirectionality(I)B
    pub fn getDirectionality__i(codePoint: i32) -> Result<i8> {
        todo!("abstract java/lang/Character.getDirectionality")
    }

    // java: isMirrored(C)Z
    pub fn isMirrored__c(ch: u16) -> Result<bool> {
        todo!("abstract java/lang/Character.isMirrored")
    }

    // java: isMirrored(I)Z
    pub fn isMirrored__i(codePoint: i32) -> Result<bool> {
        todo!("abstract java/lang/Character.isMirrored")
    }

    // java: compareTo(Ljava/lang/Character;)I
    pub fn compareTo(&self, anotherCharacter: Object) -> Result<i32> {
        todo!("abstract java/lang/Character.compareTo")
    }

    // java: compare(CC)I
    pub fn compare(x: u16, y: u16) -> Result<i32> {
        todo!("abstract java/lang/Character.compare")
    }

    // java: toUpperCaseEx(I)I
    pub fn toUpperCaseEx(codePoint: i32) -> Result<i32> {
        todo!("abstract java/lang/Character.toUpperCaseEx")
    }

    // java: toUpperCaseCharArray(I)[C
    pub fn toUpperCaseCharArray(codePoint: i32) -> Result<Vec<u16>> {
        todo!("abstract java/lang/Character.toUpperCaseCharArray")
    }

    // java: reverseBytes(C)C
    pub fn reverseBytes(ch: u16) -> Result<u16> {
        todo!("abstract java/lang/Character.reverseBytes")
    }

    // java: getName(I)Ljava/lang/String;
    pub fn getName(codePoint: i32) -> Result<String> {
        todo!("abstract java/lang/Character.getName")
    }

    // java: codePointOf(Ljava/lang/String;)I
    pub fn codePointOf(name: String) -> Result<i32> {
        todo!("abstract java/lang/Character.codePointOf")
    }
}
