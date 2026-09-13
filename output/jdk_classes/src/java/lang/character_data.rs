#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/CharacterData",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "abstract",
    source      = "CharacterData.java",
))]
pub struct CharacterData;

impl CharacterData {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/lang/CharacterData.<init>:()V")
    }

    // java: getProperties(I)I
    pub fn getProperties(&self, arg0: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterData.getProperties:(I)I")
    }

    // java: getType(I)I
    pub fn getType(&self, arg0: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterData.getType:(I)I")
    }

    // java: isDigit(I)Z
    pub fn isDigit(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isDigit:(I)Z")
    }

    // java: isLowerCase(I)Z
    pub fn isLowerCase(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isLowerCase:(I)Z")
    }

    // java: isUpperCase(I)Z
    pub fn isUpperCase(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isUpperCase:(I)Z")
    }

    // java: isWhitespace(I)Z
    pub fn isWhitespace(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isWhitespace:(I)Z")
    }

    // java: isMirrored(I)Z
    pub fn isMirrored(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isMirrored:(I)Z")
    }

    // java: isJavaIdentifierStart(I)Z
    pub fn isJavaIdentifierStart(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isJavaIdentifierStart:(I)Z")
    }

    // java: isJavaIdentifierPart(I)Z
    pub fn isJavaIdentifierPart(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isJavaIdentifierPart:(I)Z")
    }

    // java: isUnicodeIdentifierStart(I)Z
    pub fn isUnicodeIdentifierStart(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isUnicodeIdentifierStart:(I)Z")
    }

    // java: isUnicodeIdentifierPart(I)Z
    pub fn isUnicodeIdentifierPart(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isUnicodeIdentifierPart:(I)Z")
    }

    // java: isIdentifierIgnorable(I)Z
    pub fn isIdentifierIgnorable(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isIdentifierIgnorable:(I)Z")
    }

    // java: isEmoji(I)Z
    pub fn isEmoji(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isEmoji:(I)Z")
    }

    // java: isEmojiPresentation(I)Z
    pub fn isEmojiPresentation(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isEmojiPresentation:(I)Z")
    }

    // java: isEmojiModifier(I)Z
    pub fn isEmojiModifier(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isEmojiModifier:(I)Z")
    }

    // java: isEmojiModifierBase(I)Z
    pub fn isEmojiModifierBase(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isEmojiModifierBase:(I)Z")
    }

    // java: isEmojiComponent(I)Z
    pub fn isEmojiComponent(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isEmojiComponent:(I)Z")
    }

    // java: isExtendedPictographic(I)Z
    pub fn isExtendedPictographic(&self, arg0: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isExtendedPictographic:(I)Z")
    }

    // java: toLowerCase(I)I
    pub fn toLowerCase(&self, arg0: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterData.toLowerCase:(I)I")
    }

    // java: toUpperCase(I)I
    pub fn toUpperCase(&self, arg0: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterData.toUpperCase:(I)I")
    }

    // java: toTitleCase(I)I
    pub fn toTitleCase(&self, arg0: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterData.toTitleCase:(I)I")
    }

    // java: digit(II)I
    pub fn digit(&self, arg0: i32, arg1: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterData.digit:(II)I")
    }

    // java: getNumericValue(I)I
    pub fn getNumericValue(&self, arg0: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterData.getNumericValue:(I)I")
    }

    // java: getDirectionality(I)B
    pub fn getDirectionality(&self, arg0: i32) -> Result<i8> {
        panic!("stub: java/lang/CharacterData.getDirectionality:(I)B")
    }

    // java: toUpperCaseEx(I)I
    pub fn toUpperCaseEx(&self, ch: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterData.toUpperCaseEx:(I)I")
    }

    // java: toUpperCaseCharArray(I)[C
    pub fn toUpperCaseCharArray(&self, ch: i32) -> Result<Vec<u16>> {
        panic!("stub: java/lang/CharacterData.toUpperCaseCharArray:(I)[C")
    }

    // java: isOtherAlphabetic(I)Z
    pub fn isOtherAlphabetic(&self, ch: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isOtherAlphabetic:(I)Z")
    }

    // java: isIdeographic(I)Z
    pub fn isIdeographic(&self, ch: i32) -> Result<bool> {
        panic!("stub: java/lang/CharacterData.isIdeographic:(I)Z")
    }

    // java: of(I)Ljava/lang/CharacterData;
    pub fn of(ch: i32) -> Result<Object> {
        panic!("stub: java/lang/CharacterData.of:(I)Ljava/lang/CharacterData;")
    }
}
