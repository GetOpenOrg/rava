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
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = this.value.get();
        let _t1: Object = DynamicConstantDesc::ofNamed(ConstantDescs::BSM_EXPLICIT_CAST(), String::from("_"), ConstantDescs::CD_char(), &_arr0)?;
        let _t2: Object = Optional::of(_t1)?;
        Ok(_t2)
    }

    // java: <init>(C)V
    pub fn new(value: u16) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(value);
        Ok(this)
    }

    // java: valueOf(C)Ljava/lang/Character;
    pub fn valueOf(c: u16) -> Result<Object> {
        return Ok(Character_CharacterCache::cache()[c as usize].clone());
        Ok(Character::new(c)?)
    }

    // java: charValue()C
    pub fn charValue(&self) -> Result<u16> {
        let this = self;
        Ok(this.value.get())
    }

    // java: hashCode()I
    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Character::hashCode__c(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode(C)I
    // java: hashCode(C)I
    pub fn hashCode__c(value: u16) -> Result<i32> {
        Ok(value)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        let _t0 = obj.charValue()?;
        return Ok(this.value.get() == _t0);
        Ok(0i32)
    }

    // java: toString()Ljava/lang/String;
    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(String::from_owned(format!("{}", this.value.get())))
    }

    // java: toString(C)Ljava/lang/String;
    // java: toString(C)Ljava/lang/String;
    pub fn toString__c(c: u16) -> Result<String> {
        Ok(String::from_owned(format!("{}", c)))
    }

    // java: toString(I)Ljava/lang/String;
    // java: toString(I)Ljava/lang/String;
    pub fn toString__i(codePoint: i32) -> Result<String> {
        Ok(String::from_owned(format!("{}", codePoint)))
    }

    // java: isValidCodePoint(I)Z
    pub fn isValidCodePoint(codePoint: i32) -> Result<bool> {
        let mut plane: i32 = ((codePoint as u32>>(16i32&0x1f)) as i32);
        Ok(plane < 17i32)
    }

    // java: isBmpCodePoint(I)Z
    pub fn isBmpCodePoint(codePoint: i32) -> Result<bool> {
        Ok(((codePoint as u32>>(16i32&0x1f)) as i32)==0i32)
    }

    // java: isSupplementaryCodePoint(I)Z
    pub fn isSupplementaryCodePoint(codePoint: i32) -> Result<bool> {
        Ok(codePoint < 1114112i32)
    }

    // java: isHighSurrogate(C)Z
    pub fn isHighSurrogate(ch: u16) -> Result<bool> {
        Ok(ch < 56320i32)
    }

    // java: isLowSurrogate(C)Z
    pub fn isLowSurrogate(ch: u16) -> Result<bool> {
        Ok(ch < 57344i32)
    }

    // java: isSurrogate(C)Z
    pub fn isSurrogate(ch: u16) -> Result<bool> {
        Ok(ch < 57344i32)
    }

    // java: isSurrogatePair(CC)Z
    pub fn isSurrogatePair(high: u16, low: u16) -> Result<bool> {
        let _t0: bool = Character::isHighSurrogate(high)?;
        let _t1: bool = Character::isLowSurrogate(low)?;
        Ok(_t1!=0i32)
    }

    // java: charCount(I)I
    pub fn charCount(codePoint: i32) -> Result<i32> {
        Ok(codePoint < 65536i32)
    }

    // java: toCodePoint(CC)I
    pub fn toCodePoint(high: u16, low: u16) -> Result<i32> {
        Ok((((high<<(10i32&0x1f))).wrapping_add(low)).wrapping_add(-56613888i32))
    }

    // java: codePointAt(Ljava/lang/CharSequence;I)I
    // java: codePointAt(Ljava/lang/CharSequence;I)I
    pub fn codePointAt__seq_i(seq: Object, index: i32) -> Result<i32> {
        let _t0 = seq.charAt(index)?;
        let mut c1: i32 = _t0;
        let _t1: bool = Character::isHighSurrogate(c1)?;
        index = index.wrapping_add(1i32);
        let _t2 = seq.length()?;
        let _t3 = seq.charAt(index)?;
        let mut c2: i32 = _t3;
        let _t4: bool = Character::isLowSurrogate(c2)?;
        let _t5: i32 = Character::toCodePoint(c1, c2)?;
        return Ok(_t5);
        Ok(c1)
    }

    // java: codePointAt([CI)I
    // java: codePointAt([CI)I
    pub fn codePointAt__arr_c_i(a: &[u16], index: i32) -> Result<i32> {
        let _t0: i32 = Character::codePointAtImpl(&a, index, (a.len() as i32))?;
        Ok(_t0)
    }

    // java: codePointAt([CII)I
    // java: codePointAt([CII)I
    pub fn codePointAt__arr_c_i_i(a: &[u16], index: i32, limit: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i32 = Character::codePointAtImpl(&a, index, limit)?;
        Ok(_t0)
    }

    // java: codePointAtImpl([CII)I
    pub fn codePointAtImpl(a: &[u16], index: i32, limit: i32) -> Result<i32> {
        let mut c1: i32 = a[index as usize];
        let _t0: bool = Character::isHighSurrogate(c1)?;
        index = index.wrapping_add(1i32);
        let mut c2: i32 = a[index as usize];
        let _t1: bool = Character::isLowSurrogate(c2)?;
        let _t2: i32 = Character::toCodePoint(c1, c2)?;
        return Ok(_t2);
        Ok(c1)
    }

    // java: codePointBefore(Ljava/lang/CharSequence;I)I
    // java: codePointBefore(Ljava/lang/CharSequence;I)I
    pub fn codePointBefore__seq_i(seq: Object, index: i32) -> Result<i32> {
        index = index.wrapping_sub(1i32);
        let _t0 = seq.charAt(index)?;
        let mut c2: i32 = _t0;
        let _t1: bool = Character::isLowSurrogate(c2)?;
        index = index.wrapping_sub(1i32);
        let _t2 = seq.charAt(index)?;
        let mut c1: i32 = _t2;
        let _t3: bool = Character::isHighSurrogate(c1)?;
        let _t4: i32 = Character::toCodePoint(c1, c2)?;
        return Ok(_t4);
        Ok(c2)
    }

    // java: codePointBefore([CI)I
    // java: codePointBefore([CI)I
    pub fn codePointBefore__arr_c_i(a: &[u16], index: i32) -> Result<i32> {
        let _t0: i32 = Character::codePointBeforeImpl(&a, index, 0i32)?;
        Ok(_t0)
    }

    // java: codePointBefore([CII)I
    // java: codePointBefore([CII)I
    pub fn codePointBefore__arr_c_i_i(a: &[u16], index: i32, start: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i32 = Character::codePointBeforeImpl(&a, index, start)?;
        Ok(_t0)
    }

    // java: codePointBeforeImpl([CII)I
    pub fn codePointBeforeImpl(a: &[u16], index: i32, start: i32) -> Result<i32> {
        index = index.wrapping_sub(1i32);
        let mut c2: i32 = a[index as usize];
        let _t0: bool = Character::isLowSurrogate(c2)?;
        index = index.wrapping_sub(1i32);
        let mut c1: i32 = a[index as usize];
        let _t1: bool = Character::isHighSurrogate(c1)?;
        let _t2: i32 = Character::toCodePoint(c1, c2)?;
        return Ok(_t2);
        Ok(c2)
    }

    // java: highSurrogate(I)C
    pub fn highSurrogate(codePoint: i32) -> Result<u16> {
        /* TODO: i2c  */
        Ok((((codePoint as u32>>(10i32&0x1f)) as i32)).wrapping_add(55232i32))
    }

    // java: lowSurrogate(I)C
    pub fn lowSurrogate(codePoint: i32) -> Result<u16> {
        /* TODO: i2c  */
        Ok(((codePoint&1023i32)).wrapping_add(56320i32))
    }

    // java: toChars(I[CI)I
    // java: toChars(I[CI)I
    pub fn toChars__i_arr_c_i(codePoint: i32, dst: &[u16], dstIndex: i32) -> Result<i32> {
        let _t0: bool = Character::isBmpCodePoint(codePoint)?;
        /* TODO: i2c  */
        dst[dstIndex as usize] = codePoint;
        return Ok(1i32);
        let _t1: bool = Character::isValidCodePoint(codePoint)?;
        Character::toSurrogates(codePoint, &dst, dstIndex)?;
        return Ok(2i32);
        let mut _arr2: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr2[0i32 as usize] = codePoint;
        let _t3: String = String::format(String::from("Not a valid Unicode code point: 0x%X"), &_arr2)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toChars(I)[C
    // java: toChars(I)[C
    pub fn toChars__i(codePoint: i32) -> Result<Vec<u16>> {
        let _t0: bool = Character::isBmpCodePoint(codePoint)?;
        let mut _arr1: Vec<u16> = vec![0u16; 1i32 as usize];
        /* TODO: i2c  */
        _arr1[0i32 as usize] = codePoint;
        return Ok(_arr1);
        let _t2: bool = Character::isValidCodePoint(codePoint)?;
        let mut _arr3: Vec<u16> = vec![0u16; 2i32 as usize];
        let mut result: Vec<u16> = _arr3;
        Character::toSurrogates(codePoint, &result, 0i32)?;
        return Ok(result);
        let mut _arr4: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr4[0i32 as usize] = codePoint;
        let _t5: String = String::format(String::from("Not a valid Unicode code point: 0x%X"), &_arr4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toSurrogates(I[CI)V
    pub fn toSurrogates(codePoint: i32, dst: &[u16], index: i32) -> Result<()> {
        let _t0: u16 = Character::lowSurrogate(codePoint)?;
        dst[(index).wrapping_add(1i32) as usize] = _t0;
        let _t1: u16 = Character::highSurrogate(codePoint)?;
        dst[index as usize] = _t1;
        Ok(())
    }

    // java: codePointCount(Ljava/lang/CharSequence;II)I
    // java: codePointCount(Ljava/lang/CharSequence;II)I
    pub fn codePointCount__seq_i_i(seq: Object, beginIndex: i32, endIndex: i32) -> Result<i32> {
        let _t0 = seq.length()?;
        let _t1: i32 = Objects::checkFromToIndex__i_i_i(beginIndex, endIndex, _t0)?;
        let mut n: i32 = (endIndex).wrapping_sub(beginIndex);
        let mut i: i32 = beginIndex;
        loop {
            if i >= endIndex { break; }
            i = i.wrapping_add(1i32);
            let _t0 = seq.charAt(i)?;
            let _t1: bool = Character::isHighSurrogate(_t0)?;
            let _t2 = seq.charAt(i)?;
            let _t3: bool = Character::isLowSurrogate(_t2)?;
            n = n.wrapping_sub(1i32);
            i = i.wrapping_add(1i32);
        }
        Ok(n)
    }

    // java: codePointCount([CII)I
    // java: codePointCount([CII)I
    pub fn codePointCount__arr_c_i_i(a: &[u16], offset: i32, count: i32) -> Result<i32> {
        let _t0: i32 = Objects::checkFromIndexSize__i_i_i(offset, count, (a.len() as i32))?;
        let _t1: i32 = Character::codePointCountImpl(&a, offset, count)?;
        Ok(_t1)
    }

    // java: codePointCountImpl([CII)I
    pub fn codePointCountImpl(a: &[u16], offset: i32, count: i32) -> Result<i32> {
        let mut endIndex: i32 = (offset).wrapping_add(count);
        let mut n: i32 = count;
        let mut i: i32 = offset;
        loop {
            if i >= endIndex { break; }
            i = i.wrapping_add(1i32);
            let _t0: bool = Character::isHighSurrogate(a[i as usize])?;
            let _t1: bool = Character::isLowSurrogate(a[i as usize])?;
            n = n.wrapping_sub(1i32);
            i = i.wrapping_add(1i32);
        }
        Ok(n)
    }

    // java: offsetByCodePoints(Ljava/lang/CharSequence;II)I
    // java: offsetByCodePoints(Ljava/lang/CharSequence;II)I
    pub fn offsetByCodePoints__seq_i_i(seq: Object, index: i32, codePointOffset: i32) -> Result<i32> {
        let _t0 = seq.length()?;
        let mut length: i32 = _t0;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut x: i32 = index;
        let mut i: i32 = 0i32;
        loop {
            if x >= length { break; }
            x = x.wrapping_add(1i32);
            let _t0 = seq.charAt(x)?;
            let _t1: bool = Character::isHighSurrogate(_t0)?;
            let _t2 = seq.charAt(x)?;
            let _t3: bool = Character::isLowSurrogate(_t2)?;
            x = x.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        i = codePointOffset;
        loop {
            if x<=0i32 { break; }
            x = x.wrapping_sub(1i32);
            let _t0 = seq.charAt(x)?;
            let _t1: bool = Character::isLowSurrogate(_t0)?;
            let _t2 = seq.charAt((x).wrapping_sub(1i32))?;
            let _t3: bool = Character::isHighSurrogate(_t2)?;
            x = x.wrapping_sub(1i32);
            i = i.wrapping_add(1i32);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(x)
    }

    // java: offsetByCodePoints([CIIII)I
    // java: offsetByCodePoints([CIIII)I
    pub fn offsetByCodePoints__arr_c_i_i_i_i(a: &[u16], start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i32 = Character::offsetByCodePointsImpl(&a, start, count, index, codePointOffset)?;
        Ok(_t0)
    }

    // java: offsetByCodePointsImpl([CIIII)I
    pub fn offsetByCodePointsImpl(a: &[u16], start: i32, count: i32, index: i32, codePointOffset: i32) -> Result<i32> {
        let mut x: i32 = index;
        let mut limit: i32 = (start).wrapping_add(count);
        let mut i: i32 = 0i32;
        loop {
            if x >= limit { break; }
            x = x.wrapping_add(1i32);
            let _t0: bool = Character::isHighSurrogate(a[x as usize])?;
            let _t1: bool = Character::isLowSurrogate(a[x as usize])?;
            x = x.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        limit = codePointOffset;
        loop {
            if x <= start { break; }
            x = x.wrapping_sub(1i32);
            let _t0: bool = Character::isLowSurrogate(a[x as usize])?;
            let _t1: bool = Character::isHighSurrogate(a[(x).wrapping_sub(1i32) as usize])?;
            x = x.wrapping_sub(1i32);
            limit = limit.wrapping_add(1i32);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(x)
    }

    // java: isLowerCase(C)Z
    // java: isLowerCase(C)Z
    pub fn isLowerCase__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isLowerCase__i(ch)?;
        Ok(_t0)
    }

    // java: isLowerCase(I)Z
    // java: isLowerCase(I)Z
    pub fn isLowerCase__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isLowerCase(codePoint)?;
        Ok(_t1)
    }

    // java: isUpperCase(C)Z
    // java: isUpperCase(C)Z
    pub fn isUpperCase__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isUpperCase__i(ch)?;
        Ok(_t0)
    }

    // java: isUpperCase(I)Z
    // java: isUpperCase(I)Z
    pub fn isUpperCase__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isUpperCase(codePoint)?;
        Ok(_t1)
    }

    // java: isTitleCase(C)Z
    // java: isTitleCase(C)Z
    pub fn isTitleCase__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isTitleCase__i(ch)?;
        Ok(_t0)
    }

    // java: isTitleCase(I)Z
    // java: isTitleCase(I)Z
    pub fn isTitleCase__i(codePoint: i32) -> Result<bool> {
        let _t0: i32 = Character::getType__i(codePoint)?;
        Ok(_t0 == 3i32)
    }

    // java: isDigit(C)Z
    // java: isDigit(C)Z
    pub fn isDigit__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isDigit__i(ch)?;
        Ok(_t0)
    }

    // java: isDigit(I)Z
    // java: isDigit(I)Z
    pub fn isDigit__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isDigit(codePoint)?;
        Ok(_t1)
    }

    // java: isDefined(C)Z
    // java: isDefined(C)Z
    pub fn isDefined__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isDefined__i(ch)?;
        Ok(_t0)
    }

    // java: isDefined(I)Z
    // java: isDefined(I)Z
    pub fn isDefined__i(codePoint: i32) -> Result<bool> {
        let _t0: i32 = Character::getType__i(codePoint)?;
        Ok(_t0!=0i32)
    }

    // java: isLetter(C)Z
    // java: isLetter(C)Z
    pub fn isLetter__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isLetter__i(ch)?;
        Ok(_t0)
    }

    // java: isLetter(I)Z
    // java: isLetter(I)Z
    pub fn isLetter__i(codePoint: i32) -> Result<bool> {
        let _t0: i32 = Character::getType__i(codePoint)?;
        Ok(((62i32>>((_t0&0x1f)))&1i32)!=0i32)
    }

    // java: isLetterOrDigit(C)Z
    // java: isLetterOrDigit(C)Z
    pub fn isLetterOrDigit__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isLetterOrDigit__i(ch)?;
        Ok(_t0)
    }

    // java: isLetterOrDigit(I)Z
    // java: isLetterOrDigit(I)Z
    pub fn isLetterOrDigit__i(codePoint: i32) -> Result<bool> {
        let _t0: i32 = Character::getType__i(codePoint)?;
        Ok(((574i32>>((_t0&0x1f)))&1i32)!=0i32)
    }

    // java: isJavaLetter(C)Z
    pub fn isJavaLetter(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isJavaIdentifierStart__c(ch)?;
        Ok(_t0)
    }

    // java: isJavaLetterOrDigit(C)Z
    pub fn isJavaLetterOrDigit(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isJavaIdentifierPart__c(ch)?;
        Ok(_t0)
    }

    // java: isAlphabetic(I)Z
    pub fn isAlphabetic(codePoint: i32) -> Result<bool> {
        let _t0: i32 = Character::getType__i(codePoint)?;
        let _t1: Object = CharacterData::of(codePoint)?;
        let _t2 = _t1.isOtherAlphabetic(codePoint)?;
        Ok(_t2!=0i32)
    }

    // java: isIdeographic(I)Z
    pub fn isIdeographic(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isIdeographic(codePoint)?;
        Ok(_t1)
    }

    // java: isJavaIdentifierStart(C)Z
    // java: isJavaIdentifierStart(C)Z
    pub fn isJavaIdentifierStart__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isJavaIdentifierStart__i(ch)?;
        Ok(_t0)
    }

    // java: isJavaIdentifierStart(I)Z
    // java: isJavaIdentifierStart(I)Z
    pub fn isJavaIdentifierStart__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isJavaIdentifierStart(codePoint)?;
        Ok(_t1)
    }

    // java: isJavaIdentifierPart(C)Z
    // java: isJavaIdentifierPart(C)Z
    pub fn isJavaIdentifierPart__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isJavaIdentifierPart__i(ch)?;
        Ok(_t0)
    }

    // java: isJavaIdentifierPart(I)Z
    // java: isJavaIdentifierPart(I)Z
    pub fn isJavaIdentifierPart__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isJavaIdentifierPart(codePoint)?;
        Ok(_t1)
    }

    // java: isUnicodeIdentifierStart(C)Z
    // java: isUnicodeIdentifierStart(C)Z
    pub fn isUnicodeIdentifierStart__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isUnicodeIdentifierStart__i(ch)?;
        Ok(_t0)
    }

    // java: isUnicodeIdentifierStart(I)Z
    // java: isUnicodeIdentifierStart(I)Z
    pub fn isUnicodeIdentifierStart__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isUnicodeIdentifierStart(codePoint)?;
        Ok(_t1)
    }

    // java: isUnicodeIdentifierPart(C)Z
    // java: isUnicodeIdentifierPart(C)Z
    pub fn isUnicodeIdentifierPart__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isUnicodeIdentifierPart__i(ch)?;
        Ok(_t0)
    }

    // java: isUnicodeIdentifierPart(I)Z
    // java: isUnicodeIdentifierPart(I)Z
    pub fn isUnicodeIdentifierPart__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isUnicodeIdentifierPart(codePoint)?;
        Ok(_t1)
    }

    // java: isIdentifierIgnorable(C)Z
    // java: isIdentifierIgnorable(C)Z
    pub fn isIdentifierIgnorable__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isIdentifierIgnorable__i(ch)?;
        Ok(_t0)
    }

    // java: isIdentifierIgnorable(I)Z
    // java: isIdentifierIgnorable(I)Z
    pub fn isIdentifierIgnorable__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isIdentifierIgnorable(codePoint)?;
        Ok(_t1)
    }

    // java: isEmoji(I)Z
    pub fn isEmoji(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isEmoji(codePoint)?;
        Ok(_t1)
    }

    // java: isEmojiPresentation(I)Z
    pub fn isEmojiPresentation(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isEmojiPresentation(codePoint)?;
        Ok(_t1)
    }

    // java: isEmojiModifier(I)Z
    pub fn isEmojiModifier(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isEmojiModifier(codePoint)?;
        Ok(_t1)
    }

    // java: isEmojiModifierBase(I)Z
    pub fn isEmojiModifierBase(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isEmojiModifierBase(codePoint)?;
        Ok(_t1)
    }

    // java: isEmojiComponent(I)Z
    pub fn isEmojiComponent(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isEmojiComponent(codePoint)?;
        Ok(_t1)
    }

    // java: isExtendedPictographic(I)Z
    pub fn isExtendedPictographic(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isExtendedPictographic(codePoint)?;
        Ok(_t1)
    }

    // java: toLowerCase(C)C
    // java: toLowerCase(C)C
    pub fn toLowerCase__c(ch: u16) -> Result<u16> {
        let _t0: i32 = Character::toLowerCase__i(ch)?;
        /* TODO: i2c  */
        Ok(_t0)
    }

    // java: toLowerCase(I)I
    // java: toLowerCase(I)I
    pub fn toLowerCase__i(codePoint: i32) -> Result<i32> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.toLowerCase(codePoint)?;
        Ok(_t1)
    }

    // java: toUpperCase(C)C
    // java: toUpperCase(C)C
    pub fn toUpperCase__c(ch: u16) -> Result<u16> {
        let _t0: i32 = Character::toUpperCase__i(ch)?;
        /* TODO: i2c  */
        Ok(_t0)
    }

    // java: toUpperCase(I)I
    // java: toUpperCase(I)I
    pub fn toUpperCase__i(codePoint: i32) -> Result<i32> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.toUpperCase(codePoint)?;
        Ok(_t1)
    }

    // java: toTitleCase(C)C
    // java: toTitleCase(C)C
    pub fn toTitleCase__c(ch: u16) -> Result<u16> {
        let _t0: i32 = Character::toTitleCase__i(ch)?;
        /* TODO: i2c  */
        Ok(_t0)
    }

    // java: toTitleCase(I)I
    // java: toTitleCase(I)I
    pub fn toTitleCase__i(codePoint: i32) -> Result<i32> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.toTitleCase(codePoint)?;
        Ok(_t1)
    }

    // java: digit(CI)I
    // java: digit(CI)I
    pub fn digit__c_i(ch: u16, radix: i32) -> Result<i32> {
        let _t0: i32 = Character::digit__i_i(ch, radix)?;
        Ok(_t0)
    }

    // java: digit(II)I
    // java: digit(II)I
    pub fn digit__i_i(codePoint: i32, radix: i32) -> Result<i32> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.digit(codePoint, radix)?;
        Ok(_t1)
    }

    // java: getNumericValue(C)I
    // java: getNumericValue(C)I
    pub fn getNumericValue__c(ch: u16) -> Result<i32> {
        let _t0: i32 = Character::getNumericValue__i(ch)?;
        Ok(_t0)
    }

    // java: getNumericValue(I)I
    // java: getNumericValue(I)I
    pub fn getNumericValue__i(codePoint: i32) -> Result<i32> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.getNumericValue(codePoint)?;
        Ok(_t1)
    }

    // java: isSpace(C)Z
    pub fn isSpace(ch: u16) -> Result<bool> {
        /* TODO: lshr  */
        /* TODO: land  */
        /* TODO: lcmp  */
        Ok(0i64!=0i32)
    }

    // java: isSpaceChar(C)Z
    // java: isSpaceChar(C)Z
    pub fn isSpaceChar__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isSpaceChar__i(ch)?;
        Ok(_t0)
    }

    // java: isSpaceChar(I)Z
    // java: isSpaceChar(I)Z
    pub fn isSpaceChar__i(codePoint: i32) -> Result<bool> {
        let _t0: i32 = Character::getType__i(codePoint)?;
        Ok(((28672i32>>((_t0&0x1f)))&1i32)!=0i32)
    }

    // java: isWhitespace(C)Z
    // java: isWhitespace(C)Z
    pub fn isWhitespace__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isWhitespace__i(ch)?;
        Ok(_t0)
    }

    // java: isWhitespace(I)Z
    // java: isWhitespace(I)Z
    pub fn isWhitespace__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isWhitespace(codePoint)?;
        Ok(_t1)
    }

    // java: isISOControl(C)Z
    // java: isISOControl(C)Z
    pub fn isISOControl__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isISOControl__i(ch)?;
        Ok(_t0)
    }

    // java: isISOControl(I)Z
    // java: isISOControl(I)Z
    pub fn isISOControl__i(codePoint: i32) -> Result<bool> {
        Ok(((codePoint as u32>>(5i32&0x1f)) as i32)==0i32)
    }

    // java: getType(C)I
    // java: getType(C)I
    pub fn getType__c(ch: u16) -> Result<i32> {
        let _t0: i32 = Character::getType__i(ch)?;
        Ok(_t0)
    }

    // java: getType(I)I
    // java: getType(I)I
    pub fn getType__i(codePoint: i32) -> Result<i32> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.getType(codePoint)?;
        Ok(_t1)
    }

    // java: forDigit(II)C
    pub fn forDigit(digit: i32, radix: i32) -> Result<u16> {
        return Ok(0i32);
        return Ok(0i32);
        /* TODO: i2c  */
        return Ok((48i32).wrapping_add(digit));
        /* TODO: i2c  */
        Ok((87i32).wrapping_add(digit))
    }

    // java: getDirectionality(C)B
    // java: getDirectionality(C)B
    pub fn getDirectionality__c(ch: u16) -> Result<i8> {
        let _t0: i8 = Character::getDirectionality__i(ch)?;
        Ok(_t0)
    }

    // java: getDirectionality(I)B
    // java: getDirectionality(I)B
    pub fn getDirectionality__i(codePoint: i32) -> Result<i8> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.getDirectionality(codePoint)?;
        Ok(_t1)
    }

    // java: isMirrored(C)Z
    // java: isMirrored(C)Z
    pub fn isMirrored__c(ch: u16) -> Result<bool> {
        let _t0: bool = Character::isMirrored__i(ch)?;
        Ok(_t0)
    }

    // java: isMirrored(I)Z
    // java: isMirrored(I)Z
    pub fn isMirrored__i(codePoint: i32) -> Result<bool> {
        let _t0: Object = CharacterData::of(codePoint)?;
        let _t1 = _t0.isMirrored(codePoint)?;
        Ok(_t1)
    }

    // java: compareTo(Ljava/lang/Character;)I
    pub fn compareTo(&self, anotherCharacter: Object) -> Result<i32> {
        let this = self;
        let _t0: i32 = Character::compare(this.value.get(), anotherCharacter.value.get())?;
        Ok(_t0)
    }

    // java: compare(CC)I
    pub fn compare(x: u16, y: u16) -> Result<i32> {
        Ok((x).wrapping_sub(y))
    }

    // java: toUpperCaseEx(I)I
    pub fn toUpperCaseEx(codePoint: i32) -> Result<i32> {
        let _t0: bool = Character::isValidCodePoint(codePoint)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1: Object = CharacterData::of(codePoint)?;
        let _t2 = _t1.toUpperCaseEx(codePoint)?;
        Ok(_t2)
    }

    // java: toUpperCaseCharArray(I)[C
    pub fn toUpperCaseCharArray(codePoint: i32) -> Result<Vec<u16>> {
        let _t0: bool = Character::isBmpCodePoint(codePoint)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1: Object = CharacterData::of(codePoint)?;
        let _t2 = _t1.toUpperCaseCharArray(codePoint)?;
        Ok(_t2)
    }

    // java: reverseBytes(C)C
    pub fn reverseBytes(ch: u16) -> Result<u16> {
        /* TODO: i2c  */
        Ok((((ch&296i32)>>((8i32&0x1f)))|(ch<<(8i32&0x1f))))
    }

    // java: getName(I)Ljava/lang/String;
    pub fn getName(codePoint: i32) -> Result<String> {
        let _t0: bool = Character::isValidCodePoint(codePoint)?;
        let mut _arr1: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr1[0i32 as usize] = codePoint;
        let _t2: String = String::format(String::from("Not a valid Unicode code point: 0x%X"), &_arr1)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t3: Object = CharacterName::getInstance()?;
        let _t4 = _t3.getName(codePoint)?;
        let mut name: String = _t4;
        return Ok(name);
        let _t5: i32 = Character::getType__i(codePoint)?;
        /* TODO: aconst_null  */
        return Ok(_t5);
        let _t6: Object = Character_UnicodeBlock::of(codePoint)?;
        let mut block: Object = _t6;
        let _t7 = block.toString()?;
        let _t8 = _t7.replace(95i32, 32i32)?;
        String::new().append(&_t8)?;
        String::new().append(&String::from(""))?;
        let _t9: String = Integer::toHexString(codePoint)?;
        let _t10 = _t9.toUpperCase(Locale::ROOT())?;
        String::new().append(&_t10)?;
        return Ok(String::new());
        let _t11: String = Integer::toHexString(codePoint)?;
        let _t12 = _t11.toUpperCase(Locale::ROOT())?;
        Ok(_t12)
    }

    // java: codePointOf(Ljava/lang/String;)I
    pub fn codePointOf(name: String) -> Result<i32> {
        let _t0 = name.trim()?;
        let _t1 = _t0.toUpperCase(Locale::ROOT())?;
        name = _t1;
        let _t2: Object = CharacterName::getInstance()?;
        let _t3 = _t2.getCodePoint(name)?;
        let mut cp: i32 = _t3;
        return Ok(cp);
        let _t4 = name.lastIndexOf(32i32)?;
        let mut off: i32 = _t4;
        let _t5 = name.length()?;
        let _t6: i32 = Integer::parseInt__seq_i_i_i(name, (off).wrapping_add(1i32), _t5, 16i32)?;
        cp = _t6;
        let _t7: bool = Character::isValidCodePoint(cp)?;
        let _t8: String = Character::getName(cp)?;
        let _t9 = name.equals(_t8)?;
        return Ok(cp);
        off = _t9;
        String::new().append(&String::from("Unrecognized character name :"))?;
        String::new().append(&name)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
