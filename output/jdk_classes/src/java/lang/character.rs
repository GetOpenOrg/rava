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
        panic!("stub: java/lang/Character.describeConstable:()Ljava/util/Optional;")
    }

    // java: <init>(C)V
    pub fn new(&self, value: u16) -> Result<()> {
        panic!("stub: java/lang/Character.<init>:(C)V")
    }

    // java: valueOf(C)Ljava/lang/Character;
    pub fn valueOf(c: u16) -> Result<Object> {
        panic!("stub: java/lang/Character.valueOf:(C)Ljava/lang/Character;")
    }

    // java: charValue()C
    pub fn charValue(&self) -> Result<u16> {
        panic!("stub: java/lang/Character.charValue:()C")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Character.hashCode:()I")
    }

    // java: hashCode(C)I
    pub fn hashCode__c(value: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.hashCode:(C)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Character.equals:(Ljava/lang/Object;)Z")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Character.toString:()Ljava/lang/String;")
    }

    // java: toString(C)Ljava/lang/String;
    pub fn toString__c(c: u16) -> Result<String> {
        panic!("stub: java/lang/Character.toString:(C)Ljava/lang/String;")
    }

    // java: toString(I)Ljava/lang/String;
    pub fn toString__i(codePoint: i32) -> Result<String> {
        panic!("stub: java/lang/Character.toString:(I)Ljava/lang/String;")
    }

    // java: isValidCodePoint(I)Z
    pub fn isValidCodePoint(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isValidCodePoint:(I)Z")
    }

    // java: isBmpCodePoint(I)Z
    pub fn isBmpCodePoint(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isBmpCodePoint:(I)Z")
    }

    // java: isSupplementaryCodePoint(I)Z
    pub fn isSupplementaryCodePoint(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isSupplementaryCodePoint:(I)Z")
    }

    // java: isHighSurrogate(C)Z
    pub fn isHighSurrogate(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isHighSurrogate:(C)Z")
    }

    // java: isLowSurrogate(C)Z
    pub fn isLowSurrogate(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isLowSurrogate:(C)Z")
    }

    // java: isSurrogate(C)Z
    pub fn isSurrogate(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isSurrogate:(C)Z")
    }

    // java: isSurrogatePair(CC)Z
    pub fn isSurrogatePair(high: u16, low: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isSurrogatePair:(CC)Z")
    }

    // java: charCount(I)I
    pub fn charCount(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.charCount:(I)I")
    }

    // java: toCodePoint(CC)I
    pub fn toCodePoint(high: u16, low: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.toCodePoint:(CC)I")
    }

    // java: codePointAt(Ljava/lang/CharSequence;I)I
    pub fn codePointAt__seq_i(seq: Object, index: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointAt:(Ljava/lang/CharSequence;I)I")
    }

    // java: codePointAt([CI)I
    pub fn codePointAt__arr_c_i(a: Vec<u16>, index: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointAt:([CI)I")
    }

    // java: codePointAt([CII)I
    pub fn codePointAt__arr_c_i_i(a: Vec<u16>, index: i32, limit: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointAt:([CII)I")
    }

    // java: codePointAtImpl([CII)I
    pub fn codePointAtImpl(a: Vec<u16>, index: i32, limit: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointAtImpl:([CII)I")
    }

    // java: codePointBefore(Ljava/lang/CharSequence;I)I
    pub fn codePointBefore__seq_i(seq: Object, index: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointBefore:(Ljava/lang/CharSequence;I)I")
    }

    // java: codePointBefore([CI)I
    pub fn codePointBefore__arr_c_i(a: Vec<u16>, index: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointBefore:([CI)I")
    }

    // java: codePointBefore([CII)I
    pub fn codePointBefore__arr_c_i_i(a: Vec<u16>, index: i32, start: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointBefore:([CII)I")
    }

    // java: codePointBeforeImpl([CII)I
    pub fn codePointBeforeImpl(a: Vec<u16>, index: i32, start: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointBeforeImpl:([CII)I")
    }

    // java: highSurrogate(I)C
    pub fn highSurrogate(codePoint: i32) -> Result<u16> {
        panic!("stub: java/lang/Character.highSurrogate:(I)C")
    }

    // java: lowSurrogate(I)C
    pub fn lowSurrogate(codePoint: i32) -> Result<u16> {
        panic!("stub: java/lang/Character.lowSurrogate:(I)C")
    }

    // java: toChars(I[CI)I
    pub fn toChars__i_arr_c_i(codePoint: i32, dst: Vec<u16>, dstIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toChars:(I[CI)I")
    }

    // java: toChars(I)[C
    pub fn toChars__i(codePoint: i32) -> Result<Vec<u16>> {
        panic!("stub: java/lang/Character.toChars:(I)[C")
    }

    // java: toSurrogates(I[CI)V
    pub fn toSurrogates(codePoint: i32, dst: Vec<u16>, index: i32) -> Result<()> {
        panic!("stub: java/lang/Character.toSurrogates:(I[CI)V")
    }

    // java: codePointCount(Ljava/lang/CharSequence;II)I
    pub fn codePointCount__seq_i_i(seq: Object, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointCount:(Ljava/lang/CharSequence;II)I")
    }

    // java: codePointCount([CII)I
    pub fn codePointCount__arr_c_i_i(a: Vec<u16>, offset: i32, count: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointCount:([CII)I")
    }

    // java: codePointCountImpl([CII)I
    pub fn codePointCountImpl(a: Vec<u16>, offset: i32, count: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointCountImpl:([CII)I")
    }

    // java: offsetByCodePoints(Ljava/lang/CharSequence;II)I
    pub fn offsetByCodePoints__seq_i_i(seq: Object, index: i32, codePointOffset: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.offsetByCodePoints:(Ljava/lang/CharSequence;II)I")
    }

    // java: offsetByCodePoints([CIIII)I
    pub fn offsetByCodePoints__arr_c_i_i_i_i(a: Vec<u16>, start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.offsetByCodePoints:([CIIII)I")
    }

    // java: offsetByCodePointsImpl([CIIII)I
    pub fn offsetByCodePointsImpl(a: Vec<u16>, start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.offsetByCodePointsImpl:([CIIII)I")
    }

    // java: isLowerCase(C)Z
    pub fn isLowerCase__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isLowerCase:(C)Z")
    }

    // java: isLowerCase(I)Z
    pub fn isLowerCase__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isLowerCase:(I)Z")
    }

    // java: isUpperCase(C)Z
    pub fn isUpperCase__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isUpperCase:(C)Z")
    }

    // java: isUpperCase(I)Z
    pub fn isUpperCase__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isUpperCase:(I)Z")
    }

    // java: isTitleCase(C)Z
    pub fn isTitleCase__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isTitleCase:(C)Z")
    }

    // java: isTitleCase(I)Z
    pub fn isTitleCase__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isTitleCase:(I)Z")
    }

    // java: isDigit(C)Z
    pub fn isDigit__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isDigit:(C)Z")
    }

    // java: isDigit(I)Z
    pub fn isDigit__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isDigit:(I)Z")
    }

    // java: isDefined(C)Z
    pub fn isDefined__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isDefined:(C)Z")
    }

    // java: isDefined(I)Z
    pub fn isDefined__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isDefined:(I)Z")
    }

    // java: isLetter(C)Z
    pub fn isLetter__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isLetter:(C)Z")
    }

    // java: isLetter(I)Z
    pub fn isLetter__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isLetter:(I)Z")
    }

    // java: isLetterOrDigit(C)Z
    pub fn isLetterOrDigit__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isLetterOrDigit:(C)Z")
    }

    // java: isLetterOrDigit(I)Z
    pub fn isLetterOrDigit__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isLetterOrDigit:(I)Z")
    }

    // java: isJavaLetter(C)Z
    pub fn isJavaLetter(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaLetter:(C)Z")
    }

    // java: isJavaLetterOrDigit(C)Z
    pub fn isJavaLetterOrDigit(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaLetterOrDigit:(C)Z")
    }

    // java: isAlphabetic(I)Z
    pub fn isAlphabetic(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isAlphabetic:(I)Z")
    }

    // java: isIdeographic(I)Z
    pub fn isIdeographic(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isIdeographic:(I)Z")
    }

    // java: isJavaIdentifierStart(C)Z
    pub fn isJavaIdentifierStart__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaIdentifierStart:(C)Z")
    }

    // java: isJavaIdentifierStart(I)Z
    pub fn isJavaIdentifierStart__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaIdentifierStart:(I)Z")
    }

    // java: isJavaIdentifierPart(C)Z
    pub fn isJavaIdentifierPart__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaIdentifierPart:(C)Z")
    }

    // java: isJavaIdentifierPart(I)Z
    pub fn isJavaIdentifierPart__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isJavaIdentifierPart:(I)Z")
    }

    // java: isUnicodeIdentifierStart(C)Z
    pub fn isUnicodeIdentifierStart__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isUnicodeIdentifierStart:(C)Z")
    }

    // java: isUnicodeIdentifierStart(I)Z
    pub fn isUnicodeIdentifierStart__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isUnicodeIdentifierStart:(I)Z")
    }

    // java: isUnicodeIdentifierPart(C)Z
    pub fn isUnicodeIdentifierPart__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isUnicodeIdentifierPart:(C)Z")
    }

    // java: isUnicodeIdentifierPart(I)Z
    pub fn isUnicodeIdentifierPart__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isUnicodeIdentifierPart:(I)Z")
    }

    // java: isIdentifierIgnorable(C)Z
    pub fn isIdentifierIgnorable__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isIdentifierIgnorable:(C)Z")
    }

    // java: isIdentifierIgnorable(I)Z
    pub fn isIdentifierIgnorable__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isIdentifierIgnorable:(I)Z")
    }

    // java: isEmoji(I)Z
    pub fn isEmoji(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmoji:(I)Z")
    }

    // java: isEmojiPresentation(I)Z
    pub fn isEmojiPresentation(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmojiPresentation:(I)Z")
    }

    // java: isEmojiModifier(I)Z
    pub fn isEmojiModifier(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmojiModifier:(I)Z")
    }

    // java: isEmojiModifierBase(I)Z
    pub fn isEmojiModifierBase(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmojiModifierBase:(I)Z")
    }

    // java: isEmojiComponent(I)Z
    pub fn isEmojiComponent(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isEmojiComponent:(I)Z")
    }

    // java: isExtendedPictographic(I)Z
    pub fn isExtendedPictographic(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isExtendedPictographic:(I)Z")
    }

    // java: toLowerCase(C)C
    pub fn toLowerCase__c(ch: u16) -> Result<u16> {
        panic!("stub: java/lang/Character.toLowerCase:(C)C")
    }

    // java: toLowerCase(I)I
    pub fn toLowerCase__i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toLowerCase:(I)I")
    }

    // java: toUpperCase(C)C
    pub fn toUpperCase__c(ch: u16) -> Result<u16> {
        panic!("stub: java/lang/Character.toUpperCase:(C)C")
    }

    // java: toUpperCase(I)I
    pub fn toUpperCase__i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toUpperCase:(I)I")
    }

    // java: toTitleCase(C)C
    pub fn toTitleCase__c(ch: u16) -> Result<u16> {
        panic!("stub: java/lang/Character.toTitleCase:(C)C")
    }

    // java: toTitleCase(I)I
    pub fn toTitleCase__i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toTitleCase:(I)I")
    }

    // java: digit(CI)I
    pub fn digit__c_i(ch: u16, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.digit:(CI)I")
    }

    // java: digit(II)I
    pub fn digit__i_i(codePoint: i32, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.digit:(II)I")
    }

    // java: getNumericValue(C)I
    pub fn getNumericValue__c(ch: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.getNumericValue:(C)I")
    }

    // java: getNumericValue(I)I
    pub fn getNumericValue__i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.getNumericValue:(I)I")
    }

    // java: isSpace(C)Z
    pub fn isSpace(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isSpace:(C)Z")
    }

    // java: isSpaceChar(C)Z
    pub fn isSpaceChar__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isSpaceChar:(C)Z")
    }

    // java: isSpaceChar(I)Z
    pub fn isSpaceChar__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isSpaceChar:(I)Z")
    }

    // java: isWhitespace(C)Z
    pub fn isWhitespace__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isWhitespace:(C)Z")
    }

    // java: isWhitespace(I)Z
    pub fn isWhitespace__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isWhitespace:(I)Z")
    }

    // java: isISOControl(C)Z
    pub fn isISOControl__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isISOControl:(C)Z")
    }

    // java: isISOControl(I)Z
    pub fn isISOControl__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isISOControl:(I)Z")
    }

    // java: getType(C)I
    pub fn getType__c(ch: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.getType:(C)I")
    }

    // java: getType(I)I
    pub fn getType__i(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.getType:(I)I")
    }

    // java: forDigit(II)C
    pub fn forDigit(digit: i32, radix: i32) -> Result<u16> {
        panic!("stub: java/lang/Character.forDigit:(II)C")
    }

    // java: getDirectionality(C)B
    pub fn getDirectionality__c(ch: u16) -> Result<i8> {
        panic!("stub: java/lang/Character.getDirectionality:(C)B")
    }

    // java: getDirectionality(I)B
    pub fn getDirectionality__i(codePoint: i32) -> Result<i8> {
        panic!("stub: java/lang/Character.getDirectionality:(I)B")
    }

    // java: isMirrored(C)Z
    pub fn isMirrored__c(ch: u16) -> Result<bool> {
        panic!("stub: java/lang/Character.isMirrored:(C)Z")
    }

    // java: isMirrored(I)Z
    pub fn isMirrored__i(codePoint: i32) -> Result<bool> {
        panic!("stub: java/lang/Character.isMirrored:(I)Z")
    }

    // java: compareTo(Ljava/lang/Character;)I
    pub fn compareTo(&self, anotherCharacter: Object) -> Result<i32> {
        panic!("stub: java/lang/Character.compareTo:(Ljava/lang/Character;)I")
    }

    // java: compare(CC)I
    pub fn compare(x: u16, y: u16) -> Result<i32> {
        panic!("stub: java/lang/Character.compare:(CC)I")
    }

    // java: toUpperCaseEx(I)I
    pub fn toUpperCaseEx(codePoint: i32) -> Result<i32> {
        panic!("stub: java/lang/Character.toUpperCaseEx:(I)I")
    }

    // java: toUpperCaseCharArray(I)[C
    pub fn toUpperCaseCharArray(codePoint: i32) -> Result<Vec<u16>> {
        panic!("stub: java/lang/Character.toUpperCaseCharArray:(I)[C")
    }

    // java: reverseBytes(C)C
    pub fn reverseBytes(ch: u16) -> Result<u16> {
        panic!("stub: java/lang/Character.reverseBytes:(C)C")
    }

    // java: getName(I)Ljava/lang/String;
    pub fn getName(codePoint: i32) -> Result<String> {
        panic!("stub: java/lang/Character.getName:(I)Ljava/lang/String;")
    }

    // java: codePointOf(Ljava/lang/String;)I
    pub fn codePointOf(name: String) -> Result<i32> {
        panic!("stub: java/lang/Character.codePointOf:(Ljava/lang/String;)I")
    }
}
