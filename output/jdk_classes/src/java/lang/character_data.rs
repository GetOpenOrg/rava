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
        todo!("abstract java/lang/CharacterData.<init>")
    }

    // java: getProperties(I)I
    pub fn getProperties(&self, arg0: i32) -> Result<i32> {
        todo!("abstract java/lang/CharacterData.getProperties")
    }

    // java: getType(I)I
    pub fn getType(&self, arg0: i32) -> Result<i32> {
        todo!("abstract java/lang/CharacterData.getType")
    }

    // java: isDigit(I)Z
    pub fn isDigit(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isDigit")
    }

    // java: isLowerCase(I)Z
    pub fn isLowerCase(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isLowerCase")
    }

    // java: isUpperCase(I)Z
    pub fn isUpperCase(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isUpperCase")
    }

    // java: isWhitespace(I)Z
    pub fn isWhitespace(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isWhitespace")
    }

    // java: isMirrored(I)Z
    pub fn isMirrored(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isMirrored")
    }

    // java: isJavaIdentifierStart(I)Z
    pub fn isJavaIdentifierStart(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isJavaIdentifierStart")
    }

    // java: isJavaIdentifierPart(I)Z
    pub fn isJavaIdentifierPart(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isJavaIdentifierPart")
    }

    // java: isUnicodeIdentifierStart(I)Z
    pub fn isUnicodeIdentifierStart(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isUnicodeIdentifierStart")
    }

    // java: isUnicodeIdentifierPart(I)Z
    pub fn isUnicodeIdentifierPart(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isUnicodeIdentifierPart")
    }

    // java: isIdentifierIgnorable(I)Z
    pub fn isIdentifierIgnorable(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isIdentifierIgnorable")
    }

    // java: isEmoji(I)Z
    pub fn isEmoji(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isEmoji")
    }

    // java: isEmojiPresentation(I)Z
    pub fn isEmojiPresentation(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isEmojiPresentation")
    }

    // java: isEmojiModifier(I)Z
    pub fn isEmojiModifier(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isEmojiModifier")
    }

    // java: isEmojiModifierBase(I)Z
    pub fn isEmojiModifierBase(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isEmojiModifierBase")
    }

    // java: isEmojiComponent(I)Z
    pub fn isEmojiComponent(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isEmojiComponent")
    }

    // java: isExtendedPictographic(I)Z
    pub fn isExtendedPictographic(&self, arg0: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isExtendedPictographic")
    }

    // java: toLowerCase(I)I
    pub fn toLowerCase(&self, arg0: i32) -> Result<i32> {
        todo!("abstract java/lang/CharacterData.toLowerCase")
    }

    // java: toUpperCase(I)I
    pub fn toUpperCase(&self, arg0: i32) -> Result<i32> {
        todo!("abstract java/lang/CharacterData.toUpperCase")
    }

    // java: toTitleCase(I)I
    pub fn toTitleCase(&self, arg0: i32) -> Result<i32> {
        todo!("abstract java/lang/CharacterData.toTitleCase")
    }

    // java: digit(II)I
    pub fn digit(&self, arg0: i32, arg1: i32) -> Result<i32> {
        todo!("abstract java/lang/CharacterData.digit")
    }

    // java: getNumericValue(I)I
    pub fn getNumericValue(&self, arg0: i32) -> Result<i32> {
        todo!("abstract java/lang/CharacterData.getNumericValue")
    }

    // java: getDirectionality(I)B
    pub fn getDirectionality(&self, arg0: i32) -> Result<i8> {
        todo!("abstract java/lang/CharacterData.getDirectionality")
    }

    // java: toUpperCaseEx(I)I
    pub fn toUpperCaseEx(&self, ch: i32) -> Result<i32> {
        todo!("abstract java/lang/CharacterData.toUpperCaseEx")
    }

    // java: toUpperCaseCharArray(I)[C
    pub fn toUpperCaseCharArray(&self, ch: i32) -> Result<Vec<u16>> {
        todo!("abstract java/lang/CharacterData.toUpperCaseCharArray")
    }

    // java: isOtherAlphabetic(I)Z
    pub fn isOtherAlphabetic(&self, ch: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isOtherAlphabetic")
    }

    // java: isIdeographic(I)Z
    pub fn isIdeographic(&self, ch: i32) -> Result<bool> {
        todo!("abstract java/lang/CharacterData.isIdeographic")
    }

    // java: of(I)Ljava/lang/CharacterData;
    pub fn of(ch: i32) -> Result<Object> {
        todo!("abstract java/lang/CharacterData.of")
    }
}
