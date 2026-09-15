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
    binary_name       = "java/lang/Long",
    super_class       = "java/lang/Number",
    interfaces        = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access            = "public",
    modifiers         = "final",
    generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/lang/Long;>;Ljava/lang/constant/Constable;Ljava/lang/constant/ConstantDesc;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Long.java",
    inner_classes     = "java/lang/Long$LongCache:java/lang/Long:LongCache:26;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Long;java/lang/Number;java/lang/Object;java/lang/constant/Constable;java/lang/constant/ConstantDesc",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Long {
    pub _super: Number,
    #[cfg_attr(any(), java_field(name = "value", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
    pub value: JField<i64>,
}

impl Long {
    pub fn as_number(&self) -> &Number { &self._super }
    pub fn into_number(self) -> Number { self._super }
}

impl From<Long> for Number {
    fn from(v: Long) -> Number { v._super }
}

impl Long {
    #[cfg_attr(any(), java_field(name = "MIN_VALUE", descriptor = "J", access = "public", modifiers = "static final", is_static = true, constant_value = "-9223372036854775808"))]
    // static field: MIN_VALUE:J
    pub fn MIN_VALUE() -> i64 {
        -9223372036854775808i64
    }

    #[cfg_attr(any(), java_field(name = "MAX_VALUE", descriptor = "J", access = "public", modifiers = "static final", is_static = true, constant_value = "9223372036854775807"))]
    // static field: MAX_VALUE:J
    pub fn MAX_VALUE() -> i64 {
        9223372036854775807i64
    }

    #[cfg_attr(any(), java_field(name = "TYPE", descriptor = "Ljava/lang/Class;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/Class<Ljava/lang/Long;>;"))]
    // static field: TYPE:Ljava/lang/Class;
    pub fn TYPE() -> Object {
        panic!("stub: java/lang/Long.TYPE:Ljava/lang/Class;")
    }

    #[cfg_attr(any(), java_field(name = "SIZE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "64"))]
    // static field: SIZE:I
    pub fn SIZE() -> i32 {
        64
    }

    #[cfg_attr(any(), java_field(name = "BYTES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
    // static field: BYTES:I
    pub fn BYTES() -> i32 {
        8
    }

    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4290774380558885855"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        4290774380558885855i64
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(JI)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString_l_i(i: i64, arg1: i32) -> Result<String> {
        panic!("stub: java/lang/Long.toString:(JI)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toStringUTF16", descriptor = "(JI)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toStringUTF16(i: i64, arg1: i32) -> Result<String> {
        panic!("stub: java/lang/Long.toStringUTF16:(JI)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString", descriptor = "(JI)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUnsignedString_l_i(i: i64, arg1: i32) -> Result<String> {
        panic!("stub: java/lang/Long.toUnsignedString:(JI)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedBigInteger", descriptor = "(J)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUnsignedBigInteger(i: i64) -> Result<Object> {
        panic!("stub: java/lang/Long.toUnsignedBigInteger:(J)Ljava/math/BigInteger;")
    }

    #[cfg_attr(any(), java_method(name = "toHexString", descriptor = "(J)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toHexString(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toHexString:(J)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toOctalString", descriptor = "(J)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toOctalString(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toOctalString:(J)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toBinaryString", descriptor = "(J)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toBinaryString(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toBinaryString:(J)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString0", descriptor = "(JI)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUnsignedString0(val: i64, arg1: i32) -> Result<String> {
        panic!("stub: java/lang/Long.toUnsignedString0:(JI)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "formatUnsignedLong0", descriptor = "(JI[BII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn formatUnsignedLong0(val: i64, arg1: i32, shift: Rc<RefCell<Vec<i8>>>, buf: i32, offset: i32) -> Result<()> {
        panic!("stub: java/lang/Long.formatUnsignedLong0:(JI[BII)V")
    }

    #[cfg_attr(any(), java_method(name = "formatUnsignedLong0UTF16", descriptor = "(JI[BII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn formatUnsignedLong0UTF16(val: i64, arg1: i32, shift: Rc<RefCell<Vec<i8>>>, buf: i32, offset: i32) -> Result<()> {
        panic!("stub: java/lang/Long.formatUnsignedLong0UTF16:(JI[BII)V")
    }

    #[cfg_attr(any(), java_method(name = "fastUUID", descriptor = "(JJ)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn fastUUID(lsb: i64, arg1: i64) -> Result<String> {
        panic!("stub: java/lang/Long.fastUUID:(JJ)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(J)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: toString(J)Ljava/lang/String;
    pub fn toString_l(mut i: i64) -> Result<String> {
        let _t0: i32 = Long::stringSize(i)?;
        let mut size: i32 = _t0;
        if String::COMPACT_STRINGS() {
            let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; size as usize]));
            let mut buf: Rc<RefCell<Vec<i8>>> = _arr1;
            let _t2: i32 = Long::getChars(i, size, Clone::clone(&buf))?;
            return Ok(String::new_arr_b_b(Clone::clone(&buf), ((0i32) as i8))?);
        }
        let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (size).wrapping_mul(2i32) as usize]));
        let mut buf: Rc<RefCell<Vec<i8>>> = _arr1;
        let _t2: i32 = StringUTF16::getChars_l_i_arr_b(i, size, Clone::clone(&buf))?;
        Ok(String::new_arr_b_b(Clone::clone(&buf), ((1i32) as i8))?)
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString", descriptor = "(J)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUnsignedString_l(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toUnsignedString:(J)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(JI[B)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getChars(mut i: i64, mut index: i32, mut buf: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        let mut charPos: i32 = index;
        let mut negative = ((((i>(0i64)) as i32-((i)<(0i64)) as i32)<0)) as i32;
        if (negative==0) {
            i = (i).wrapping_neg();
        }
        let mut r: i32 = Default::default();
        loop {
            if (((i>(-2147483648i64)) as i32-((i)<(-2147483648i64)) as i32)>0) { break; }
            let mut q = (i/100i64);
            r = (((q).wrapping_mul(100i64)).wrapping_sub(i) as i32);
            i = q;
            charPos = charPos.wrapping_sub(1i32);
            buf.borrow_mut()[charPos as usize] = ((Integer::DigitOnes().borrow()[r as usize] as i32)) as i8;
            charPos = charPos.wrapping_sub(1i32);
            buf.borrow_mut()[charPos as usize] = ((Integer::DigitTens().borrow()[r as usize] as i32)) as i8;
        }
        let mut i2: i32 = (i as i32);
        loop {
            if i2 > -100i32 { break; }
            let mut q2 = (i2/100i32);
            r = ((q2).wrapping_mul(100i32)).wrapping_sub(i2);
            i2 = q2;
            charPos = charPos.wrapping_sub(1i32);
            buf.borrow_mut()[charPos as usize] = ((Integer::DigitOnes().borrow()[r as usize] as i32)) as i8;
            charPos = charPos.wrapping_sub(1i32);
            buf.borrow_mut()[charPos as usize] = ((Integer::DigitTens().borrow()[r as usize] as i32)) as i8;
        }
        charPos = charPos.wrapping_sub(1i32);
        buf.borrow_mut()[charPos as usize] = ((Integer::DigitOnes().borrow()[(i2).wrapping_neg() as usize] as i32)) as i8;
        if i2 < -9i32 {
            charPos = charPos.wrapping_sub(1i32);
            buf.borrow_mut()[charPos as usize] = ((Integer::DigitTens().borrow()[(i2).wrapping_neg() as usize] as i32)) as i8;
        }
        if (negative!=0) {
            charPos = charPos.wrapping_sub(1i32);
            buf.borrow_mut()[charPos as usize] = (45i32) as i8;
        }
        Ok(charPos)
    }

    #[cfg_attr(any(), java_method(name = "stringSize", descriptor = "(J)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn stringSize(mut x: i64) -> Result<i32> {
        let mut d: i32 = 1i32;
        if (((x>(0i64)) as i32-((x)<(0i64)) as i32)>=0) {
            d = 0i32;
            x = (x).wrapping_neg();
        }
        let mut p: i64 = -10i64;
        let mut i: i32 = 1i32;
        loop {
            if i >= 19i32 { break; }
            if (((x>(p)) as i32-((x)<(p)) as i32)>0) {
                return Ok((i).wrapping_add(d));
            }
            p = (10i64).wrapping_mul(p);
            i = i.wrapping_add(1i32);
        }
        Ok((19i32).wrapping_add(d))
    }

    #[cfg_attr(any(), java_method(name = "parseLong", descriptor = "(Ljava/lang/String;I)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseLong_str_i(s: String, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.parseLong:(Ljava/lang/String;I)J")
    }

    #[cfg_attr(any(), java_method(name = "parseLong", descriptor = "(Ljava/lang/CharSequence;III)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseLong_seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.parseLong:(Ljava/lang/CharSequence;III)J")
    }

    #[cfg_attr(any(), java_method(name = "parseLong", descriptor = "(Ljava/lang/String;)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseLong_str(s: String) -> Result<i64> {
        panic!("stub: java/lang/Long.parseLong:(Ljava/lang/String;)J")
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedLong", descriptor = "(Ljava/lang/String;I)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseUnsignedLong_str_i(s: String, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.parseUnsignedLong:(Ljava/lang/String;I)J")
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedLong", descriptor = "(Ljava/lang/CharSequence;III)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseUnsignedLong_seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.parseUnsignedLong:(Ljava/lang/CharSequence;III)J")
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedLong", descriptor = "(Ljava/lang/String;)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseUnsignedLong_str(s: String) -> Result<i64> {
        panic!("stub: java/lang/Long.parseUnsignedLong:(Ljava/lang/String;)J")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/String;I)Ljava/lang/Long;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn valueOf_str_i(s: String, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.valueOf:(Ljava/lang/String;I)Ljava/lang/Long;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Long;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn valueOf_str(s: String) -> Result<i64> {
        panic!("stub: java/lang/Long.valueOf:(Ljava/lang/String;)Ljava/lang/Long;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(J)Ljava/lang/Long;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_l(l: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.valueOf:(J)Ljava/lang/Long;")
    }

    #[cfg_attr(any(), java_method(name = "decode", descriptor = "(Ljava/lang/String;)Ljava/lang/Long;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn decode(nm: String) -> Result<i64> {
        panic!("stub: java/lang/Long.decode:(Ljava/lang/String;)Ljava/lang/Long;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn new_l(value: i64) -> Result<Self> {
        panic!("stub: java/lang/Long.<init>:(J)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException", is_deprecated = true))]
    pub fn new_str(s: String) -> Result<Self> {
        panic!("stub: java/lang/Long.<init>:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "byteValue", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Long.byteValue:()B")
    }

    #[cfg_attr(any(), java_method(name = "shortValue", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Long.shortValue:()S")
    }

    #[cfg_attr(any(), java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Long.intValue:()I")
    }

    #[cfg_attr(any(), java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn longValue(&self) -> Result<i64> {
        let this = self;
        Ok(this.value.get())
    }

    #[cfg_attr(any(), java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Long.floatValue:()F")
    }

    #[cfg_attr(any(), java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Long.doubleValue:()D")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Long.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Long.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode_l(value: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.hashCode:(J)I")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Long.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "getLong", descriptor = "(Ljava/lang/String;)Ljava/lang/Long;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getLong_str(nm: String) -> Result<i64> {
        panic!("stub: java/lang/Long.getLong:(Ljava/lang/String;)Ljava/lang/Long;")
    }

    #[cfg_attr(any(), java_method(name = "getLong", descriptor = "(Ljava/lang/String;J)Ljava/lang/Long;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getLong_str_l(nm: String, val: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.getLong:(Ljava/lang/String;J)Ljava/lang/Long;")
    }

    #[cfg_attr(any(), java_method(name = "getLong", descriptor = "(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getLong_str_lng(nm: String, val: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.getLong:(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;")
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/Long;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareTo(&self, anotherLong: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.compareTo:(Ljava/lang/Long;)I")
    }

    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(JJ)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compare(x: i64, arg1: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.compare:(JJ)I")
    }

    #[cfg_attr(any(), java_method(name = "compareUnsigned", descriptor = "(JJ)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareUnsigned(x: i64, arg1: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.compareUnsigned:(JJ)I")
    }

    #[cfg_attr(any(), java_method(name = "divideUnsigned", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn divideUnsigned(dividend: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.divideUnsigned:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "remainderUnsigned", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remainderUnsigned(dividend: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.remainderUnsigned:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "highestOneBit", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn highestOneBit(i: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.highestOneBit:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "lowestOneBit", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lowestOneBit(i: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.lowestOneBit:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "numberOfLeadingZeros", descriptor = "(J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn numberOfLeadingZeros(i: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.numberOfLeadingZeros:(J)I")
    }

    #[cfg_attr(any(), java_method(name = "numberOfTrailingZeros", descriptor = "(J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn numberOfTrailingZeros(i: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.numberOfTrailingZeros:(J)I")
    }

    #[cfg_attr(any(), java_method(name = "bitCount", descriptor = "(J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn bitCount(i: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.bitCount:(J)I")
    }

    #[cfg_attr(any(), java_method(name = "rotateLeft", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn rotateLeft(i: i64, arg1: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.rotateLeft:(JI)J")
    }

    #[cfg_attr(any(), java_method(name = "rotateRight", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn rotateRight(i: i64, arg1: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.rotateRight:(JI)J")
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reverse(i: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.reverse:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "compress", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compress(i: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.compress:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "expand", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn expand(i: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.expand:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "parallelSuffix", descriptor = "(J)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn parallelSuffix(maskCount: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.parallelSuffix:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "signum", descriptor = "(J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn signum(i: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.signum:(J)I")
    }

    #[cfg_attr(any(), java_method(name = "reverseBytes", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reverseBytes(i: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.reverseBytes:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "sum", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn sum(a: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.sum:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn max(a: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.max:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn min(a: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.min:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/Long;>;"))]
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Long.describeConstable:()Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Long;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i64> {
        panic!("stub: java/lang/Long.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Long;")
    }
}
